/**
 * This module is responsible for managing the cache file that stores the hash of the
 * database schema information. The cache file is used to determine if there have been
 * any changes in the database schema since the last time it was generated. The cache
 * file is stored in a specified directory and is named "`carpathia_cache.json`".
 * The cache content is stored as a JSON object where the keys are the names of the
 * database entities (e.g., table names) and the values are the hashes of their schema
 * information. When it generate code based on the database schema, it will first check
 * the cache to see if there have been any changes. If there are changes, it will
 * update the cache file with the new hashes. If there are no changes,
 * it can skip the code generation process for those entities.
 *
 */
use super::cache_structs::{CacheFile, CacheFileDiff, compare_cache_files};
use crate::configuration::carpathia_conf::CarpathiaConfig;
use crate::db::db_schema_structs::AbstractDbRepr;
use crate::return_values::carpathia_errors::CarpathiaError;
use log::{error, info};

use std::{collections::BTreeMap, fs, path::PathBuf};

pub struct Cache {}

impl Cache {
    pub fn get_changed_entities(
        config: &CarpathiaConfig,
        new_content: &AbstractDbRepr,
        templates: &BTreeMap<String, PathBuf>,
    ) -> Result<CacheFileDiff, CarpathiaError> {
        let old_cache = match CacheFile::from_file(&config.cache_file) {
            Ok(cache) => cache,
            Err(e) => {
                error!(
                    "Failed to read cache file at {:?}: {}",
                    &config.cache_file, e
                );
                CacheFile::new()
            }
        };
        let new_cache = CacheFile::from_new_run(new_content, templates);
        let cache_diff = compare_cache_files(&old_cache, &new_cache, config.cache_modus);
        match new_cache.save_to_file(&config.cache_file) {
            Ok(()) => {
                info!(
                    "Cache file updated successfully at {}",
                    &config.cache_file.display()
                );
                Ok(cache_diff)
            }
            Err(e) => {
                error!("Failed to write cache file: {e}");
                Err(e)
            }
        }
    }

    #[allow(dead_code)]
    fn remove_cache_file(config: &CarpathiaConfig) {
        /*
         * This function is used in the tests to ensure that we start with a clean slate for the cache. It will try to remove the cache file if it exists. If the file is successfully removed, it will log a success message. If the file cannot be removed (e.g., due to permissions issues), it will log an error message. This function is not intended to be used in the main application logic,
         * but rather as a utility for testing purposes.
         */
        //let cache_file_path = format!("{}/{}", &self.path, CACHE_FILE_NAME);
        if fs::remove_file(&config.cache_file).is_ok() {
            info!(
                "Cache file removed successfully at {}",
                &config.cache_file.display()
            );
        } else {
            error!(
                "Failed to remove cache file at {}",
                &config.cache_file.display()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::cache_file::Cache;
    use crate::configuration::conf_enums::{CacheModus, DbPool};
    use crate::configuration::conf_structs::Types;
    use crate::db::db_schema_structs::AbstractAttribute;
    use crate::db::db_schema_structs::AbstractDbRepr;
    use crate::db::db_schema_structs::{
        ABSTRACT_DB_REPR_VERSION, AbstractTableRepr, ConstraintType, IsGenerated, IsIdentity,
        IsNullable, ObjectType,
    };
    use std::collections::{BTreeMap, BTreeSet};
    const TEMPLATES: &BTreeMap<String, PathBuf> = &BTreeMap::new();
    fn create_abstract_db_repr(
        table_name: &str,
        column_name: &str,
        object_type: ObjectType,
    ) -> AbstractDbRepr {
        let atr = create_abstract_selectable(table_name, column_name, object_type);
        let mut db_repr = AbstractDbRepr {
            version: ABSTRACT_DB_REPR_VERSION.to_string(),
            tables: BTreeMap::new(),
            views: BTreeMap::new(),
        };
        db_repr.tables.insert(table_name.to_string(), atr);
        db_repr
    }

    fn create_abstract_selectable(
        table_name: &str,
        column_name: &str,
        object_type: ObjectType,
    ) -> AbstractTableRepr {
        let mut abstract_attribte_map: BTreeMap<String, AbstractAttribute> = BTreeMap::new();
        abstract_attribte_map.insert(
            column_name.to_string(),
            AbstractAttribute {
                column_name: column_name.to_string(),
                data_type: "integer".to_string(),
                u_type: "whatever".to_string(),
                is_nullable: "NO".parse().unwrap_or(IsNullable::No),
                column_default: Some("nextval('users_id_seq'::regclass)".to_string()),
                character_maximum_length: None,
                numeric_precision: Some(32),
                numeric_scale: Some(0),
                is_identity: "NO".parse().unwrap_or(IsIdentity::No),
                identity_generation: None,
                is_generated: "NO".parse().unwrap_or(IsGenerated::Always),
                generation_expression: None,
                constraint_name: Some("users_pkey".to_string()),
                constraint_type: "PRIMARY KEY".parse().unwrap_or(ConstraintType::None),
                referenced_table: None,
                referenced_column: None,
                comment: Some("Primary key for users table".to_string()),
            },
        );
        AbstractTableRepr {
            table_name: table_name.to_string(),
            object_type,
            u_imports: BTreeSet::new(),
            comment: Some("Test table".to_string()),
            attributes: abstract_attribte_map,
        }
    }

    fn get_config_with_cache_modus(cache_modus: CacheModus) -> CarpathiaConfig {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache_file_path = temp_dir.path().to_path_buf().join("carpathia_cache.json");
        CarpathiaConfig {
            db_pool: DbPool::Dummy,
            cache_modus,
            template_directory: tempfile::tempdir().unwrap().path().to_path_buf(),
            output_directory: tempfile::tempdir().unwrap().path().to_path_buf(),
            cache_file: cache_file_path,
            type_map: Types::new(),
            print_schema: false,
            print_db_types: false,
            execute_templates: false,
        }
    }

    #[test]
    fn test_get_changed_entities() {
        let config = get_config_with_cache_modus(CacheModus::UseCache);
        //let cache = Cache::new(temp_dir.path().to_path_buf(), CacheModus::UseCache);
        let new_content: AbstractDbRepr =
            create_abstract_db_repr("test_table", "test_column", ObjectType::BaseTable);
        match Cache::get_changed_entities(&config, &new_content, TEMPLATES) {
            Ok(result) => {
                assert_eq!(
                    result.tables.to_generate.len(),
                    1,
                    "Should have one changed entry - got: {:?}",
                    result.tables.to_generate
                );
                assert_eq!(
                    result.tables.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.tables.to_remove
                );
                assert_eq!(
                    result.views.to_generate.len(),
                    0,
                    "Should have one changed entry - got: {:?}",
                    result.views.to_generate
                );
                assert_eq!(
                    result.views.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.views.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        };

        Cache::remove_cache_file(&config);
    }

    #[test]
    fn test_get_changed_entities_but_no_changes() {
        let config = get_config_with_cache_modus(CacheModus::UseCache);
        //let cache = Cache::new(temp_dir.path().to_path_buf(), CacheModus::UseCache);
        //cache.remove_cache_file(); // Ensure we start with a clean slate
        let mut new_content: AbstractDbRepr =
            create_abstract_db_repr("test_table", "test_column", ObjectType::BaseTable);
        match Cache::get_changed_entities(&config, &new_content, TEMPLATES) {
            Ok(result) => {
                assert_eq!(
                    result.tables.to_generate.len(),
                    1,
                    "Should have one changed entry - got: {:?}",
                    result.tables.to_generate
                );
                assert_eq!(
                    result.tables.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.tables.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        };

        //let cache_after_first_run = Cache::new(temp_dir.path().to_path_buf(), CacheModus::UseCache);

        match Cache::get_changed_entities(&config, &new_content, TEMPLATES) {
            Ok(result) => {
                assert_eq!(
                    result.tables.to_generate.len(),
                    0,
                    "Should have no changed entries - got: {:?}",
                    result.tables.to_generate
                );
                assert_eq!(
                    result.tables.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.tables.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        }

        new_content.tables.insert(
            "test_table_brand_new".to_string(),
            create_abstract_selectable(
                "test_table_brand_new",
                "test_column",
                ObjectType::BaseTable,
            ),
        );
        //let config_third_run = get_config_with_cache_modus(CacheModus::UseCache);
        match Cache::get_changed_entities(&config, &new_content, TEMPLATES) {
            Ok(result) => {
                assert_eq!(
                    result.tables.to_generate.len(),
                    1,
                    "Should have one changed entry - got: {:?}",
                    result.tables.to_generate
                );
                assert_eq!(
                    result.tables.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.tables.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        }

        Cache::remove_cache_file(&config);
    }

    #[test]
    fn test_get_changed_entities_with_forced() {
        let config = get_config_with_cache_modus(CacheModus::BypassCache);
        let new_content: AbstractDbRepr =
            create_abstract_db_repr("test_table", "test_column", ObjectType::BaseTable);

        match Cache::get_changed_entities(&config, &new_content, TEMPLATES) {
            Ok(result) => {
                assert_eq!(
                    result.tables.to_generate.len(),
                    1,
                    "Should have one changed entry - got: {:?}",
                    result.tables.to_generate
                );
                assert_eq!(
                    result.tables.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.tables.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        };
        let config_after_first_run = get_config_with_cache_modus(CacheModus::BypassCache);
        match Cache::get_changed_entities(&config_after_first_run, &new_content, TEMPLATES) {
            Ok(result) => {
                assert_eq!(
                    result.tables.to_generate.len(),
                    1,
                    "Should have one changed entry due to forced option - got: {:?}",
                    result.tables.to_generate
                );
                assert_eq!(
                    result.tables.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.tables.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        }

        Cache::remove_cache_file(&config_after_first_run);
    }

    #[test]
    fn test_cache_removed_entries() {
        let config = get_config_with_cache_modus(CacheModus::UseCache);
        let mut new_content: AbstractDbRepr =
            create_abstract_db_repr("test_table", "test_column", ObjectType::BaseTable);

        match Cache::get_changed_entities(&config, &new_content, TEMPLATES) {
            Ok(result) => {
                assert_eq!(
                    result.tables.to_generate.len(),
                    1,
                    "Should have one changed entry - got: {:?}",
                    result.tables.to_generate
                );
                assert_eq!(
                    result.tables.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.tables.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        };

        // Now we remove the entry from the new content and check if it is detected as removed
        new_content.tables.remove("test_table");
        //let config_third_run = get_config_with_cache_modus(CacheModus::UseCache);
        match Cache::get_changed_entities(&config, &new_content, TEMPLATES) {
            Ok(result) => {
                assert_eq!(
                    result.tables.to_generate.len(),
                    0,
                    "Should have no changed entries - got: {:?}",
                    result.tables.to_generate
                );
                assert_eq!(
                    result.tables.to_remove.len(),
                    1,
                    "Should have one removed entry - got: {:?}",
                    result.tables.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        }

        Cache::remove_cache_file(&config);
    }
}
