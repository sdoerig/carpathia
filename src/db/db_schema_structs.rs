use sha2::{Digest, Sha256};
use serde_json;

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub(crate) struct ColumnInfo {
    pub table_name: String,
    pub column_name: String,
    pub data_type: String,
    pub is_nullable: String,
    pub column_default: Option<String>,
    pub character_maximum_length: Option<i32>,
    pub numeric_precision: Option<i32>,
    pub numeric_scale: Option<i32>,
    pub is_identity: String,
    pub identity_generation: Option<String>,
    pub is_generated: String,
    pub generation_expression: Option<String>,
    pub constraint_name: Option<String>,
    pub constraint_type: Option<String>,
    pub referenced_table: Option<String>,
    pub referenced_column: Option<String>,
}

impl ColumnInfo {
    pub(crate) fn to_json_hash(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Serialize to JSON string
        let json = serde_json::to_string(self)?;
        
        // Compute SHA-256 hash
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        let hash = hasher.finalize();
        
        // Encode hash as hex string
        Ok(hex::encode(hash))
    }
}
