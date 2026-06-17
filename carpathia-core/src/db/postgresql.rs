/// PostgreSQL schema querieer. Currently implemented
/// - Basic tables
/// - Views
/// - Materialized Views
use super::db_schema_structs::{
    ABSTRACT_DB_REPR_VERSION, AbstractAttribute, AbstractDbRepr, AbstractTableRepr, ConstraintType,
    IsGenerated, IsIdentity, IsNullable, ObjectType,
};
use super::traits::DatabaseQuerier;
use crate::configuration::carpathia_conf::CarpathiaConfig;
use crate::configuration::conf_enums::DbPool;
use crate::db::postgresql_structs::PgColumnInfo;
use crate::return_values::carpathia_errors::CarpathiaError;
use log::{debug, error, info};
use std::collections::{BTreeMap, BTreeSet};
pub(crate) struct PostgresQuerier {}

const LIMIT: i64 = 1000;
const SCHEMA_QUERY: &str = r"
WITH cols AS (
    SELECT
        n.nspname AS table_schema,
        c.relname AS table_name,
        a.attname AS column_name,
        format_type(a.atttypid, a.atttypmod) AS data_type,
        a.attndims::int4 AS array_dimensions,
        NOT a.attnotnull AS is_nullable,
        pg_get_expr(ad.adbin, ad.adrelid) AS column_default,
        a.attnum,
        c.oid AS table_oid,
        a.attrelid AS attrelid,
        a.attidentity::text AS identity_generation,
        a.attgenerated
    FROM pg_class c
    JOIN pg_namespace n ON n.oid = c.relnamespace
    JOIN pg_attribute a 
        ON a.attrelid = c.oid 
       AND a.attnum > 0 
       AND NOT a.attisdropped
    LEFT JOIN pg_attrdef ad 
        ON ad.adrelid = c.oid 
       AND ad.adnum = a.attnum
    WHERE n.nspname = 'public'
      AND c.relkind IN ('r','v')  -- tables + views
),

pk_constraints AS (
    SELECT
        con.conname AS constraint_name,
        con.conrelid AS table_oid,
        unnest(con.conkey) AS column_attnum
    FROM pg_constraint con
    WHERE con.contype = 'p'
),

fk_constraints AS (
    SELECT
        con.conname AS constraint_name,
        con.conrelid AS table_oid,
        con.confrelid AS referenced_table_oid,
        unnest(con.conkey) AS column_attnum,
        unnest(con.confkey) AS referenced_attnum
    FROM pg_constraint con
    WHERE con.contype = 'f'
)

SELECT
    CASE c.relkind
        WHEN 'r' THEN 'BASE TABLE'
        WHEN 'v' THEN 'VIEW'
    END AS object_type,
    col.table_name,
    col.column_name,
    col.data_type,
    col.array_dimensions,
    CASE WHEN col.is_nullable THEN 'YES' ELSE 'NO' END AS is_nullable,
    col.column_default,
    CASE WHEN c.relkind = 'r' THEN 'YES' ELSE 'NO' END AS table_is_insertable,
    CASE WHEN c.relkind = 'r' THEN 'YES' ELSE 'NO' END AS column_is_updatable,
    NULL AS character_maximum_length,
    NULL AS numeric_precision,
    NULL AS numeric_scale,

    CASE WHEN col.identity_generation <> '' THEN 'YES' ELSE 'NO' END AS is_identity,
    col.identity_generation,

    CASE WHEN col.attgenerated <> '' THEN 'ALWAYS' ELSE 'NEVER' END AS is_generated,
    CASE 
        WHEN col.attgenerated <> '' THEN pg_get_expr(ad.adbin, ad.adrelid)
        ELSE NULL
    END AS generation_expression,

    -- unified constraint_name
    COALESCE(pk.constraint_name, fk.constraint_name) AS constraint_name,

    CASE
        WHEN pk.constraint_name IS NOT NULL THEN 'Primary Key'
        WHEN fk.constraint_name IS NOT NULL THEN 'Foreign Key'
        ELSE NULL
    END AS constraint_type,

    rt.relname AS referenced_table,
    ra.attname AS referenced_column,

    obj_description(col.table_oid) AS table_comment,
    col_description(col.attrelid, col.attnum) AS column_comment

FROM cols col
JOIN pg_class c ON c.oid = col.table_oid

LEFT JOIN pg_attrdef ad 
    ON ad.adrelid = col.attrelid 
   AND ad.adnum = col.attnum

LEFT JOIN pk_constraints pk
    ON pk.table_oid = col.table_oid
   AND pk.column_attnum = col.attnum

LEFT JOIN fk_constraints fk
    ON fk.table_oid = col.table_oid
   AND fk.column_attnum = col.attnum

LEFT JOIN pg_class rt ON rt.oid = fk.referenced_table_oid
LEFT JOIN pg_attribute ra
    ON ra.attrelid = fk.referenced_table_oid
   AND ra.attnum = fk.referenced_attnum

UNION ALL

SELECT
    'MATERIALIZED VIEW' as object_type,
    mat.matviewname as table_name,
    a.attname as column_name,
    format_type(a.atttypid, a.atttypmod) as data_type,
    a.attndims::int4 AS array_dimensions,
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
    NULL AS constraint_name,
    NULL AS constraint_type,
    NULL AS referenced_table,
    NULL AS referenced_column,
    obj_description((quote_ident(mat.schemaname) || '.' || quote_ident(mat.matviewname))::regclass) AS table_comment,
    col_description(a.attrelid, a.attnum) AS column_comment
FROM pg_matviews mat
JOIN pg_attribute a 
    ON a.attrelid = (quote_ident(mat.schemaname) || '.' || quote_ident(mat.matviewname))::regclass
WHERE mat.schemaname = 'public'
  AND a.attnum > 0
  AND NOT a.attisdropped

ORDER BY table_name, column_name
LIMIT $1
OFFSET $2;
    ";

impl PostgresQuerier {}

impl DatabaseQuerier for PostgresQuerier {
    async fn get_schema(config: &CarpathiaConfig) -> Result<AbstractDbRepr, CarpathiaError> {
        // Here you would implement the logic to query the database for its schema
        // and populate your data structures with the extracted information.
        // This is just a placeholder for demonstration purposes.
        info!("Parsing schema for PostgreSQL database:");
        let mut table_info_map: std::collections::BTreeMap<String, AbstractTableRepr> =
            std::collections::BTreeMap::new();
        let mut view_info_map: std::collections::BTreeMap<String, AbstractTableRepr> =
            std::collections::BTreeMap::new();
        let mut offset = 0;
        let pool = match config.db_pool {
            DbPool::Postgres(ref pool) => pool,
            _ => {
                return Err(CarpathiaError {
                    message: "Invalid database pool type for PostgreSQL querier".to_string(),
                    error_type:
                        crate::return_values::carpathia_errors::ErrorNumber::InvalidPoolType,
                });
            }
        };
        //// let type_map = &config.type_map.type_mapping;
        loop {
            let rows: Vec<PgColumnInfo> = sqlx::query_as::<_, PgColumnInfo>(SCHEMA_QUERY)
                .bind(LIMIT)
                .bind(offset)
                .fetch_all(pool)
                .await
                .map_err(|e| {
                    debug!("Error executing schema query: {e}");
                    CarpathiaError {
                        message: format!("Failed to execute schema query: {e}"),
                        error_type: crate::return_values::carpathia_errors::ErrorNumber::DatabaseConnectionError,
                    }
                })?;
            let num_rows = rows.len();
            debug!("Fetched {num_rows} rows from schema query with offset {offset}");
            for row in rows {
                debug!(
                    "Processing column: {}.{}",
                    &row.table_name, &row.column_name
                );
                let data_type = if let Some(dimensions) = row.array_dimensions {
                    if dimensions != 0 {
                        format!("{}[{}]", &row.data_type, dimensions)
                    } else {
                        row.data_type.clone()
                    }
                } else {
                    row.data_type.clone()
                };
                // map the user type to the ADR
                ////let u_type_map = match type_map.get(&row.data_type) {
                ////    Some(t) => t,
                ////    None => NONE_TYPE_MAPPING,
                ////};

                let attribute = AbstractAttribute {
                    column_name: row.column_name,
                    data_type,
                    u_type: String::new(), // Placeholder, will be filled in by enrich_adr
                    is_nullable: row
                        .is_nullable
                        .parse()
                        .unwrap_or(IsNullable::Unknown(row.is_nullable)),
                    column_default: row.column_default,
                    character_maximum_length: row.character_maximum_length,
                    numeric_precision: row.numeric_precision,
                    numeric_scale: row.numeric_scale,
                    is_identity: row
                        .is_identity
                        .parse()
                        .unwrap_or(IsIdentity::Unknown(row.is_identity)),
                    identity_generation: row.identity_generation,
                    is_generated: row
                        .is_generated
                        .parse()
                        .unwrap_or(IsGenerated::Unknown(row.is_generated)),
                    generation_expression: row.generation_expression,
                    constraint_name: row.constraint_name,
                    constraint_type: row
                        .constraint_type
                        .as_ref()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(ConstraintType::None),
                    referenced_table: row.referenced_table,
                    referenced_column: row.referenced_column,
                    comment: row.column_comment,
                };
                let object_type = row.object_type.parse().unwrap_or_else(|_| {
                    debug!("Unknown object type: {}", row.object_type);
                    ObjectType::Other
                });
                match object_type {
                    ObjectType::BaseTable => {
                        table_info_map
                            .entry(row.table_name.clone())
                            .or_insert_with(|| AbstractTableRepr {
                                table_name: row.table_name.clone(),
                                u_imports: BTreeSet::new(),
                                object_type,
                                comment: row.table_comment,
                                attributes: BTreeMap::new(),
                            })
                            .attributes
                            .insert(attribute.column_name.clone(), attribute);
                        //insert_u_import(&mut table_info_map, &row.table_name, u_type_map);
                    }
                    ObjectType::View | ObjectType::MaterializedView => {
                        view_info_map
                            .entry(row.table_name.clone())
                            .or_insert_with(|| AbstractTableRepr {
                                table_name: row.table_name.clone(),
                                u_imports: BTreeSet::new(),
                                object_type,
                                comment: row.table_comment,
                                attributes: BTreeMap::new(),
                            })
                            .attributes
                            .insert(attribute.column_name.clone(), attribute);
                    }
                    _ => {
                        error!(
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
            version: ABSTRACT_DB_REPR_VERSION.to_string(),
            tables: table_info_map,
            views: view_info_map,
        })
    }
}
