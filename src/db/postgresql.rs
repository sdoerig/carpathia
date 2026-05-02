use super::db_schema_structs::{AbstractAttribute, AbstractDbRepr};
use super::traits::DatabaseQuerier;
use log::{debug, error, info};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub(crate) struct PostgresQuerier {
    pool: Pool<Postgres>,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
struct PgColumnInfo {
    pub table_name: String,
    pub column_name: String,
    pub data_type: String,
    pub is_nullable: String,
    pub column_default: Option<String>,
    pub character_maximum_length: Option<i32>,
    pub numeric_precision: Option<i32>,
    pub numeric_scale: Option<i32>,
    pub is_identity: String,
    pub identity_generation: Option<String>,
    pub is_generated: String,
    pub generation_expression: Option<String>,
    pub constraint_name: Option<String>,
    pub constraint_type: Option<String>,
    pub referenced_table: Option<String>,
    pub referenced_column: Option<String>,
}

const SCHEMA_QUERY: &str = r#"
SELECT
    -- Column Metadata
    c.table_name as table_name,
    c.column_name as column_name,
    c.data_type as data_type,
    c.is_nullable as is_nullable,
    c.column_default as column_default,
    c.character_maximum_length as character_maximum_length,
    c.numeric_precision as numeric_precision,
    c.numeric_scale as numeric_scale,
    c.is_identity as is_identity,
    c.identity_generation as identity_generation,
    c.is_generated as is_generated,
    c.generation_expression as generation_expression,
    -- Constraint Details
    tc.constraint_name as constraint_name,
    tc.constraint_type as constraint_type,
    -- Foreign Key Targets
    ccu.table_name AS referenced_table,
    ccu.column_name AS referenced_column
FROM 
    information_schema.columns c
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
WHERE 
    c.table_schema = 'public'
ORDER BY 
    c.table_name, 
    c.ordinal_position;

"#;

impl PostgresQuerier {
    pub(crate) fn new(db_url: &str, db_name: &str) -> Self {
        let full_db_url = format!("{}/{}", db_url, db_name);
        let pool = PgPoolOptions::new()
            .connect_lazy(&full_db_url)
            .expect("Failed to create database connection pool");
        Self { pool }
    }
}

impl DatabaseQuerier for PostgresQuerier {
    async fn get_schema(
        &self,
    ) -> Result<std::collections::HashMap<String, AbstractDbRepr>, Box<dyn std::error::Error>> {
        // Here you would implement the logic to query the database for its schema
        // and populate your data structures with the extracted information.
        // This is just a placeholder for demonstration purposes.
        info!(
            "Parsing schema for PostgreSQL database: {}",
            &self
                .pool
                .connect_options()
                .get_database()
                .unwrap_or(&"unknown".to_string())
        );
        let mut table_info_map: std::collections::HashMap<String, AbstractDbRepr> =
            std::collections::HashMap::new();
        let rows: Vec<PgColumnInfo> = sqlx::query_as::<_, PgColumnInfo>(SCHEMA_QUERY)
            .fetch_all(&self.pool)
            .await
            .expect("Failed to execute schema query");
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
            };
            table_info_map
                .entry(row.table_name.clone())
                .or_insert_with(|| AbstractDbRepr {
                    table_name: row.table_name,
                    attributes: Vec::new(),
                })
                .attributes
                .push(attribute);
        }

        Ok(table_info_map)
    }
}
