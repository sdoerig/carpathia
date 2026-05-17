use crate::db::db_schema_structs::ABSTRACT_DB_REPR_VERSION;
use serde::{Deserialize, Serialize};
/// Serializable type mapping structur.
/// Provides mapping from DB-Types to custom types.
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Types {
    pub version: String,
    pub type_mapping: BTreeMap<String, TypeMapping>,
}

impl Types {
    pub fn new() -> Self {
        Types {
            version: ABSTRACT_DB_REPR_VERSION.to_string(),
            type_mapping: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TypeMapping {
    pub u_import: String,
    pub u_type: Option<String>,
}
