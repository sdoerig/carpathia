#[derive(sqlx::FromRow, serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]

pub(crate) struct PgColumnInfo {
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

#[derive(sqlx::FromRow, serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]

pub(crate) struct PgConstraintInfo {
    pub constraint_oid: i64,
    pub constraint_name: String,
    pub schema_name: String,
    pub table_schema: String,
    pub table_name: String,
    pub table_kind: String,
    pub table_oid: i64,
    pub constraint_type: String,
    pub referenced_table_oid: Option<i64>,
    pub referenced_table: Option<String>,
    pub column_attnum: i32,
    pub column_name: String,
    pub referenced_attnum: Option<i32>,
    pub referenced_column: Option<String>,
    pub key_position: i32,
    pub constraint_definition: String,
}
