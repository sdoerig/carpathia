use crate::db::db_schema_structs::ColumnInfo;
use core::hash;
use log::{error, info};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, fs};

const CACHE_FILE_NAME: &str = "carpathia_cache.json";
pub(crate) struct Cache {
    path: String,
    content: HashMap<String, String>,
}

impl Cache {
    pub(crate) fn new(path: String) -> Self {
        let file_content = std::fs::read_to_string(format!("{}/{}", &path, CACHE_FILE_NAME))
            .unwrap_or_else(|_| {
                // If the file doesn't exist, we can start with an empty cache
                info!(
                    "Cache file not found at {}, starting with an empty cache.",
                    &path
                );
                "{}".to_string() // Return an empty JSON object as a string
            });
        let content = serde_json::from_str(&file_content).unwrap_or_default();
        Self { path, content }
    }

    pub(crate) fn get_changed_entities(
        &self,
        new_content: &HashMap<String, Vec<ColumnInfo>>,
    ) -> Vec<String> {
        let mut new_entries_not_in_old: HashMap<String, String> = HashMap::new();
        let mut changed_new_entries: Vec<String> = Vec::new();
        let mut new_cache_content: HashMap<String, String> = HashMap::new();
        for (key, column_info) in new_content.iter() {
            new_entries_not_in_old.insert(
                key.clone(),
                to_json_hash(column_info).unwrap_or_else(|_e| "NO_NEW_HASH".to_string()),
            );
        }
        // Can be an old endtry does not appear in the new content,
        // then this means that the table was removed, so we should remove it from the cache and not consider it as a changed entry
        for key in new_entries_not_in_old.keys() {
            let old_hash = match self.content.get(key) {
                Some(hash) => hash,
                None => &"NO_OLD_HASH".to_string(), // This case is already handled by the previous check
            };
            let new_hash = match new_entries_not_in_old.get(key) {
                Some(hash) => hash,
                None => &"NO_NEW_HASH".to_string(), // This case is already handled by the previous check
            };
            new_cache_content.insert(key.clone(), new_hash.clone());
            if old_hash != new_hash {
                changed_new_entries.push(key.clone());
            }
        }
        for (key, new_hash) in new_entries_not_in_old.iter() {
            new_cache_content.insert(key.clone(), new_hash.clone());
            //changed_new_entries.push(key.clone());
        }

        self.write_cache_file(new_cache_content);

        changed_new_entries
    }

    fn write_cache_file(&self, new_cache_content: HashMap<String, String>) {
        match fs::create_dir_all(&self.path) {
            Ok(_) => {
                let cache_file_path = format!("{}/{}", &self.path, CACHE_FILE_NAME);
                let cache_content_json = serde_json::to_string_pretty(&new_cache_content).unwrap();
                match fs::write(&cache_file_path, cache_content_json) {
                    Ok(_) => info!("Cache file updated successfully at {}", &cache_file_path),
                    Err(e) => error!("Failed to write cache file: {}", e),
                }
            }
            Err(e) => error!("Failed to create cache directory: {}", e),
        }
    }

    fn remove_cache_file(&self) {
        let cache_file_path = format!("{}/{}", &self.path, CACHE_FILE_NAME);
        if fs::remove_file(&cache_file_path).is_ok() {
            info!("Cache file removed successfully at {}", &cache_file_path);
        } else {
            error!("Failed to remove cache file at {}", &cache_file_path);
        }
    }
}

fn to_json_hash(column_info: &Vec<ColumnInfo>) -> Result<String, Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string(column_info)?;
    let mut hasher = Sha256::new();
    hasher.update(json_string.as_bytes());
    let hash_result = hasher.finalize();
    Ok(format!("{:x}", hash_result))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_changed_entities() {
        let cache = Cache::new("test_cache".to_string());
        let mut new_content: HashMap<String, Vec<ColumnInfo>> = HashMap::new();
        new_content.insert(
            "test_table".to_string(),
            vec![ColumnInfo {
                table_name: "test_table".to_string(),
                column_name: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    .to_string(), // Use a dynamic value to ensure the hash changes
                data_type: "VARCHAR".to_string(),
                is_nullable: "YES".to_string(),
                column_default: None,
                character_maximum_length: Some(255),
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
        );
        let changed_entities = cache.get_changed_entities(&new_content);
        assert_eq!(changed_entities.len(), 1);
        assert_eq!(changed_entities[0], "test_table".to_string());
    }

    #[test]
    fn test_get_changed_entities_but_no_changes() {
        let cache = Cache::new("test_cache_no_changes".to_string());
        cache.remove_cache_file(); // Ensure we start with a clean slate
        let mut new_content: HashMap<String, Vec<ColumnInfo>> = HashMap::new();
        new_content.insert(
            "test_table".to_string(),
            vec![ColumnInfo {
                table_name: "test_table".to_string(),
                column_name: "the_column".to_string(), // Always the same
                data_type: "VARCHAR".to_string(),
                is_nullable: "YES".to_string(),
                column_default: None,
                character_maximum_length: Some(255),
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
        );
        let changed_entities = cache.get_changed_entities(&new_content);
        assert_eq!(changed_entities.len(), 1);
        let cache_after_first_run = Cache::new("test_cache_no_changes".to_string());
        assert_eq!(cache_after_first_run.content.len(), 1);
        let changed_entities = cache_after_first_run.get_changed_entities(&new_content);
        assert_eq!(
            changed_entities.len(),
            0,
            "Should have no changed entries - got: {:?}",
            changed_entities
        );

        new_content.insert(
            "test_table_brand_new".to_string(),
            vec![ColumnInfo {
                table_name: "test_table_brand_new".to_string(),
                column_name: "the_column".to_string(), // Always the same
                data_type: "VARCHAR".to_string(),
                is_nullable: "YES".to_string(),
                column_default: None,
                character_maximum_length: Some(255),
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
        );
        let cache_third_run = Cache::new("test_cache_no_changes".to_string());
        let changed_entities = cache_third_run.get_changed_entities(&new_content);
        assert_eq!(
            changed_entities.len(),
            1,
            "Should have one changed entry - got: {:?}",
            changed_entities
        );
        assert!(cache_third_run.content.get("test_table").is_some());
        assert!(changed_entities[0] == "test_table_brand_new".to_string());
        cache_third_run.remove_cache_file();
    }

    #[test]
    fn test_to_json_hash() {
        let column_info = vec![ColumnInfo {
            table_name: "test_table".to_string(),
            column_name: "test_column".to_string(),
            data_type: "VARCHAR".to_string(),
            is_nullable: "YES".to_string(),
            column_default: None,
            character_maximum_length: Some(255),
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
        }];
        let hash = to_json_hash(&column_info).unwrap();
        assert!(!hash.is_empty());
    }
}
