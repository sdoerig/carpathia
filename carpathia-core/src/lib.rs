//! carpathia-core
//! 
//! Language agnostic database generator. Short description of the structure:
//! 
//! - cache
//!   Any generation operation is cached in a JSON file. The cache is needed to avoit 
//!   unnecessary generation of unchanged entities. Entities (Database entities or templates) are cached using blake2 hashing.
//!   Hashing is based on deterministic entitiy - so with in databases entities BTreeMap or -Set are 
//!   uses to achieve this.
//!   Regeneration is done if one or more of these conditions are true: 
//!   - The entity is not in the cache
//!   - The entity is in the cache but the hash has changed
//!   - The template has changed
//!   - Cache override CacheModus::BypassCache is set
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
//!  
//!   Building the configuration means
//!   - user defined type mapping are deserialized within the configuration 
//!   - database pool is being configurated

pub mod cache;
pub mod configuration;
pub mod db;
pub mod generator;
pub mod return_values;
pub mod templates;
