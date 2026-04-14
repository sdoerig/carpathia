use crate::db::db_schema_structs::ColumnInfo;
use std::collections::HashMap;

pub(crate) struct Cache {
    path: String,
    content: HashMap<String, String>,
}

impl Cache {
    pub(crate) fn new(path: String) -> Self {
        let file_content = std::fs::read_to_string(&path).unwrap_or_else(|_| {
            // If the file doesn't exist, we can start with an empty cache
            println!(
                "Cache file not found at {}, starting with an empty cache.",
                &path
            );
            "{}".to_string() // Return an empty JSON object as a string
        });
        let content = serde_json::from_str(&file_content).unwrap_or_default();
        Self { path, content }
    }
    pub(crate) fn get_changed_enties(
        &self,
        new_content: &HashMap<String, ColumnInfo>,
    ) -> Vec<String> {
        let mut entries_to_remove: Vec<String> = Vec::new();
        let mut new_entries_not_in_old: HashMap<String, ColumnInfo> = new_content.clone();
        let mut changed_new_entries: Vec<String> = Vec::new();
        for (key) in self.content.keys() {
            if !new_content.contains_key(key) {
                entries_to_remove.push(key.clone());
            }
            new_entries_not_in_old.remove(key);
            let old_hash = match self.content.get(key) {
                Some(hash) => hash,
                None => &"NO_OLD_HASH".to_string(), // This case is already handled by the previous check
            };
            let new_hash = match new_content.get(key) {
                Some(column_info) => match column_info.to_json_hash() {
                    Ok(hash) => hash,
                    Err(e) => "NO_NEW_HASH".to_string(),
                },
                None => "NO_NEW_HASH".to_string(), // This case is already handled by the previous check
            };
        }
        changed_new_entries
    }

    pub(crate) fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json_content = serde_json::to_string_pretty(&self.content)?;
        std::fs::write(&self.path, json_content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cache() {
        let cache_path = "test_cache.json".to_string();
        let mut cache = Cache::new(cache_path.clone());
        cache
            .content
            .insert("key1".to_string(), "value1".to_string());
        cache
            .content
            .insert("key2".to_string(), "value2".to_string());
        cache.save().unwrap();
        let loaded_cache = Cache::new(cache_path.clone());
        assert_eq!(cache.content, loaded_cache.content);
        std::fs::remove_file(cache_path).unwrap();
    }
}
