use std::collections::HashMap;

use crate::db::db_schema_structs::ColumnInfo;
/// This module extracts the datebase schema from a PostgreSQL database and
/// generates a Rust struct for each table in the database. It also proviedes the
/// intermeditate data structures to hold the extracted schema information.
///
///
use log::{debug, info};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
// scratch code for testing the database connection and schema extraction
// pk
//SELECT table_name, column_name, data_type, is_nullable
//FROM information_schema.columns
//WHERE table_schema = 'public'
//ORDER BY table_name, ordinal_position;
//
// fk
//SELECT kcu.table_name, kcu.column_name
//FROM information_schema.table_constraints tco
//JOIN information_schema.key_column_usage kcu ON kcu.constraint_name = tco.constraint_name
//WHERE tco.constraint_type = 'PRIMARY KEY' AND tco.table_schema = 'public';

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

pub(crate) struct DbSchemaParser {
    // You can add fields here if needed, for example, to hold configuration or state
    db_name: String,
    pool: Pool<Postgres>,
}

impl DbSchemaParser {
    pub(crate) async fn new(db_url: String, db_name: String) -> Self {
        let full_db_url = format!("{}/{}", db_url, db_name);
        info!("Connecting to database at URL: {}", &full_db_url);
        let pool = PgPoolOptions::new()
            .connect(&full_db_url)
            .await
            .expect("Failed to connect to the database");
        Self { db_name, pool }
    }

    pub(crate) async fn parse_schema(
        &self,
    ) -> Result<HashMap<String, Vec<ColumnInfo>>, Box<dyn std::error::Error>> {
        // Here you would implement the logic to query the database for its schema
        // and populate your data structures with the extracted information.
        // This is just a placeholder for demonstration purposes.
        info!("Parsing schema for database: {}", &self.db_name);
        let mut table_info_map: std::collections::HashMap<String, Vec<ColumnInfo>> =
            std::collections::HashMap::new();
        let rows: Vec<ColumnInfo> = sqlx::query_as::<_, ColumnInfo>(SCHEMA_QUERY)
            .fetch_all(&self.pool)
            .await
            .expect("Failed to execute schema query");
        for row in rows {
            debug!(
                "Processing column: {}.{}",
                &row.table_name, &row.column_name
            );
            table_info_map
                .entry(row.table_name.clone())
                .or_default()
                .push(row);
        }

        Ok(table_info_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_db_schema_parser() {
        // This is a placeholder test. You would need to set up a test database and populate it with test data to make this meaningful.
        let db_url = "postgres://doerig:doerig@127.0.2.15:5432".to_string();
        let db_name = "carpathia".to_string();
        let parser = DbSchemaParser::new(db_url, db_name).await;
        let schema = parser.parse_schema().await.unwrap();
        // Add assertions here based on your test database schema
        assert!(!schema.is_empty(), "Schema should not be empty");
    }
}
