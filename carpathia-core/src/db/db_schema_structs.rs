//! This module defines the intermediate database schema representation that will be
//! used by the schema parser and the code generator. The AbstractDbRepr (ADR) struct
//! represents a database database in a canonical model. It will be referenced as ADR or
//! Internal Representation (IR). It can be seen as a contract between the templates and carpathia.
//!
use log::debug;
use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

/// The version of the ADR - it has nothing to do with the software version of carpathia -
/// it only references to the ADR itself. Exprect for
///
/// - Mayor changes e.g. 0.1.0 to 1.0.0 changes that will break your templates which worked
///   fine under 0.1.0.
/// - Minor changes e.g. 0.1.0 to 0.2.0 will not break you themplates but allow you to enlarge them if needed.
///   A change like this will for example add new attributes to the ADR.
/// - Patch changes e.g. 0.1.0 to 0.1.1 will just fix bugs e.g. if the database constrant UNIQUE would have ben
///   given back as none, fixig it to return unique would be such a change.
pub const ABSTRACT_DB_REPR_VERSION: &str = "0,1,0";

/// Wrapping structure holding the database representation.
#[derive(serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AbstractDbRepr {
    /// The version of ADR
    pub version: String,
    /// Tables found in the database - they are always in a deterministic order
    pub tables: BTreeMap<String, AbstractTableRepr>,
    /// Views found in the database - always in a deterministic order
    pub views: BTreeMap<String, AbstractTableRepr>,
}

/// This struct represents a table-like database object. This can be a
///
/// - table
/// - view
/// - materalized view
#[derive(serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AbstractTableRepr {
    pub object_type: ObjectType,
    /// Your data types mapping go into u_imports. Again the order is deterministic.
    pub u_imports: BTreeSet<String>,
    /// The name of the database object.
    pub table_name: String,
    pub comment: Option<String>,
    /// The attributes the database object consists of.
    pub attributes: BTreeMap<String, AbstractAttribute>,
}

/// This module defines the intermediate database attribute representation.
#[derive(serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AbstractAttribute {
    pub column_name: String,
    pub data_type: String,
    pub u_type: String,
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
    Unique,
    None,
    Unknown(String),
}

impl FromStr for ConstraintType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "primary key" => Ok(ConstraintType::PrimaryKey),
            "foreign key" => Ok(ConstraintType::ForeignKey),
            "unique" => Ok(ConstraintType::Unique),
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
        AbstractAttribute {
            column_name: column_name.to_string(),
            data_type: "integer".to_string(),
            u_type: "whatever".to_string(),
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
        }
    }
    fn create_table_info(table_name: &str) -> AbstractTableRepr {
        AbstractTableRepr {
            table_name: table_name.to_string(),
            object_type: "BASE TABLE".parse().unwrap_or(ObjectType::BaseTable),
            attributes: BTreeMap::new(),
            u_imports: BTreeSet::new(),
            comment: Some("Users table".to_string()),
        }
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
