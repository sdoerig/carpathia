use std::collections::BTreeMap;
use crate::db::db_schema_structs::DbType;
use crate::db::db_schema_structs::AbstractDbRepr;
/// This module extracts the datebase schema from a PostgreSQL database and
/// generates a Rust struct for each table in the database. It also proviedes the
/// intermeditate data structures to hold the extracted schema information.
///
///
use log::{debug, info};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use crate::db::traits::DatabaseQuerier;

use crate::db::postgresql::PostgresQuerier;


pub(crate) struct DbSchemaParser {
    // You can add fields here if needed, for example, to hold configuration or state
    db_name: String,
    db_url: String,
    db_type: DbType
    
}

impl DbSchemaParser {
    pub(crate)  fn new(db_url: String, db_name: String, db_type: DbType) -> Self {
       Self { db_name, db_url, db_type }
    }

    pub(crate) async fn parse_schema(
        &self,
    ) -> Result<BTreeMap<String, AbstractDbRepr>, Box<dyn std::error::Error>> {
        
        match self.db_type {
            DbType::Postgres => {
                let querier = PostgresQuerier::new(&self.db_url, &self.db_name);
                querier.get_schema().await
            },
            DbType::MySql => {
                unimplemented!("MySQL support is not implemented yet");
            },
            DbType::Sqlite => {
                unimplemented!("SQLite support is not implemented yet");
            }
        }
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
        let parser = DbSchemaParser::new(db_url, db_name, DbType::Postgres);
        let schema = parser.parse_schema().await.unwrap();
        // Add assertions here based on your test database schema
        assert!(!schema.is_empty(), "Schema should not be empty");
    }
}
