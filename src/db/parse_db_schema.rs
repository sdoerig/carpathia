/// This module extracts the datebase schema from a PostgreSQL database and
/// generates a Rust struct for each table in the database. It also proviedes the
/// intermeditate data structures to hold the extracted schema information.
///
///
use log::{debug, info};
use sqlx::{Pool, Postgres, Row, postgres::PgPoolOptions};
use crate::db::db_schema_structs::ColumnInfo;
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
    table_name,
    column_name,
    data_type,
    is_nullable,
    column_default,
    character_maximum_length,
    numeric_precision,
    numeric_scale,
    is_identity,
    identity_generation,
    is_generated,
    generation_expression
FROM
    information_schema.columns
WHERE
    table_schema = 'public'
ORDER BY
    table_name,
    ordinal_position;
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

    pub(crate) async fn parse_schema(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Here you would implement the logic to query the database for its schema
        // and populate your data structures with the extracted information.
        // This is just a placeholder for demonstration purposes.
        info!("Parsing schema for database: {}", &self.db_name);
        let rows: Vec<ColumnInfo> = sqlx::query_as::<_,ColumnInfo>(SCHEMA_QUERY)
            .fetch_all(&self.pool)
            .await
            .expect("Failed to execute schema query");
        for row in rows {
            

            debug!(
                "Table: {}, Column: {}, Data Type: {}, Is Nullable: {}, Column Default: {:?}, Character Maximum Length: {:?}, Numeric Precision: {:?}, Numeric Scale: {:?}, Is Identity: {}, Identity Generation: {:?}, Is Generated: {}, Generation Expression: {:?}",
                &row.table_name,
                &row.column_name,
                &row.data_type,
                &row.is_nullable,
                &row.column_default,
                &row.character_maximum_length,
                &row.numeric_precision,
                &row.numeric_scale,
                &row.is_identity,
                &row.identity_generation,
                &row.is_generated,
                &row.generation_expression
            );
        }
        Ok(())
    }
}
