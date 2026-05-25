/// Serializable type mapping structur.
/// Provides mapping from DB-Types to custom types.
use crate::db::db_schema_structs::ABSTRACT_DB_REPR_VERSION;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub(crate) const DEFAULT_TYPE_MAPPING: &TypeMapping = &TypeMapping {
    u_import: Some(String::new()),
    u_type: String::new(),
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Types {
    pub version: String,
    pub type_mapping: BTreeMap<String, TypeMapping>,
}

impl Default for Types {
    fn default() -> Self {
        Self::new()
    }
}

impl Types {
    pub fn new() -> Self {
        Types {
            version: ABSTRACT_DB_REPR_VERSION.to_string(),
            type_mapping: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeMapping {
    pub u_import: Option<String>,
    pub u_type: String,
}
