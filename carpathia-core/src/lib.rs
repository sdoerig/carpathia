//! Language agnostic database code generator. 
//! - tera 2 based
//! - delta aware - just generate what has changed.
//! - abstract database representation - assemble the data smart, keep the templates simple.
//! - tested against PostgreSQL version 13 to 18
//! - carpathia is not an ORM - and will never be one
//! - it is declarative, leaves you the freedom what to generate
//! - allows you to map database types to your own custom types
//! - offers a mapping of database names (e.g. tables, view, attributes) to code friendley names.
//!   Exampel: in the database is a table named  `match`. This will not work when generating Rust
//! 
//! A run consists of 
//! - building [CarpathiaConf](crate::configuration::carpathia_conf::CarpathiaConfigBuilder)
//! - deciding what to do
//!   - print database type mapping
//!     - (introspect)[crate::db] the database
//!       which delivers the internal model (abstract database represenation)[crate::db::db_schema_structs::AbstractDbRepr]
//! 
//!   - executing the tera templates
//!     - (introspect)[crate::db] the database
//!     - call (generate_code)[crate::generator::template_engine::TemplateEngine::generate_code]
//!   - initializing templates - so you don't have to start from zero
//!   
//! 
//! - configuration
//!   Contains the configuration any part of carpatia-core can be passed to. The configuration contains
//!   of a configuration struct and a configuration builder.
//!   The configuration has two main workingmodes:
//!   - Database connection needed
//!   - No database connection needed (e.g. for template generation only)
//!   
//!   Database connection are needed to these operations:
//!   - executing templates - hence generate code for database entities (tables, views, etc.)
//!   - printing the database type mapping 
//!   - printing the database schema
//!   - printing the abstract database representation
//!  
//!   The database connection is not needed for:
//!   - initializing the template examples
//! 
//! - db
//!   This part contains anything dealing with the database
//! 
//! - generator 
//!   Does all the tera 2 execution and generation work
//! 
//! - return_values
//!   Contains the error definitions and return values
//! 
//! - templates
//!   Any example template collections go in here.

pub mod cache;
pub mod configuration;
pub mod db;
pub mod generator;
pub mod return_values;
pub mod templates;
