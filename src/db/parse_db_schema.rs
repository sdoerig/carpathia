use crate::configuration::carpathia_conf::CarpathiaConfig;
use crate::configuration::conf_enums::{DbPool, DbType};
/// This module extracts the datebase schema from a `PostgreSQL` database and
/// generates a Rust struct for each table in the database. It also proviedes the
/// intermeditate data structures to hold the extracted schema information.
use crate::db::db_schema_structs::AbstractDbRepr;
use crate::db::postgresql::PostgresQuerier;
use crate::db::traits::DatabaseQuerier;
use crate::return_values::carpathia_errors::CarpathiaError;

pub(crate) struct DbSchemaParser {
    // You can add fields here if needed, for example, to hold configuration or state
}

impl DbSchemaParser {
    pub(crate) async fn parse_schema(
        config: &CarpathiaConfig,
    ) -> Result<AbstractDbRepr, CarpathiaError> {
        match config.db_pool {
            DbPool::Postgres(_) => PostgresQuerier::get_schema(config).await,
            DbPool::Dummy => todo!("Dummy database pool not implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_config() -> CarpathiaConfig {
        // Lade .env.test (falls vorhanden)
        dotenv::from_filename(".env.test").ok();

        // Verwende Umgebungsvariablen mit Fallback für CI
        let db_url = std::env::var("TEST_DB_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());

        let db_name = std::env::var("TEST_DB_NAME").unwrap_or_else(|_| "postgres".to_string());

        CarpathiaConfig::new(
            &db_url,
            &db_name,
            &DbType::Postgres,
            crate::configuration::conf_enums::CacheModus::BypassCache,
            &"./output".to_string(),
            &"./cache".to_string(),
            false,
            false,
        )
        .expect("Failed to create test configuration")
    }

    #[tokio::test]
    async fn test_db_schema_parser() {
        // Lade .env.test (falls vorhanden)
        dotenv::from_filename(".env.test").ok();

        let config = setup_test_config();
        let schema = DbSchemaParser::parse_schema(&config).await.unwrap();
        assert!(!schema.tables.is_empty(), "Schema should not be empty");
    }
}
