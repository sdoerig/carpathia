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
use crate::db::db_schema_structs::AbstractDbRepr;
use crate::return_values::carpathia_errors::{CarpathiaError, ErrorNumber};
use log::{error, info};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::{collections::HashMap, fs};

const CACHE_FILE_NAME: &str = "carpathia_cache.json";
pub(crate) struct Cache {
    path: PathBuf,
    forced: bool,
    content: HashMap<String, String>,
}

pub(crate) struct CacheResult {
    pub to_generate: Vec<String>,
    pub to_remove: Vec<String>,
}

impl std::fmt::Debug for CacheResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CacheResult")
            .field("to_generate", &self.to_generate)
            .field("to_remove", &self.to_remove)
            .finish()
    }
}

impl std::fmt::Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cache")
            .field("path", &self.path)
            .field("forced", &self.forced)
            .field("content", &self.content)
            .finish()
    }
}

impl Cache {
    pub(crate) fn new(path: String, forced: bool) -> Self {
        /*
         * When it create a new cache, it will try to read the existing cache file if it exists.
         * If the file does not exist, it will start with an empty cache. The cache content will be stored as a HashMap where the key is the name of the database entity (e.g., table name) and the value is a hash of the entity's schema information. This way, it can easily compare the new schema information with the cached information to determine if there have been any changes.
         * If the cache file exists but cannot be read (e.g., due to permissions issues),
         * it will also start with an empty cache and log an error message.
         * The cache file will be created or updated when it write the new cache content after
         * comparing it with the new schema information.
         *
         */
        let cache_dir: PathBuf = PathBuf::from(&path);
        let cache_file_path = cache_dir.join(CACHE_FILE_NAME);
        let file_content = std::fs::read_to_string(&cache_file_path).unwrap_or_else(|_| {
            // If the file doesn't exist, it can start with an empty cache
            info!(
                "Cache file not found at {}, starting with an empty cache.",
                &path
            );
            "{}".to_string() // Return an empty JSON object as a string
        });
        let content = serde_json::from_str(&file_content).unwrap_or_default();
        Self {
            path: cache_file_path,
            forced,
            content,
        }
    }

    pub(crate) fn get_changed_entities(
        &self,
        new_content: &HashMap<String, AbstractDbRepr>,
    ) -> Result<CacheResult, CarpathiaError> {
        // it will compare the new content with the old content and determine which entries have changed
        // it will create a new cache content based on the new content and write it to the cache file
        let mut new_cached_entries: HashMap<String, String> = HashMap::new();
        let mut to_generate: Vec<String> = Vec::new();
        let mut to_remove: Vec<String> = Vec::new();
        let mut new_cache_content: HashMap<String, String> = HashMap::new();
        for (key, column_info) in new_content {
            new_cached_entries.insert(
                key.clone(),
                to_json_hash(column_info).unwrap_or_else(|_e| "NO_NEW_HASH".to_string()),
            );
        }
        for key in self.content.keys() {
            if !new_cached_entries.contains_key(key) {
                info!(
                    "Entry '{key}' is present in the old cache but not in the new content. It will be removed from the cache."
                );
                to_remove.push(key.clone());
            }
        }
        // Now it will check for changed entries and also build the new cache content
        // It will iterate over the new cached entries and compare them with the old cached entries. If an entry is new or has a different hash than the old entry, it will be added to the list of entries to generate.
        // It will also build the new cache content based on the new cached entries.
        for key in new_cached_entries.keys() {
            let old_hash = self.content.get(key);
            let new_hash = new_cached_entries.get(key);
            new_cache_content.insert(key.clone(), new_hash.unwrap().clone());
            if old_hash != new_hash || self.forced {
                to_generate.push(key.clone());
            }
        }

        match self.write_cache_file(new_cache_content) {
            Ok(()) => info!("Cache file updated successfully."),
            Err(e) => error!("Failed to update cache file: {e}"),
        }
        Ok(CacheResult {
            to_generate,
            to_remove,
        })
    }

    fn write_cache_file(
        &self,
        new_cache_content: HashMap<String, String>,
    ) -> Result<(), CarpathiaError> {
        match fs::create_dir_all(self.path.parent().unwrap()) {
            Ok(()) => {
                //let cache_file_path = format!("{}/{}", &self.path, CACHE_FILE_NAME);
                let cache_content_json = serde_json::to_string_pretty(&new_cache_content).unwrap();
                match fs::write(&self.path, cache_content_json) {
                    Ok(()) => {
                        info!(
                            "Cache file updated successfully at {}",
                            &self.path.display()
                        );
                        Ok(())
                    }
                    Err(e) => {
                        error!("Failed to write cache file: {e}");
                        Err(CarpathiaError {
                            message: "Failed to write cache file".to_string(),
                            error_type: ErrorNumber::CacheFileError,
                        })
                    }
                }
            }
            Err(e) => {
                error!("Failed to create cache directory: {e}");
                Err(CarpathiaError {
                    message: "Failed to create cache directory".to_string(),
                    error_type: ErrorNumber::CacheFileError,
                })
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

fn to_json_hash(column_info: &AbstractDbRepr) -> Result<String, Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string(column_info)?;
    let mut hasher = Sha256::new();
    hasher.update(json_string.as_bytes());
    let hash_result = hasher.finalize();
    Ok(format!("{hash_result:x}"))
}

#[cfg(test)]
mod tests {
    use crate::db::db_schema_structs::AbstractAttribute;

    use super::*;

    fn create_column_info(table_name: &str, column_name: &str) -> AbstractDbRepr {
        AbstractDbRepr {
            table_name: table_name.to_string(),
            attributes: vec![AbstractAttribute {
                column_name: column_name.to_string(),
                data_type: "text".to_string(),
                is_nullable: "NO".to_string(),
                column_default: None,
                character_maximum_length: None,
                numeric_precision: None,
                numeric_scale: None,
                is_identity: "NO".to_string(),
                identity_generation: None,
                is_generated: "NO".to_string(),
                generation_expression: None,
                constraint_name: None,
                constraint_type: None,
                referenced_table: None,
                referenced_column: None,
            }],
        }
    }

    #[test]
    fn test_get_changed_entities() {
        let cache = Cache::new("test_cache".to_string(), false);
        let mut new_content: HashMap<String, AbstractDbRepr> = HashMap::new();

        new_content.insert(
            "test_table".to_string(),
            create_column_info("test_table", "a"),
        );
        match cache.get_changed_entities(&new_content) {
            Ok(result) => {
                assert_eq!(
                    result.to_generate.len(),
                    1,
                    "Should have one changed entry - got: {:?}",
                    result.to_generate
                );
                assert_eq!(
                    result.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        };
        cache.remove_cache_file();
    }

    #[test]
    fn test_get_changed_entities_but_no_changes() {
        let cache = Cache::new("test_cache_no_changes".to_string(), false);
        cache.remove_cache_file(); // Ensure we start with a clean slate
        let mut new_content: HashMap<String, AbstractDbRepr> = HashMap::new();
        new_content.insert(
            "test_table".to_string(),
            create_column_info("test_table", "a"),
        );
        match cache.get_changed_entities(&new_content) {
            Ok(result) => {
                assert_eq!(
                    result.to_generate.len(),
                    1,
                    "Should have one changed entry - got: {:?}",
                    result.to_generate
                );
                assert_eq!(
                    result.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        };

        let cache_after_first_run = Cache::new("test_cache_no_changes".to_string(), false);

        match cache_after_first_run.get_changed_entities(&new_content) {
            Ok(result) => {
                assert_eq!(
                    result.to_generate.len(),
                    0,
                    "Should have no changed entries - got: {:?}",
                    result.to_generate
                );
                assert_eq!(
                    result.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        }

        new_content.insert(
            "test_table_brand_new".to_string(),
            create_column_info("test_table_brand_new", "test_column"),
        );
        let cache_third_run = Cache::new("test_cache_no_changes".to_string(), false);
        match cache_third_run.get_changed_entities(&new_content) {
            Ok(result) => {
                assert_eq!(
                    result.to_generate.len(),
                    1,
                    "Should have one changed entry - got: {:?}",
                    result.to_generate
                );
                assert_eq!(
                    result.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        }

        cache_third_run.remove_cache_file();
    }

    #[test]
    fn test_get_changed_entities_with_forced() {
        let cache = Cache::new("test_cache_forced".to_string(), true);
        let mut new_content: HashMap<String, AbstractDbRepr> = HashMap::new();
        new_content.insert(
            "test_table".to_string(),
            create_column_info("test_table", "a"),
        );
        match cache.get_changed_entities(&new_content) {
            Ok(result) => {
                assert_eq!(
                    result.to_generate.len(),
                    1,
                    "Should have one changed entry - got: {:?}",
                    result.to_generate
                );
                assert_eq!(
                    result.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        };
        let cache_after_first_run = Cache::new("test_cache_forced".to_string(), true);
        match cache_after_first_run.get_changed_entities(&new_content) {
            Ok(result) => {
                assert_eq!(
                    result.to_generate.len(),
                    1,
                    "Should have one changed entry due to forced option - got: {:?}",
                    result.to_generate
                );
                assert_eq!(
                    result.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        }

        cache_after_first_run.remove_cache_file();
    }
    #[test]
    fn test_to_json_hash() {
        let column_info = create_column_info("table_name", "column_name");
        let hash = to_json_hash(&column_info).unwrap();
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_cache_removed_entries() {
        let cache = Cache::new("test_cache_removed_entries".to_string(), false);
        let mut new_content: HashMap<String, AbstractDbRepr> = HashMap::new();
        new_content.insert(
            "test_table".to_string(),
            create_column_info("test_table", "a"),
        );
        match cache.get_changed_entities(&new_content) {
            Ok(result) => {
                assert_eq!(
                    result.to_generate.len(),
                    1,
                    "Should have one changed entry - got: {:?}",
                    result.to_generate
                );
                assert_eq!(
                    result.to_remove.len(),
                    0,
                    "Should have no removed entries - got: {:?}",
                    result.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        };

        // Now we remove the entry from the new content and check if it is detected as removed
        new_content.remove("test_table");
        let cache_third_run = Cache::new("test_cache_removed_entries".to_string(), false);
        match cache_third_run.get_changed_entities(&new_content) {
            Ok(result) => {
                assert_eq!(
                    result.to_generate.len(),
                    0,
                    "Should have no changed entries - got: {:?}",
                    result.to_generate
                );
                assert_eq!(
                    result.to_remove.len(),
                    1,
                    "Should have one removed entry - got: {:?}",
                    result.to_remove
                );
            }
            Err(e) => panic!("Expected Ok result but got Err: {}", e),
        }

        cache_third_run.remove_cache_file();
    }
}
