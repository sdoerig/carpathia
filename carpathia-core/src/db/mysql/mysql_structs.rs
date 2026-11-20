use crate::db::mysql::mysql_enums::{MySqlIsNullable, MySqlObjectType};
use std::collections::{BTreeMap, BTreeSet};

use carpathia_adr::adr::abstract_db_repr::{
    AbstractAttribute, AbstractConstraint, AbstractTableRepr, ConstraintType,
};

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

impl From<&MySqlInfo> for AbstractTableRepr {
    fn from(info: &MySqlInfo) -> Self {
        let abstract_attribute: AbstractAttribute = info.into();
        AbstractTableRepr {
            object_type: info
                .table_type
                .parse()
                .unwrap_or(MySqlObjectType::Unknown(info.table_type.clone()))
                .into(),
            table_properties: BTreeSet::new(),
            u_imports: BTreeSet::new(),
            table_name: abstract_attribute.column_name.clone(),
            u_table_name: String::new(),
            attributes: BTreeMap::new(),
            comment: None,
        }
    }
}

impl From<MySqlInfo> for AbstractAttribute {
    fn from(info: MySqlInfo) -> Self {
        let is_nullable: MySqlIsNullable = info
            .is_nullable
            .parse()
            .unwrap_or(MySqlIsNullable::Unknown(info.is_nullable.clone()));
        let mut constraints: BTreeMap<ConstraintType, AbstractConstraint> = BTreeMap::new();
        if info.constraint_type.as_deref() == Some("PRIMARY KEY") {
            constraints.insert(
                ConstraintType::PrimaryKey,
                AbstractConstraint {
                    constraint_name: info.constraint_name.clone().unwrap_or_default(),
                    constraint_value: info.constraint_type.clone().unwrap_or_default(),
                    referenced_schema_name: None,
                    referenced_table: None,
                    u_referenced_table: None,
                    referenced_column: None,
                    u_referenced_column: None,
                },
            );
        } else if info.constraint_type.as_deref() == Some("FOREIGN KEY") {
            constraints.insert(
                ConstraintType::ForeignKey,
                AbstractConstraint {
                    constraint_name: info.constraint_name.clone().unwrap_or_default(),
                    constraint_value: info.constraint_type.clone().unwrap_or_default(),
                    referenced_schema_name: info.referenced_table_schema.clone(),
                    referenced_table: info.referenced_table_name.clone(),
                    u_referenced_table: None,
                    referenced_column: info.referenced_column_name.clone(),
                    u_referenced_column: None,
                },
            );
        } else if info.constraint_type.as_deref() == Some("UNIQUE") {
            constraints.insert(
                ConstraintType::Unique,
                AbstractConstraint {
                    constraint_name: info.constraint_name.clone().unwrap_or_default(),
                    constraint_value: info.constraint_type.clone().unwrap_or_default(),
                    referenced_schema_name: None,
                    referenced_table: None,
                    u_referenced_table: None,
                    referenced_column: None,
                    u_referenced_column: None,
                },
            );
        } else if info.constraint_type.as_deref() == Some("CHECK") {
            constraints.insert(
                ConstraintType::Check,
                AbstractConstraint {
                    constraint_name: info.constraint_name.clone().unwrap_or_default(),
                    constraint_value: info.constraint_type.clone().unwrap_or_default(),
                    referenced_schema_name: None,
                    referenced_table: None,
                    u_referenced_table: None,
                    referenced_column: None,
                    u_referenced_column: None,
                },
            );
        } else if info.is_nullable == "is_not_nullable" {
            constraints.insert(
                ConstraintType::NotNull,
                AbstractConstraint {
                    constraint_name: info.constraint_name.clone().unwrap_or_default(),
                    constraint_value: info.constraint_type.clone().unwrap_or_default(),
                    referenced_schema_name: None,
                    referenced_table: None,
                    u_referenced_table: None,
                    referenced_column: None,
                    u_referenced_column: None,
                },
            );
        }

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
            constraints,
            comment: None,
        }
    }
}

impl From<&MySqlInfo> for AbstractAttribute {
    fn from(info: &MySqlInfo) -> Self {
        info.clone().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::mysql::mysql_enums::MySqlIsNullable;

    #[test]
    fn test_mysql_info_to_abstract_attribute() {
        let mysql_info = MySqlInfo {
            database_name: "test_db".to_string(),
            table_schema: "public".to_string(),
            table_name: "test_table".to_string(),
            column_name: "test_column".to_string(),
            column_position: 1,
            data_type: "VARCHAR".to_string(),
            column_type: "VARCHAR(255)".to_string(),
            character_maximum_length: Some(255),
            numeric_precision: None,
            numeric_scale: None,
            datetime_precision: None,
            is_nullable: "is_not_nullable".to_string(),
            column_default: Some("default_value".to_string()),
            extra: "".to_string(),
            generation_expression: None,
            column_comment: Some("This is a test column".to_string()),
            table_type: "BASE TABLE".to_string(),
            engine: Some("InnoDB".to_string()),
            table_collation: Some("utf8_general_ci".to_string()),
            table_comment: Some("This is a test table".to_string()),
            is_identity: "NO".to_string(),
            constraint_name: Some("PRIMARY".to_string()),
            constraint_type: Some("PRIMARY KEY".to_string()),
            constraint_column_position: Some(1),
            position_in_unique_constraint: None,
            referenced_table_schema: None,
            referenced_table_name: None,
            referenced_column_name: None,
            update_rule: None,
            delete_rule: None,
            index_name: None,
            seq_in_index: None,
            non_unique: None,
            index_type: None,
            check_clause: None,
            constraints: BTreeMap::new(),
        };
        let abstract_attribute: AbstractAttribute = mysql_info.into();
        assert_eq!(abstract_attribute.column_name, "test_column");
        assert_eq!(abstract_attribute.data_type, "VARCHAR");
        assert_eq!(abstract_attribute.is_nullable, MySqlIsNullable::No.into());
        assert!(abstract_attribute.is_primary_key);
        assert_eq!(abstract_attribute.character_maximum_length, Some(255));
        assert_eq!(abstract_attribute.constraints.len(), 1);
        assert!(
            abstract_attribute
                .constraints
                .contains_key(&ConstraintType::PrimaryKey)
        );
    }
}
