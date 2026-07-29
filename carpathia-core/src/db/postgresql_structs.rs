use std::collections::BTreeMap;

use crate::db::db_schema_structs::{
    AbstractAttribute, ConstraintType, IsGenerated, IsIdentity, IsNullable,
};

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

impl PgColumnInfo {
    pub(crate) fn constraint_map(mut self, constraint_map: &PgConstraintMap) -> Self {
        if let Some(constraint_name) = &self.constraint_name {
            let key = format!(
                "{}::{}::{}",
                self.table_name, self.column_name, constraint_name
            );
            if let Some(constraint_info) = constraint_map.pg_constraint_info.get(&key) {
                self.constraint_name = Some(constraint_info.constraint_name.clone());
                self.constraint_type = Some(constraint_info.constraint_type.clone());
                self.referenced_table = constraint_info.referenced_table.clone();
                self.referenced_column = constraint_info.referenced_column.clone();
            }
        }
        self
    }
}

impl From<PgColumnInfo> for AbstractAttribute {
    fn from(pg_column_info: PgColumnInfo) -> Self {
        let data_type = if let Some(dimensions) = pg_column_info.array_dimensions {
            if dimensions != 0 {
                format!("{}[{}]", pg_column_info.data_type, dimensions)
            } else {
                pg_column_info.data_type.clone()
            }
        } else {
            pg_column_info.data_type.clone()
        };
        AbstractAttribute {
            column_name: pg_column_info.column_name,
            data_type,
            u_type: String::new(), // Placeholder, will be filled in by enrich_adr
            is_nullable: pg_column_info
                .is_nullable
                .parse()
                .unwrap_or(IsNullable::Unknown(pg_column_info.is_nullable)),
            column_default: pg_column_info.column_default,
            character_maximum_length: pg_column_info.character_maximum_length,
            numeric_precision: pg_column_info.numeric_precision,
            numeric_scale: pg_column_info.numeric_scale,
            is_identity: pg_column_info
                .is_identity
                .parse()
                .unwrap_or(IsIdentity::Unknown(pg_column_info.is_identity)),
            identity_generation: pg_column_info.identity_generation,
            is_generated: pg_column_info
                .is_generated
                .parse()
                .unwrap_or(IsGenerated::Unknown(pg_column_info.is_generated)),
            generation_expression: pg_column_info.generation_expression,
            constraint_name: pg_column_info.constraint_name,
            constraint_type: pg_column_info
                .constraint_type
                .as_ref()
                .and_then(|s| s.parse().ok())
                .unwrap_or(ConstraintType::None),
            referenced_table: pg_column_info.referenced_table,
            referenced_column: pg_column_info.referenced_column,
            comment: pg_column_info.column_comment,
        }
    }
}

pub(crate) struct PgConstraintMap {
    pg_constraint_info: BTreeMap<String, PgConstraintInfo>,
}

impl PgConstraintMap {
    pub(crate) fn new(constraint_infos: Vec<PgConstraintInfo>) -> Self {
        let mut pg_constraint_info = BTreeMap::new();
        for constraint_info in constraint_infos {
            let constraint_name = format!(
                "{}::{}::{}",
                constraint_info.schema_name,
                constraint_info.table_name,
                constraint_info.column_name
            );
            pg_constraint_info.insert(constraint_name, constraint_info);
        }
        PgConstraintMap { pg_constraint_info }
    }
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
