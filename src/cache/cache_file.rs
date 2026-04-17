use crate::db::db_schema_structs::ColumnInfo;
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
        let mut entries_to_remove: Vec<String> = Vec::new();
        let mut new_entries_not_in_old: HashMap<String, Vec<ColumnInfo>> = new_content.clone();
        let mut changed_new_entries: Vec<String> = Vec::new();
        let mut new_cache_content: HashMap<String, String> = HashMap::new();
        for key in self.content.keys() {
            if !new_content.contains_key(key) {
                entries_to_remove.push(key.clone());
            }
            new_entries_not_in_old.remove(key);
            let old_hash = match self.content.get(key) {
                Some(hash) => hash,
                None => &"NO_OLD_HASH".to_string(), // This case is already handled by the previous check
            };
            let new_hash = match new_content.get(key) {
                Some(column_info) => match to_json_hash(column_info) {
                    Ok(hash) => hash,
                    Err(_e) => "NO_NEW_HASH".to_string(),
                },
                None => "NO_NEW_HASH".to_string(), // This case is already handled by the previous check
            };
            new_cache_content.insert(key.clone(), new_hash.clone());
            if old_hash != &new_hash {
                changed_new_entries.push(key.clone());
            }
        }
        for (key, column_info) in new_entries_not_in_old.iter() {
            let new_hash = match to_json_hash(column_info) {
                Ok(hash) => hash,
                Err(_e) => "NO_NEW_HASH".to_string(),
            };
            new_cache_content.insert(key.clone(), new_hash);
            changed_new_entries.push(key.clone());
        }

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

        changed_new_entries
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
        let cache = Cache::new("test_cache".to_string());

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
        let cache_after_first_run = Cache::new("test_cache".to_string());
        let changed_entities = cache_after_first_run.get_changed_entities(&new_content);
        assert_eq!(changed_entities.len(), 0);
        cache_after_first_run.remove_cache_file();
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
