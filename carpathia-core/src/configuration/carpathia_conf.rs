//! Configuration consists of
//!
//! - CarpathiaConfig
//! - CarpathiaConfigBuilder
//!
//! to avoid parameter explosion.
use super::conf_enums::CacheModus;
use super::conf_enums::{DbPool, DbType};
use super::conf_file_reader::load_type_mappings;
use super::conf_structs::Types;
use crate::return_values::carpathia_errors::{CarpathiaError, ErrorNumber};
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;

/// Structured configuration - to build it use
///   CarpathiaConfigBuilder.
pub struct CarpathiaConfig {
    /// Database pool to connect to, feed with
    ///
    /// - db_user
    /// - db_name
    /// - db_type
    ///
    /// CarpathiaConfigBuilder does it for you.
    pub db_pool: DbPool,
    pub cache_modus: CacheModus,
    pub template_directory: PathBuf,
    #[allow(unfulfilled_lint_expectations)]
    #[allow(dead_code)]
    pub output_directory: PathBuf,
    pub cache_file: PathBuf,
    /// Database types mapped to user types
    pub type_map: Types,
    pub print_schema: bool,
    pub print_db_types: bool,
    pub execute_templates: bool,
}

/// CarpathiaConfigBuilder is close related to all the
/// configuration parameters. E.g. from a CLI.
/// Its only purpose is to create the CarpathiaConfig.
pub struct CarpathiaConfigBuilder {
    db_host: Option<String>,
    db_port: Option<i32>,
    db_user: Option<String>,
    db_password: Option<String>,
    db_name: Option<String>,
    db_type: Option<DbType>,
    cache_modus: CacheModus,
    template_directory: PathBuf,
    output_directory: PathBuf,
    cache_file: PathBuf,
    type_mapping_file: PathBuf,
    print_schema: bool,
    print_db_types: bool,
    execute_templates: bool,
}

impl Default for CarpathiaConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CarpathiaConfigBuilder {
    pub fn new() -> Self {
        Self {
            db_host: None,
            db_port: None,
            db_user: None,
            db_password: None,
            db_name: None,
            db_type: None,
            cache_modus: CacheModus::UseCache,
            template_directory: "tera/rust_lib".into(),
            output_directory: ".".into(),
            cache_file: "./carpathia_cache.json".into(),
            type_mapping_file: "carpathia_type_mapping.json".into(),
            print_schema: false,
            print_db_types: false,
            execute_templates: false,
        }
    }
    pub fn db_host(mut self, host: impl Into<String>) -> Self {
        self.db_host = Some(host.into());
        self
    }
    pub fn db_port(mut self, port: impl Into<i32>) -> Self {
        self.db_port = Some(port.into());
        self
    }

    pub fn db_user(mut self, user: impl Into<String>) -> Self {
        self.db_user = Some(user.into());
        self
    }

    pub fn db_password(mut self, password: impl Into<String>) -> Self {
        self.db_password = Some(password.into());
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

    pub fn template_directory(mut self, dir: impl Into<PathBuf>) -> Self {
        self.template_directory = dir.into();
        self
    }

    pub fn output_directory(mut self, dir: impl Into<PathBuf>) -> Self {
        self.output_directory = dir.into();
        self
    }

    pub fn cache_file(mut self, file: impl Into<PathBuf>) -> Self {
        self.cache_file = file.into();
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

    pub fn execute_templates(mut self, val: bool) -> Self {
        self.execute_templates = val;
        self
    }
}

impl CarpathiaConfigBuilder {
    /// Building CarpathiaConfig.
    pub fn build(self) -> Result<CarpathiaConfig, CarpathiaError> {
        let db_host = self.db_host.ok_or_else(|| CarpathiaError {
            message: "db_host missing".into(),
            error_type: ErrorNumber::InvalidConfiguration,
        })?;
        let db_user = self.db_user.ok_or_else(|| CarpathiaError {
            message: "db_user missing".into(),
            error_type: ErrorNumber::InvalidConfiguration,
        })?;
        let db_password = self.db_password.ok_or_else(|| CarpathiaError {
            message: "db_password missing".into(),
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

        let db_port: i32 = match self.db_port {
            Some(p) => p,
            None => i32::from(db_type),
        };
        let db_pool = match db_type {
            // building an url like string db_type://user_name:user_password@db_host/db_name
            DbType::Postgres => DbPool::Postgres(
                PgPoolOptions::new()
                    .connect_lazy(&format!(
                        "{db_type}://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}"
                    ))
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
            template_directory: self.template_directory,
            output_directory: self.output_directory,
            cache_file: self.cache_file,
            type_map,
            print_schema: self.print_schema,
            print_db_types: self.print_db_types,
            execute_templates: self.execute_templates,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = CarpathiaConfigBuilder::new()
            .db_type(DbType::Postgres)
            .db_host("localhost")
            .db_user("db_user")
            .db_password("db_password")
            .db_name("test_db")
            .db_type(DbType::Dummy)
            .cache_modus(CacheModus::UseCache)
            .output_directory("./output")
            .cache_file("./cache/carpathia_cache.json")
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
        assert!(error.message.contains("missing"));
    }
}
