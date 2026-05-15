// This module defines the intermediate database schema representation that will be
//used by the schema parser and the code generator. The AbstractDbRepr struct
// represents a database table, while the AbstractAttribute struct represents a column
// in a table.
// The DbType enum represents the supported database types, which can be extended in the future to support more databases.
use log::debug;
use std::{collections::BTreeMap, str::FromStr};

pub const ABSTRACT_DB_REPR_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct AbstractDbRepr {
    // Apply the version as string, might have to deserialize it back to a struct.
    // Furthermore as there will be differnt versions and users can
    // print out the ADR the version might help in case of debugging.
    pub version: String,
    pub tables: BTreeMap<String, AbstractTableRepr>,
    pub views: BTreeMap<String, AbstractTableRepr>,
}

#[derive(serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct AbstractTableRepr {
    pub object_type: ObjectType,
    pub table_name: String,
    pub comment: Option<String>,
    pub attributes: BTreeMap<String, AbstractAttribute>,
}

// This module defines the intermediate database attribute representation.
#[derive(serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct AbstractAttribute {
    pub column_name: String,
    pub data_type: String,
    pub is_nullable: IsNullable,
    pub column_default: Option<String>,
    pub character_maximum_length: Option<i32>,
    pub numeric_precision: Option<i32>,
    pub numeric_scale: Option<i32>,
    pub is_identity: IsIdentity,
    pub identity_generation: Option<String>,
    pub is_generated: IsGenerated,
    pub generation_expression: Option<String>,
    pub constraint_name: Option<String>,
    pub constraint_type: ConstraintType,
    pub referenced_table: Option<String>,
    pub referenced_column: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ObjectType {
    BaseTable,
    View,
    MaterializedView,
    Other,
    Unknown(String),
}

impl FromStr for ObjectType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "base table" => Ok(ObjectType::BaseTable),
            "view" => Ok(ObjectType::View),
            "materialized view" => Ok(ObjectType::MaterializedView),
            _ => {
                debug!("Invalid object type: {}", s);
                Ok(ObjectType::Unknown(s.to_string()))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub enum IsNullable {
    Yes,
    No,
    Unknown(String),
}

impl FromStr for IsNullable {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "yes" => Ok(IsNullable::Yes),
            "no" => Ok(IsNullable::No),
            _ => {
                debug!("Invalid value for is_nullable: {}", s);
                Ok(IsNullable::Unknown(s.to_string()))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub enum IsIdentity {
    Yes,
    No,
    Unknown(String),
}

impl FromStr for IsIdentity {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "yes" => Ok(IsIdentity::Yes),
            "no" => Ok(IsIdentity::No),
            _ => {
                debug!("Invalid value for is_identity: {}", s);
                Ok(IsIdentity::Unknown(s.to_string()))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub enum IsGenerated {
    Always,
    ByDefault,
    ByDefaultOnNull,
    Never,
    Unknown(String),
}

impl FromStr for IsGenerated {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "always" => Ok(IsGenerated::Always),
            "by default" => Ok(IsGenerated::ByDefault),
            "by default on null" => Ok(IsGenerated::ByDefaultOnNull),
            "never" => Ok(IsGenerated::Never),
            _ => {
                debug!("Invalid value for is_generated: {}", s);
                Ok(IsGenerated::Unknown(s.to_string()))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ConstraintType {
    PrimaryKey,
    ForeignKey,
    None,
    Unknown(String),
}

impl FromStr for ConstraintType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "primary key" => Ok(ConstraintType::PrimaryKey),
            "foreign key" => Ok(ConstraintType::ForeignKey),
            "unique" => Ok(ConstraintType::None),
            _ => {
                debug!("Invalid constraint type: {}", s);
                Ok(ConstraintType::Unknown(s.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_column_info(column_name: &str) -> AbstractAttribute {
        let attribute = AbstractAttribute {
            column_name: column_name.to_string(),
            data_type: "integer".to_string(),
            is_nullable: "NO".parse().unwrap_or(IsNullable::No),
            column_default: Some("nextval('users_id_seq'::regclass)".to_string()),
            character_maximum_length: None,
            numeric_precision: Some(32),
            numeric_scale: Some(0),
            is_identity: "NO".parse().unwrap_or(IsIdentity::No),
            identity_generation: None,
            is_generated: "NO".parse().unwrap_or(IsGenerated::Always),
            generation_expression: None,
            constraint_name: Some("users_pkey".to_string()),
            constraint_type: "PRIMARY KEY".parse().unwrap_or(ConstraintType::None),
            referenced_table: None,
            referenced_column: None,
            comment: Some("Primary key for users table".to_string()),
        };
        attribute
    }
    fn create_table_info(table_name: &str) -> AbstractTableRepr {
        let mut table_info = AbstractTableRepr {
            table_name: table_name.to_string(),
            object_type: "BASE TABLE".parse().unwrap_or(ObjectType::BaseTable),
            attributes: BTreeMap::new(),
            comment: Some("Users table".to_string()),
        };
        table_info
    }

    #[test]
    fn test_abstract_db_repr() {
        let mut table_info = create_table_info("users");
        assert_eq!(table_info.table_name, "users");
        table_info
            .attributes
            .insert("id".to_string(), create_column_info("id")); // Attempt to add a first attribute
        assert_eq!(table_info.attributes.len(), 1);
        table_info
            .attributes
            .insert("id".to_string(), create_column_info("id")); // Attempt to add a duplicate attribute
        assert_eq!(table_info.attributes.len(), 1);
        table_info
            .attributes
            .insert("name".to_string(), create_column_info("name")); // Add a new attribute
        assert_eq!(table_info.attributes.len(), 2);
        table_info
            .attributes
            .insert("name".to_string(), create_column_info("name")); // Attempt to add a duplicate attribute again
        assert_eq!(table_info.attributes.len(), 2);
    }
}
