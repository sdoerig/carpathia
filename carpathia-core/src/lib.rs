//! # Language agnostic database code generator.
//! - tera 2 based
//! - delta aware - just generate what has changed.
//!   comes with a caching system,
//! - abstract database representation - assemble the data smart, keep the templates simple.
//! - tested against PostgreSQL version 13 to 18
//! - carpathia is not an ORM - and will never be one
//! - it is declarative, leaves you the freedom what to generate
//! - allows you to map database types to your own custom types
//! - offers a mapping of database names (e.g. tables, view, attributes) to code friendley names.
//!   Exampel: in the database is a table named  `match`. This will not work when generating Rust
//!
//! # Usage
//!
//! - building [CarpathiaConf](crate::configuration::carpathia_conf::CarpathiaConfigBuilder)
//! - deciding what to do
//!   - getting the database type mapping
//!     - (introspect)[crate::db::parse_db_schema] the database
//!       which delivers the internal model (abstract database represenation)[crate::db::db_schema_structs::AbstractDbRepr]
//!     - (get_db_types)[crate::generator::template_engine::get_db_types]
//!
//!   - executing the tera templates
//!     - (introspect)[crate::db::parse_db_schema::DbSchemaParser::parse_schema] the database
//!     - call (generate_code)[crate::generator::template_engine::TemplateEngine::generate_code]
//!
//!   - (initializing)[crate::templates::init_templates::extract_to_disk] templates - so you don't have to start from zero
//!   

pub mod cache;
pub mod configuration;
pub mod db;
pub mod generator;
pub mod return_values;
pub mod templates;
