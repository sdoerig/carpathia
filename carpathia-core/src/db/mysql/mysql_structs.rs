use crate::db::mysql::mysql_enums::MySqlIsNullable;
use std::collections::BTreeMap;

use carpathia_adr::adr::abstract_db_repr::{AbstractAttribute, AbstractConstraint, ConstraintType};

#[derive(sqlx::FromRow, serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct MySqlInfo {
    pub database_name: String,
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
    pub column_position: u64,
    pub data_type: String,
    pub column_type: String,
    pub character_maximum_length: Option<u64>,
    pub numeric_precision: Option<u64>,
    pub numeric_scale: Option<u64>,
    pub datetime_precision: Option<u64>,
    pub is_nullable: String,
    pub column_default: Option<String>,
    pub extra: String,
    pub generation_expression: Option<String>,
    pub column_comment: Option<String>,
    pub table_type: String,
    pub engine: Option<String>,
    pub table_collation: Option<String>,
    pub table_comment: Option<String>,
    pub is_identity: String,
    pub constraint_name: Option<String>,
    pub constraint_type: Option<String>,
    pub constraint_column_position: Option<i32>,
    pub position_in_unique_constraint: Option<i32>,
    pub referenced_table_schema: Option<String>,
    pub referenced_table_name: Option<String>,
    pub referenced_column_name: Option<String>,
    pub update_rule: Option<String>,
    pub delete_rule: Option<String>,
    pub index_name: Option<String>,
    pub seq_in_index: Option<u32>,
    pub non_unique: Option<i32>,
    pub index_type: Option<String>,
    pub check_clause: Option<String>,

    #[sqlx(skip)]
    pub constraints: BTreeMap<ConstraintType, AbstractConstraint>,
}

impl From<MySqlInfo> for AbstractAttribute {
    fn from(info: MySqlInfo) -> Self {
        let is_nullable: MySqlIsNullable = info
            .is_nullable
            .parse()
            .unwrap_or(MySqlIsNullable::Unknown(info.is_nullable.clone()));
        AbstractAttribute {
            column_name: info.column_name,
            u_column_name: String::new(),
            data_type: info.data_type,
            u_type: String::new(),
            is_nullable: is_nullable.into(),
            column_default: info.column_default,
            is_primary_key: info.constraint_type.as_deref() == Some("PRIMARY KEY"),
            character_maximum_length: info.character_maximum_length.map(|len| len as i64),
            numeric_precision: info.numeric_precision.map(|prec| prec as i64),
            numeric_scale: info.numeric_scale.map(|scale| scale as i64),
            constraints: BTreeMap::new(),
            comment: None,
        }
    }
}
