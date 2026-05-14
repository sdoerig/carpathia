use crate::configuration::conf_enums::DbType;
/// This module extracts the datebase schema from a `PostgreSQL` database and
/// generates a Rust struct for each table in the database. It also proviedes the
/// intermeditate data structures to hold the extracted schema information.
use crate::db::db_schema_structs::AbstractDbRepr;
use crate::db::postgresql::PostgresQuerier;
use crate::db::traits::DatabaseQuerier;
use crate::return_values::carpathia_errors::CarpathiaError;

pub(crate) struct DbSchemaParser {
    // You can add fields here if needed, for example, to hold configuration or state
    db_name: String,
    db_url: String,
    db_type: DbType,
}

impl DbSchemaParser {
    pub(crate) fn new(db_url: String, db_name: String, db_type: DbType) -> Self {
        Self {
            db_name,
            db_url,
            db_type,
        }
    }

    pub(crate) async fn parse_schema(&self) -> Result<AbstractDbRepr, CarpathiaError> {
        match self.db_type {
            DbType::Postgres => {
                let querier = PostgresQuerier::new(&self.db_url, &self.db_name)?;
                querier.get_schema().await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_db_schema_parser() {
        // Lade .env.test (falls vorhanden)
        dotenv::from_filename(".env.test").ok();

        // Verwende Umgebungsvariablen mit Fallback für CI
        let db_url = std::env::var("TEST_DB_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());

        let db_name = std::env::var("TEST_DB_NAME").unwrap_or_else(|_| "postgres".to_string());

        let parser = DbSchemaParser::new(db_url, db_name, DbType::Postgres);
        let schema = parser.parse_schema().await.unwrap();
        assert!(!schema.tables.is_empty(), "Schema should not be empty");
    }
}
