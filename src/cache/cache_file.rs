// Copyright 2026 Stefan Dörig
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::cache_structs::{CacheFile, CacheFileDiff, compare_cache_files};
use crate::cache::cache_structs::CacheModus;
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
use crate::db::db_schema_structs::{ABSTRACT_DB_REPR_VERSION, AbstractDbRepr};
use crate::return_values::carpathia_errors::CarpathiaError;
use log::{error, info};

use std::fs;
use std::path::PathBuf;

const CACHE_FILE_NAME: &str = "carpathia_cache.json";

pub(crate) struct Cache {
    path: PathBuf,
    cache_modus: CacheModus,
}

impl std::fmt::Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cache")
            .field("path", &self.path)
            .field("cache_modus", &self.cache_modus)
            .finish()
    }
}

impl Cache {
    pub(crate) fn new(path: PathBuf, cache_modus: CacheModus) -> Self {
        /*
         * When it create a new cache, it will try to read the existing cache file if it exists.
         * If the file does not exist, it will start with an empty cache. The cache content will be stored as a HashMap where the key is the name of the database entity (e.g., table name) and the value is a hash of the entity's schema information. This way, it can easily compare the new schema information with the cached information to determine if there have been any changes.
         * If the cache file exists but cannot be read (e.g., due to permissions issues),
         * it will also start with an empty cache and log an error message.
         * The cache file will be created or updated when it write the new cache content after
         * comparing it with the new schema information.
         *
         */
        let cache_file_path = path.join(CACHE_FILE_NAME);

        Self {
            path: cache_file_path,
            cache_modus,
        }
    }

    pub(crate) fn get_changed_entities(
        &self,
        new_content: &AbstractDbRepr,
    ) -> Result<CacheFileDiff, CarpathiaError> {
        let old_cache = match CacheFile::from_file(&self.path) {
            Ok(cache) => cache,
            Err(e) => {
                error!("Failed to read cache file at {:?}: {}", &self.path, e);
                CacheFile::new()
            }
        };
        let new_cache = CacheFile::from_abstract_db_repr(new_content);
        let cache_diff = compare_cache_files(&old_cache, &new_cache, self.cache_modus);
        match new_cache.save_to_file(&self.path) {
            Ok(()) => {
                info!(
                    "Cache file updated successfully at {}",
                    &self.path.display()
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
    fn remove_cache_file(&self) {
        /*
         * This function is used in the tests to ensure that we start with a clean slate for the cache. It will try to remove the cache file if it exists. If the file is successfully removed, it will log a success message. If the file cannot be removed (e.g., due to permissions issues), it will log an error message. This function is not intended to be used in the main application logic,
         * but rather as a utility for testing purposes.
         */
        //let cache_file_path = format!("{}/{}", &self.path, CACHE_FILE_NAME);
        if fs::remove_file(&self.path).is_ok() {
            info!(
                "Cache file removed successfully at {}",
                &self.path.display()
            );
        } else {
            error!("Failed to remove cache file at {}", &self.path.display());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::cache_file::Cache;
    use crate::cache::cache_structs::CacheModus;
    use crate::db::db_schema_structs::AbstractAttribute;
    use crate::db::db_schema_structs::AbstractDbRepr;
    use crate::db::db_schema_structs::AbstractTableRepr;
    use std::collections::BTreeMap;
    use tempfile::NamedTempFile;
    use tempfile::env::temp_dir;
    enum DbObjectType {
        Table,
        View,
    }
    impl From<DbObjectType> for String {
        fn from(object_type: DbObjectType) -> String {
            match object_type {
                DbObjectType::Table => "BASE TABLE".to_string(),
                DbObjectType::View => "VIEW".to_string(),
            }
        }
    }

    fn create_abstract_db_repr(
        table_name: &str,
        column_name: &str,
        object_type: DbObjectType,
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
        object_type: DbObjectType,
    ) -> AbstractTableRepr {
        let mut abstract_attribte_map: BTreeMap<String, AbstractAttribute> = BTreeMap::new();
        abstract_attribte_map.insert(
            column_name.to_string(),
            AbstractAttribute {
                column_name: column_name.to_string(),
                data_type: "integer".to_string(),
                is_nullable: "NO".to_string(),
                column_default: Some("nextval('users_id_seq'::regclass)".to_string()),
                character_maximum_length: None,
                numeric_precision: Some(32),
                numeric_scale: Some(0),
                is_identity: "NO".to_string(),
                identity_generation: None,
                is_generated: "NO".to_string(),
                generation_expression: None,
                constraint_name: Some("users_pkey".to_string()),
                constraint_type: Some("PRIMARY KEY".to_string()),
                referenced_table: None,
                referenced_column: None,
                comment: Some("Primary key for users table".to_string()),
            },
        );
        let atr = AbstractTableRepr {
            table_name: table_name.to_string(),
            object_type: String::from(object_type),
            comment: Some("Test table".to_string()),
            attributes: abstract_attribte_map,
        };
        atr
    }

    #[test]
    fn test_get_changed_entities() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache = Cache::new(temp_dir.path().to_path_buf(), CacheModus::UseCache);
        let new_content: AbstractDbRepr =
            create_abstract_db_repr("test_table", "test_column", DbObjectType::Table);
        match cache.get_changed_entities(&new_content) {
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

        cache.remove_cache_file();
    }

    #[test]
    fn test_get_changed_entities_but_no_changes() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache = Cache::new(temp_dir.path().to_path_buf(), CacheModus::UseCache);
        //cache.remove_cache_file(); // Ensure we start with a clean slate
        let mut new_content: AbstractDbRepr =
            create_abstract_db_repr("test_table", "test_column", DbObjectType::Table);
        match cache.get_changed_entities(&new_content) {
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

        let cache_after_first_run = Cache::new(temp_dir.path().to_path_buf(), CacheModus::UseCache);

        match cache_after_first_run.get_changed_entities(&new_content) {
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
            create_abstract_selectable("test_table_brand_new", "test_column", DbObjectType::Table),
        );
        let cache_third_run = Cache::new(temp_dir.path().to_path_buf(), CacheModus::UseCache);
        match cache_third_run.get_changed_entities(&new_content) {
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

        cache_third_run.remove_cache_file();
    }

    #[test]
    fn test_get_changed_entities_with_forced() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache = Cache::new(temp_dir.path().to_path_buf(), CacheModus::BypassCache);
        let mut new_content: AbstractDbRepr =
            create_abstract_db_repr("test_table", "test_column", DbObjectType::Table);

        match cache.get_changed_entities(&new_content) {
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
        let cache_after_first_run =
            Cache::new(temp_dir.path().to_path_buf(), CacheModus::BypassCache);
        match cache_after_first_run.get_changed_entities(&new_content) {
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

        cache_after_first_run.remove_cache_file();
    }

    #[test]
    fn test_cache_removed_entries() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache = Cache::new(temp_dir.path().to_path_buf(), CacheModus::UseCache);
        let mut new_content: AbstractDbRepr =
            create_abstract_db_repr("test_table", "test_column", DbObjectType::Table);

        match cache.get_changed_entities(&new_content) {
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
        let cache_third_run = Cache::new(temp_dir.path().to_path_buf(), CacheModus::UseCache);
        match cache_third_run.get_changed_entities(&new_content) {
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

        cache_third_run.remove_cache_file();
    }
}
