use super::conf_enums::CacheModus;
use super::conf_enums::{DbPool, DbType};
use crate::return_values::carpathia_errors::{CarpathiaError, ErrorNumber};
use log::error;
use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;
pub struct CarpathiaConfig {
    pub db_pool: DbPool,
    pub cache_modus: CacheModus,
    pub output_directory: PathBuf,
    pub cache_directory: PathBuf,
    pub print_schema: bool,
    pub print_db_types: bool,
}

impl CarpathiaConfig {
    pub fn new(
        db_url: &String,
        db_name: &String,
        db_type: &DbType,
        cache_modus: CacheModus,
        output_directory: &String,
        cache_directory: &String,
        print_schema: bool,
        print_db_types: bool,
    ) -> Result<Self, CarpathiaError> {
        let db_pool = match db_type {
            DbType::Postgres => DbPool::Postgres(PgPoolOptions::new()
            .connect_lazy(&format!("{db_url}/{db_name}"))
            .map_err(|e| {
                error!("Error creating database connection pool: {e}");
                CarpathiaError {
                    message: format!("Failed to create database connection pool: {e}"),
                    error_type: crate::return_values::carpathia_errors::ErrorNumber::DatabaseConnectionError,
                }
            })?),
            // Future support for MySQL and SQLite can be added here by adding new variants to the DbPool enum and handling them accordingly.
        };

        Ok(CarpathiaConfig {
            db_pool,
            cache_modus,
            output_directory: PathBuf::from(output_directory),
            cache_directory: PathBuf::from(cache_directory),
            print_schema,
            print_db_types,
        })
    }
}
