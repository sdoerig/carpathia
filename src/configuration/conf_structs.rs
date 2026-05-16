use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TypeMapping {
    pub u_import: String,
    pub u_type: String,
}