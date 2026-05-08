

#[derive(sqlx::FromRow, serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub (crate) struct PgColumnInfo {
    pub object_type: String,
    pub table_name: String,
    pub column_name: String,
    pub data_type: String,
    pub array_dimensions: Option<i32>,
    pub is_nullable: String,
    pub column_default: Option<String>,
    pub table_is_insertable: String,
    pub column_is_updatable: String,
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
    pub table_comment: Option<String>,
    pub column_comment: Option<String>,
}
