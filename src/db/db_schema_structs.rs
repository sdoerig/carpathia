#[derive(sqlx::FromRow)]
pub(crate) struct ColumnInfo {
    pub table_name: String,
    pub column_name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub column_default: Option<String>,
    pub character_maximum_length: Option<i32>,
    pub numeric_precision: Option<i32>,
    pub numeric_scale: Option<i32>,
    pub is_identity: bool,
    pub identity_generation: Option<String>,
    pub is_generated: bool,
    pub generation_expression: Option<String>,
}