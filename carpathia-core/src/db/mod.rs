//! db
//!
//! This module handles the database introspection. Its main function is
//! - (parse_schema)[crate::db::parse_db_schema::DbSchemaParser::parse_schema]
//!   which delivers the abstract database representation (ADR)[crate::db::db_schema_structs::AbstractDbRepr], The database connection
//!   is passed via (configuration)[crate::configuration::carpathia_conf::CarpathiaConfig] struct
//!

pub use carpathia_adr::adr::enrich_adr;
pub mod parse_db_schema;
pub mod postgres;

pub mod traits;
