//! cache
//! Any generation operation is cached in a JSON file. The cache is needed to avoit
//! unnecessary generation of unchanged entities. Entities (Database entities or templates) are cached using blake2 hashing.
//! Hashing is based on deterministic entitiy - so with in databases entities BTreeMap or -Set are
//! uses to achieve this.
//! Regeneration is done if one or more of these conditions are true:
//! - The entity is not in the cache
//! - The entity is in the cache but the hash has changed
//! - The template has changed
//! - Cache override CacheModus::BypassCache is set
//!
//! For more iinformation see cache_file.rs and/or

pub mod carpathia_conf;
pub mod conf_enums;
pub mod conf_file_reader;
pub mod conf_structs;
