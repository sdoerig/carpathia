use super::conf_enums::CacheModus;
use super::conf_enums::{DbPool, DbType};
use super::conf_file_reader::load_type_mappings;
use super::conf_structs::Types;
use crate::return_values::carpathia_errors::{CarpathiaError, ErrorNumber};
use log::{error, info};
use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;

const CACHE_FILE_NAME: &str = "carpathia_cache.json";

pub struct CarpathiaConfig {
    pub db_pool: DbPool,
    pub cache_modus: CacheModus,
    pub output_directory: PathBuf,
    pub cache_file: PathBuf,
    pub type_map: Types,
    pub print_schema: bool,
    pub print_db_types: bool,
}

pub struct CarpathiaConfigBuilder {
    db_url: Option<String>,
    db_name: Option<String>,
    db_type: Option<DbType>,
    cache_modus: CacheModus,
    output_directory: PathBuf,
    cache_directory: PathBuf,
    type_mapping_file: PathBuf,
    print_schema: bool,
    print_db_types: bool,
}

impl CarpathiaConfigBuilder {
    pub fn new() -> Self {
        Self {
            db_url: None,
            db_name: None,
            db_type: None,
            cache_modus: CacheModus::UseCache,
            output_directory: ".".into(),
            cache_directory: ".".into(),
            type_mapping_file: "carpathia_type_mapping.json".into(),
            print_schema: false,
            print_db_types: false,
        }
    }

    pub fn db_url(mut self, url: impl Into<String>) -> Self {
        self.db_url = Some(url.into());
        self
    }

    pub fn db_name(mut self, name: impl Into<String>) -> Self {
        self.db_name = Some(name.into());
        self
    }

    pub fn db_type(mut self, db_type: DbType) -> Self {
        self.db_type = Some(db_type);
        self
    }

    pub fn cache_modus(mut self, modus: CacheModus) -> Self {
        self.cache_modus = modus;
        self
    }

    pub fn output_directory(mut self, dir: impl Into<PathBuf>) -> Self {
        self.output_directory = dir.into();
        self
    }

    pub fn cache_directory(mut self, dir: impl Into<PathBuf>) -> Self {
        self.cache_directory = dir.into();
        self
    }

    pub fn carpathia_type_mapping(mut self, file: impl Into<PathBuf>) -> Self {
        self.type_mapping_file = file.into();
        self
    }

    pub fn print_schema(mut self, val: bool) -> Self {
        self.print_schema = val;
        self
    }

    pub fn print_db_types(mut self, val: bool) -> Self {
        self.print_db_types = val;
        self
    }
}

impl CarpathiaConfigBuilder {
    pub fn build(self) -> Result<CarpathiaConfig, CarpathiaError> {
        let db_url = self.db_url.ok_or_else(|| CarpathiaError {
            message: "db_url missing".into(),
            error_type: ErrorNumber::InvalidConfiguration,
        })?;
        let db_name = self.db_name.ok_or_else(|| CarpathiaError {
            message: "db_name missing".into(),
            error_type: ErrorNumber::InvalidConfiguration,
        })?;
        let db_type = self.db_type.ok_or_else(|| CarpathiaError {
            message: "db_type missing".into(),
            error_type: ErrorNumber::InvalidConfiguration,
        })?;

        let type_map = match load_type_mappings(&self.type_mapping_file) {
            Ok(types) => {
                info!(
                    "Successfully loaded file {:?} with types {:?}",
                    self.type_mapping_file.as_os_str(),
                    types
                );
                types
            }
            Err(_) => {
                info!("Could not load {:?}", self.type_mapping_file.as_os_str());
                Types::new()
            }
        };

        let db_pool = match db_type {
            DbType::Postgres => DbPool::Postgres(
                PgPoolOptions::new()
                    .connect_lazy(&format!("{db_url}/{db_name}"))
                    .map_err(|e| CarpathiaError {
                        message: format!("DB error: {e}"),
                        error_type: ErrorNumber::DatabaseConnectionError,
                    })?,
            ),
            DbType::Dummy => DbPool::Dummy,
        };

        Ok(CarpathiaConfig {
            db_pool,
            cache_modus: self.cache_modus,
            output_directory: self.output_directory,
            cache_file: self.cache_directory.join("carpathia_cache.json"),
            type_map: type_map,
            print_schema: self.print_schema,
            print_db_types: self.print_db_types,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = CarpathiaConfigBuilder::new()
            .db_url("postgres://localhost")
            .db_name("test_db")
            .db_type(DbType::Dummy)
            .cache_modus(CacheModus::UseCache)
            .output_directory("./output")
            .cache_directory("./cache")
            .print_schema(true)
            .print_db_types(true)
            .build()
            .expect("Failed to build CarpathiaConfig");

        //assert_eq!(config.db_pool, DbPool::Dummy);
        assert_eq!(config.cache_modus, CacheModus::UseCache);
        assert_eq!(config.output_directory, PathBuf::from("./output"));
        assert_eq!(
            config.cache_file,
            PathBuf::from("./cache").join("carpathia_cache.json")
        );
        assert!(config.print_schema);
        assert!(config.print_db_types);
    }

    #[test]
    fn test_config_builder_missing_fields() {
        let result = CarpathiaConfigBuilder::new()
            .db_name("test_db")
            .db_type(DbType::Postgres)
            .build();

        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.error_type, ErrorNumber::InvalidConfiguration);
        assert_eq!(error.message, "db_url missing");
    }
}
