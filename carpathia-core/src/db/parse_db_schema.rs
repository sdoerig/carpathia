/// This module extracts the datebase schema from a `PostgreSQL` database and
/// generates a Rust struct for each table in the database. It also proviedes the
/// intermeditate data structures to hold the extracted schema information.
use crate::configuration::carpathia_conf::CarpathiaConfig;
use crate::configuration::conf_enums::DbPool;
use crate::db::db_schema_structs::AbstractDbRepr;
use crate::db::postgresql::PostgresQuerier;
use crate::db::traits::DatabaseQuerier;
use crate::return_values::carpathia_errors::CarpathiaError;

pub struct DbSchemaParser {
    // You can add fields here if needed, for example, to hold configuration or state
}

impl DbSchemaParser {
    pub async fn parse_schema(config: &CarpathiaConfig) -> Result<AbstractDbRepr, CarpathiaError> {
        match config.db_pool {
            DbPool::Postgres(_) => PostgresQuerier::get_schema(config).await,
            DbPool::Dummy => todo!("Dummy database pool not implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::carpathia_conf::CarpathiaConfigBuilder;
    use crate::configuration::conf_enums::DbType;

    fn setup_test_config() -> CarpathiaConfig {
        // Load .env.test (if available)
        dotenv::from_filename(".env.test").ok();

        let db_type = match std::env::var("TEST_DB_TYPE") {
            Ok(s) => s.parse::<DbType>().unwrap_or(DbType::Postgres),
            Err(_) => DbType::Postgres,
        };
        let db_host = std::env::var("TEST_DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let db_port = match std::env::var("TEST_DB_PORT") {
            Ok(s) => s.parse::<i32>().unwrap_or(5432),
            Err(_) => 5432,
        };
        let db_user = std::env::var("TEST_DB_USER").unwrap_or_else(|_| "postgres".to_string());
        let db_password =
            std::env::var("TEST_DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string());

        let db_name = std::env::var("TEST_DB_NAME").unwrap_or_else(|_| "postgres".to_string());

        CarpathiaConfigBuilder::new()
            .db_type(db_type)
            .db_host(db_host)
            .db_port(db_port)
            .db_user(db_user)
            .db_password(db_password)
            .db_name(&db_name)
            .db_type(DbType::Postgres)
            .cache_modus(crate::configuration::conf_enums::CacheModus::BypassCache)
            .carpathia_type_mapping("carpathia_type_mapping.json".to_string())
            .output_directory("./output".to_string())
            .cache_file("./cache/carpathia_cache.json".to_string())
            .print_schema(false)
            .print_db_types(false)
            .build()
            .expect("Config building failed...")
    }

    #[tokio::test]
    async fn test_db_schema_parser() {
        // load .env.test if available.
        dotenv::from_filename(".env.test").ok();

        let config = setup_test_config();
        let schema = DbSchemaParser::parse_schema(&config).await.unwrap();
        assert!(!schema.tables.is_empty(), "Schema should not be empty");
    }
}
