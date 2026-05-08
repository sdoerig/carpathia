use super::db_schema_structs::{AbstractAttribute, AbstractDbRepr, AbstractTableRepr};
use super::traits::DatabaseQuerier;
use log::{debug, info};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use crate::db::postgresql_structs::PgColumnInfo;
pub(crate) struct PostgresQuerier {
    pool: Pool<Postgres>,
}

const LIMIT: i64 = 30;

const SCHEMA_QUERY: &str = r"
SELECT
    t.table_type AS object_type,
    c.table_name,
    c.column_name,
    CASE
        WHEN c.data_type = 'USER-DEFINED' THEN c.udt_name
        ELSE c.data_type
    END as data_type,
    c.is_nullable,
    c.column_default,
    t.is_insertable_into AS table_is_insertable, 
    c.is_updatable AS column_is_updatable,        
    c.character_maximum_length,
    c.numeric_precision,
    c.numeric_scale,
    c.is_identity,
    c.identity_generation,
    c.is_generated,
    c.generation_expression,
    tc.constraint_name,
    tc.constraint_type,
    ccu.table_name AS referenced_table,
    ccu.column_name AS referenced_column,
    obj_description(pg_class.oid) AS table_comment,
    col_description(pg_attribute.attrelid, pg_attribute.attnum) AS column_comment
FROM
    information_schema.columns c
JOIN
    information_schema.tables t
    ON c.table_name = t.table_name
    AND c.table_schema = t.table_schema
LEFT JOIN
    information_schema.key_column_usage kcu
    ON c.table_name = kcu.table_name
    AND c.column_name = kcu.column_name
    AND c.table_schema = kcu.table_schema
LEFT JOIN
    information_schema.table_constraints tc
    ON kcu.constraint_name = tc.constraint_name
    AND kcu.table_schema = tc.table_schema
LEFT JOIN
    information_schema.constraint_column_usage ccu
    ON tc.constraint_name = ccu.constraint_name
    AND tc.table_schema = ccu.table_schema
    AND tc.constraint_type = 'FOREIGN KEY'
LEFT JOIN
    pg_class ON pg_class.relname = c.table_name AND pg_class.relnamespace = (SELECT oid FROM pg_namespace WHERE nspname = c.table_schema)
LEFT JOIN
    pg_attribute ON pg_attribute.attrelid = pg_class.oid AND pg_attribute.attname = c.column_name
WHERE
    c.table_schema = 'public'

UNION ALL

-- Materialized Views (immer Read-Only)
SELECT
    'MATERIALIZED VIEW' as object_type,
    mat.matviewname as table_name,
    a.attname as column_name,
    format_type(a.atttypid, a.atttypmod) as data_type,
    CASE WHEN a.attnotnull THEN 'NO' ELSE 'YES' END as is_nullable,
    NULL as column_default,
    'NO' as table_is_insertable,  
    'NO' as column_is_updatable,   
    NULL as character_maximum_length,
    NULL as numeric_precision,
    NULL as numeric_scale,
    'NO' as is_identity,
    NULL as identity_generation,
    'NEVER' as is_generated,
    NULL as generation_expression,
    NULL as constraint_name,
    NULL as constraint_type,
    NULL as referenced_table,
    NULL as referenced_column,
    obj_description((quote_ident(mat.schemaname) || '.' || quote_ident(mat.matviewname))::regclass) AS table_comment,
    col_description(a.attrelid, a.attnum) AS column_comment
FROM
    pg_matviews mat
JOIN
    pg_attribute a ON a.attrelid = (quote_ident(mat.schemaname) || '.' || quote_ident(mat.matviewname))::regclass
WHERE
    mat.schemaname = 'public'
    AND a.attnum > 0
    AND NOT a.attisdropped
ORDER BY
    table_name,
    column_name
LIMIT $1
OFFSET $2;
    ";

impl PostgresQuerier {
    pub(crate) fn new(db_url: &str, db_name: &str) -> Self {
        let full_db_url = format!("{db_url}/{db_name}");
        let pool = PgPoolOptions::new()
            .connect_lazy(&full_db_url)
            .expect("Failed to create database connection pool");
        Self { pool }
    }
}

impl DatabaseQuerier for PostgresQuerier {
    async fn get_schema(&self) -> Result<AbstractDbRepr, Box<dyn std::error::Error>> {
        // Here you would implement the logic to query the database for its schema
        // and populate your data structures with the extracted information.
        // This is just a placeholder for demonstration purposes.
        info!(
            "Parsing schema for PostgreSQL database: {}",
            &self
                .pool
                .connect_options()
                .get_database()
                .unwrap_or("unknown")
        );
        let mut table_info_map: std::collections::BTreeMap<String, AbstractTableRepr> =
            std::collections::BTreeMap::new();
        let mut view_info_map: std::collections::BTreeMap<String, AbstractTableRepr> =
            std::collections::BTreeMap::new();
        let mut offset = 0;
        loop {
            let rows: Vec<PgColumnInfo> = sqlx::query_as::<_, PgColumnInfo>(SCHEMA_QUERY)
                .bind(LIMIT)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .expect("Failed to execute schema query");
            let num_rows = rows.len();
            debug!("Fetched {} rows from schema query with offset {}", num_rows, offset);
            for row in rows {
                debug!(
                    "Processing column: {}.{}",
                    &row.table_name, &row.column_name
                );
                let attribute = AbstractAttribute {
                    column_name: row.column_name,
                    data_type: row.data_type,
                    is_nullable: row.is_nullable,
                    column_default: row.column_default,
                    character_maximum_length: row.character_maximum_length,
                    numeric_precision: row.numeric_precision,
                    numeric_scale: row.numeric_scale,
                    is_identity: row.is_identity,
                    identity_generation: row.identity_generation,
                    is_generated: row.is_generated,
                    generation_expression: row.generation_expression,
                    constraint_name: row.constraint_name,
                    constraint_type: row.constraint_type,
                    referenced_table: row.referenced_table,
                    referenced_column: row.referenced_column,
                    comment: row.column_comment,
                };
                match row.object_type.as_str() {
                    "BASE TABLE" => {
                        table_info_map
                            .entry(row.table_name.clone())
                            .or_insert_with(|| AbstractTableRepr {
                                table_name: row.table_name,
                                object_type: row.object_type,
                                comment: row.table_comment,
                                attributes: Vec::new(),
                            })
                            .unique_push(attribute);
                    }
                    "VIEW" | "MATERIALIZED VIEW" => {
                        view_info_map
                            .entry(row.table_name.clone())
                            .or_insert_with(|| AbstractTableRepr {
                                table_name: row.table_name,
                                object_type: row.object_type,
                                comment: row.table_comment,
                                attributes: Vec::new(),
                            })
                            .unique_push(attribute);
                    }
                    _ => {
                        debug!(
                            "Skipping unsupported object type: {} for table {}",
                            row.object_type, row.table_name
                        );
                    }
                }
            }
            offset += LIMIT;
            if num_rows < LIMIT as usize {
                break;
            }
        }
  
        

        Ok(AbstractDbRepr {
            tables: table_info_map,
            views: view_info_map,
        })
    }
}
