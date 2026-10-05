//! This module defines the intermediate database schema representation that will be
//! used by the schema parser and the code generator. The AbstractDbRepr (ADR) struct
//! represents a database database in a canonical model. It will be referenced as ADR or
//! Internal Representation (IR). It can be seen as a contract between the templates and carpathia.
//!
//! The version of the ADR - it has nothing to do with the software version of carpathia -
//! it only references to the ADR itself. Exprect for
//!
//! - Mayor changes e.g. 0.1.0 to 1.0.0 changes that will break your templates which worked
//!   fine under 0.1.0.
//! - Minor changes e.g. 0.1.0 to 0.2.0 will not break you themplates but allow you to enlarge them if needed.
//!   A change like this will for example add new attributes to the ADR.
//! - Patch changes e.g. 0.1.0 to 0.1.1 will just fix bugs e.g. if the database constrant UNIQUE would have ben
//!   given back as none, fixig it to return unique would be such a change.
//!
//! ```
//! use carpathia_adr::adr::abstract_db_repr::{AbstractDbRepr,
//!     AbstractTableRepr,
//!     AbstractAttribute, IsNullable, ObjectType, ConstraintType, AbstractConstraint, TableProperties,
//!     AbstractPrimaryKey, AbstractForeignKey, AbstractReferencedTable, KeyType, IsIdentity, IsGenerated};
//! use carpathia_adr::adr::enrich_adr::add_user_mapping_to_adr;
//! use std::collections::BTreeMap;
//! use serde::Serialize;
//!
//!
//! fn main() {
//!     let mut adr = AbstractDbRepr {
//!         version: "0.1.0".to_string(),
//!         tables: BTreeMap::<String, AbstractTableRepr>::new(),
//!         views: BTreeMap::<String, AbstractTableRepr>::new(),
//!     };
//!     adr.tables.insert("users".to_string(), AbstractTableRepr::new(
//!         ObjectType::BaseTable, "users".to_string(),
//!         Some("Users table".to_string())));
//!
//!     adr.tables.get_mut("users").unwrap().attributes.insert("id".to_string(),
//!         AbstractAttribute {
//!             column_name: "id".to_string(),
//!             u_column_name: "id".to_string(),
//!             data_type: "integer".to_string(),
//!             u_type: "whatever".to_string(),
//!             is_nullable: IsNullable::No,
//!             is_primary_key: true,
//!             column_default: Some("nextval('users_id_seq'::regclass)".to_string()),
//!             numeric_precision: None::<i32>,
//!             numeric_scale: None::<i32>,
//!             character_maximum_length: None::<i32>,
//!             constraints: BTreeMap::new(),
//!             comment: Some("Primary key for users table".to_string()),
//!         });
//!     // To keep short be brave and unwrap. In production code you should handle the error properly.
//!     
//!     let adr_json = serde_json::to_string(&adr).unwrap();
//!     /*assert_eq!(adr_json, serde_json::json!({
//!   "version": "0.1.0",
//!   "tables": {
//!     "users": {
//!       "object_type": "BaseTable",
//!       "u_imports": [],
//!       "table_name": "users",
//!       "u_table_name": "users",
//!       "table_properties": [],
//!       "comment": "Users table",
//!       "attributes": {
//!         "id": {
//!           "column_name": "id",
//!           "u_column_name": "id",
//!           "data_type": "integer",
//!           "u_type": "whatever",
//!           "is_nullable": "No",
//!           "column_default": "nextval('users_id_seq'::regclass)",
//!           "is_primary_key": true,
//!           "character_maximum_length": null,
//!           "numeric_precision": null,
//!           "numeric_scale": null,
//!           "constraints": {},
//!           "comment": "Primary key for users table"
//!         }
//!       }
//!     }
//!   },
//!   "views": {}
//! }));*/
//! }
//! ```
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Add;

/// This struct represents a key in the database. It can be a primary key or a foreign key.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AbstractKey<K> {
    /// The columns that make up the key. This is a set of AbstractReferencedTable, which contains information about the database attribute and its referenced table and attribute.
    pub columns: BTreeSet<AbstractReferencedTable>,
    #[serde(skip)]
    pub _marker: PhantomData<K>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrimaryKeySemantics;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ForeignKeySemantics;

/// This type alias represents a primary key in the database. It is an AbstractKey with PrimaryKeySemantics.
pub type AbstractPrimaryKey = AbstractKey<PrimaryKeySemantics>;

/// This type alias represents a foreign key in the database. It is an AbstractKey with ForeignKeySemantics.
pub type AbstractForeignKey = AbstractKey<ForeignKeySemantics>;

/// Version of the ADR is the same as the crate-version.
pub const ABSTRACT_DB_REPR_VERSION: &str = env!("CARGO_PKG_VERSION");
const KEY_TYPE_CHANGE_DEFAULT: &KeyTypeChange = &KeyTypeChange {
    attribute_name: vec![],
    key_type: KeyType::SingleColumn,
};

/// This struct represents an internal view of the database. It consists of
///
/// - table
/// - view
/// - materalized view
///
/// Keep in mind any attribute prefixed with u_ is handled by (enrich_adr)[crate::adr::enrich_adr::add_user_mapping_to_adr].
/// So do not attempt to fill in the u_-attibutes when building the ADR.
/// This representation is core-internal only. It is not the structure, templates are programmed against. The structure
/// passed to the templates is definde in (tera_conversion)[crate::adr::tera_conversion].
/// The internal ADR can be viewed using the cli-flag `--print-internal-schema`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AbstractDbRepr {
    /// The version of ADR
    pub version: String,
    /// Tables found in the database - they are always in a deterministic order
    pub tables: BTreeMap<String, AbstractTableRepr>,
    /// Views found in the database - always in a deterministic order
    pub views: BTreeMap<String, AbstractTableRepr>,
}

/// Internal representation of a database table, view or materialized view. It is used to represent the database schema in a canonical model.
/// When populating this struct, do not fill in the u_-attributes. They are filled in by (enrich_adr)[crate::adr::enrich_adr::add_user_mapping_to_adr].
/// To make enrich_adr work you need to populate table_properties with the constraints of this specific table attribute.
///
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AbstractTableRepr {
    pub object_type: ObjectType,
    /// Your data types mapping go into u_imports. Again the order is deterministic.
    pub u_imports: BTreeSet<String>,
    /// The name of the database object.
    pub table_name: String,
    /// language safe name of the database object. This is the name you will use in your templates to reference the database object.
    /// filled by (enrich_adr)[crate::adr::enrich_adr::add_user_mapping_to_adr].
    pub u_table_name: String,
    /// The properties of the database object. This is a set of TableProperties, which contains information about the database object and its properties.
    /// The information is retrieved by enrich_adr destilling the constraint informations from the attributes.
    pub table_properties: BTreeSet<TableProperties>,
    pub comment: Option<String>,
    /// The attributes the database object consists of.
    pub attributes: BTreeMap<String, AbstractAttribute>,
}

impl AbstractTableRepr {
    pub fn new(object_type: ObjectType, table_name: String, comment: Option<String>) -> Self {
        Self {
            object_type,
            u_imports: BTreeSet::new(),
            table_name: table_name.clone(),
            u_table_name: table_name,
            table_properties: BTreeSet::new(),
            comment,
            attributes: BTreeMap::new(),
        }
    }
}

/// This module defines the intermediate database attribute representation.
/// An attribute is a column in a table or view. It is used to represent the database schema in a canonical model.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AbstractAttribute {
    /// The name of the database table attribute.
    pub column_name: String,
    /// language safe name of the database attribute. This is the name you will use in your templates to reference the database attribute.
    pub u_column_name: String,
    /// The data type of the database attribute. This is the data type as it is defined in the database.
    pub data_type: String,
    /// The data type as you definded it in your data types mapping. E.g. varchar(16) becomes string - define it as needed.
    pub u_type: String,
    pub is_nullable: IsNullable,
    /// The default value of the database attribute. This is the default value as it is defined in the database.
    pub column_default: Option<String>,
    /// Whether the database attribute is a primary key. This is determined by the constraints of the database attribute.
    pub is_primary_key: bool,
    /// The maximum length of the database attribute. This is the maximum length as it is defined in the database. Might be None for some database types.
    pub character_maximum_length: Option<i32>,
    pub numeric_precision: Option<i32>,
    pub numeric_scale: Option<i32>,
    /// The constraints of the database attribute. This is a set of ConstraintType, which contains information about the database attribute and its constraints.
    pub constraints: BTreeMap<ConstraintType, AbstractConstraint>,
    pub comment: Option<String>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]

/// This struct represents a constraint of a database attribute.
/// This values are then used to populate the table_properties.
/// A constraint is a rule enforced on the data of this attribute.
/// It can be anything definde by (ConsraintType)[crate::adr::abstract_db_repr::ConstraintType] and is used to determine the properties of the database attribute.
pub struct AbstractConstraint {
    /// The name of the constraint - e.g. users_pkey.
    pub constraint_name: String,
    /// The value of the constraint - e.g. PRIMARY KEY (actor_id), Note this is going to
    /// vary between different from database types.
    pub constraint_value: String,
    /// The name of the referenced schema - e.g. pagila. The attribute will be skipped if None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_schema_name: Option<String>,
    /// The name of the referenced table - e.g. users. The attribute will be skipped if None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_table: Option<String>,
    /// The language-safe name of the referenced table - e.g. Users. The attribute will be skipped if None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_referenced_table: Option<String>,
    /// The name of the referenced column - e.g. actor_id. The attribute will be skipped if None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_column: Option<String>,
    /// The language-safe name of the referenced column - e.g. ActorId. The attribute will be skipped if None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_referenced_column: Option<String>,
}

/// Represents the global properties of a table or view. These properties are derived from the constraints of the table or view and are used to determine the capabilities of the table or view.
/// Use (enrich_adr)[crate::adr::enrich_adr::add_user_mapping_to_adr] to populate the table_properties of a table or view.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Ord, PartialOrd)]
pub enum TableProperties {
    PrimaryKey(AbstractPrimaryKey),
    Autogenerated,
    ForeignKey(AbstractForeignKey),
    Selectable,
    Insertable,
    Deletable,
    Updatable,
    NotNull,
    Nullable,
    Default,
}

/// Type of the key - a MultiColumn key has at least two columns sharing the same constraint name. A SingleColumn key has only one column with the same constraint name.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub enum KeyType {
    SingleColumn,
    MultiColumn,
}

/// Must implent PartialEq, Eq, PartialOrd, Ord to be able to perform
/// addition under the codition equal is any AbstractForeignKey as a
/// type and not taking its content in account.
impl<K> PartialEq for AbstractKey<K> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<K> Eq for AbstractKey<K> {}

impl<K> PartialOrd for AbstractKey<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K> Ord for AbstractKey<K> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

struct KeyTypeChange {
    attribute_name: Vec<String>,
    key_type: KeyType,
}

/// Implement the Add trait for AbstractKey to allow combining two AbstractKey instances.
/// The addition operation merges the columns of both keys, updating the key type to MultiColumn if any constraint name is shared between the two keys.
/// The content of columns is deliberately ignored in the comparison of AbstractKey instances, as the equality and ordering are based solely on the type parameter K.
impl<K> Add for AbstractKey<K> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        //let other_columns = other.columns;
        let mut other_columns: BTreeSet<AbstractReferencedTable> = BTreeSet::new();
        let mut changed_key_types: HashMap<String, KeyTypeChange> = HashMap::new();
        for column in self.columns.iter().chain(other.columns.iter()) {
            if let Some(existing) = changed_key_types.get_mut(&column.constraint_name) {
                if !existing.attribute_name.contains(&column.column) {
                    existing.attribute_name.push(column.column.clone());
                    existing.key_type = KeyType::MultiColumn;
                } else if existing.attribute_name.contains(&column.column) {
                    // If the column is already present, we don't need to change the key type
                    // Done to prevent overwriting of the key type, which
                    // will result in losing the column names subsumed under this particlular
                    // constraint name. If removed this will result in a loss of information
                    // and will result in a wrong representation of the database schema.
                    // No operation, just a placeholder to indicate no change
                }
            } else {
                changed_key_types.insert(
                    column.constraint_name.clone(),
                    KeyTypeChange {
                        attribute_name: vec![column.column.clone()],
                        key_type: KeyType::SingleColumn,
                    },
                );
            }
        }
        for column in self.columns.into_iter().chain(other.columns) {
            let key_type = changed_key_types
                .get(&column.constraint_name)
                .unwrap_or(KEY_TYPE_CHANGE_DEFAULT);
            other_columns.insert(AbstractReferencedTable {
                constraint_name: column.constraint_name,
                key_type: key_type.key_type.clone(),
                column: column.column,
                u_column: column.u_column,
                referenced_table: column.referenced_table,
                u_referenced_table: column.u_referenced_table,
                referenced_column: column.referenced_column,
                u_referenced_column: column.u_referenced_column,
            });
        }

        Self {
            columns: other_columns,
            _marker: PhantomData,
        }
    }
}

/// This struct represents a referenced table in the database. It contains information about the constraint name, key type, column name, and referenced table and column names.
/// You don't need to fill in the u_-attributes when building the ADR. They are filled in by (enrich_adr)[crate::adr::enrich_adr::add_user_mapping_to_adr].
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub struct AbstractReferencedTable {
    pub constraint_name: String,
    pub key_type: KeyType,
    pub column: String,
    pub u_column: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_referenced_table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_column: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_referenced_column: Option<String>,
}

/// Object the database introspection found.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub enum ObjectType {
    BaseTable,
    PartitionedTable,
    View,
    MaterializedView,
    Other,
    /// This is used to represent an object type that is not recognized by the current version of the ADR. It allows for forward compatibility with future database object types.
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum IsNullable {
    Yes,
    No,
    /// This is used to represent a nullable status that is not recognized by the current version of the ADR. It allows for forward compatibility with future database nullable statuses.
    Unknown(String),
}

/// An identity column is a primary key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum IsIdentity {
    Yes,
    No,
    /// This is used to represent an identity status that is not recognized by the current version of the ADR. It allows for forward compatibility with future database identity statuses.
    Unknown(String),
}

/// A generated column is a column that is automatically generated by the database. It can be generated always, by default, by default on null, or never.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum IsGenerated {
    Always,
    ByDefault,
    ByDefaultOnNull,
    Never,
    /// This is used to represent a generated status that is not recognized by the current version of the ADR. It allows for forward compatibility with future database generated statuses.
    Unknown(String),
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub enum ConstraintType {
    PrimaryKey,
    ForeignKey,
    Unique,
    Check,
    Exclusion,
    NotNull,
    ConstraintTrigger,
    None,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_column_info(column_name: &str) -> AbstractAttribute {
        AbstractAttribute {
            column_name: column_name.to_string(),
            u_column_name: column_name.to_string(),
            data_type: "integer".to_string(),
            u_type: "whatever".to_string(),
            is_nullable: IsNullable::No,
            is_primary_key: false,
            column_default: Some("nextval('users_id_seq'::regclass)".to_string()),
            numeric_precision: None,
            numeric_scale: None,
            character_maximum_length: None,
            constraints: {
                let mut constraints = BTreeMap::new();
                constraints.insert(
                    ConstraintType::PrimaryKey,
                    AbstractConstraint {
                        constraint_name: "users_pkey".to_string(),
                        constraint_value: "".to_string(),
                        referenced_schema_name: None,
                        u_referenced_column: None,
                        referenced_table: None,
                        u_referenced_table: None,
                        referenced_column: None,
                    },
                );
                constraints
            },
            comment: Some("Primary key for users table".to_string()),
        }
    }
    fn create_table_info(table_name: &str) -> AbstractTableRepr {
        AbstractTableRepr {
            table_name: table_name.to_string(),
            u_table_name: table_name.to_string(),
            object_type: ObjectType::BaseTable,
            attributes: BTreeMap::new(),
            u_imports: BTreeSet::new(),
            table_properties: BTreeSet::new(),
            comment: Some("Users table".to_string()),
        }
    }
    #[test]
    fn test_abstract_foreign_key_addition() {
        let column1 = AbstractForeignKey {
            columns: BTreeSet::from([AbstractReferencedTable {
                constraint_name: "fk1".to_string(),
                key_type: KeyType::SingleColumn,
                column: "column1".to_string(),
                u_column: "u_column1".to_string(),
                referenced_table: Some("ref_table1".to_string()),
                u_referenced_table: None,
                referenced_column: Some("ref_column1".to_string()),
                u_referenced_column: None,
            }]),
            _marker: PhantomData,
        };
        assert_eq!(column1.columns.len(), 1);
        assert_eq!(
            column1.columns.iter().next().unwrap().constraint_name,
            "fk1"
        );
        assert_eq!(
            column1.columns.iter().next().unwrap().key_type,
            KeyType::SingleColumn
        );
        // Adding the same foreign key again - it should still be a single column foreign key
        let column1_duplicate = column1.clone();
        let combined_fk = column1.clone() + column1_duplicate;
        assert_eq!(
            combined_fk.columns.len(),
            1,
            "Expected 1 column in combined foreign key, got {:?}",
            combined_fk
        );
        assert_eq!(
            combined_fk.columns.iter().next().unwrap().key_type,
            KeyType::SingleColumn,
            "Expected key_type to be SingleColumn {:?}",
            combined_fk
        );

        let column2 = AbstractForeignKey {
            columns: BTreeSet::from([AbstractReferencedTable {
                constraint_name: "fk1".to_string(),
                key_type: KeyType::SingleColumn,
                column: "column2".to_string(),
                u_column: "u_column2".to_string(),
                referenced_table: Some("ref_table2".to_string()),
                u_referenced_table: None,
                referenced_column: Some("ref_column2".to_string()),
                u_referenced_column: None,
            }]),
            _marker: PhantomData,
        };

        let combined_fk = column1 + column2;

        assert_eq!(
            combined_fk.columns.len(),
            2,
            "Expected 2 columns in combined foreign key, got {:?}",
            combined_fk
        );
        assert_eq!(
            combined_fk
                .columns
                .iter()
                .find(|c| c.column == "column1")
                .unwrap()
                .key_type,
            KeyType::MultiColumn,
            "Expected column1 to have key_type MultiColumn {:?}",
            combined_fk
        );
        assert_eq!(
            combined_fk
                .columns
                .iter()
                .find(|c| c.column == "column2")
                .unwrap()
                .key_type,
            KeyType::MultiColumn,
            "Expected column2 to have key_type MultiColumn {:?}",
            combined_fk
        );

        let column3 = AbstractForeignKey {
            columns: BTreeSet::from([AbstractReferencedTable {
                constraint_name: "fk2".to_string(),
                key_type: KeyType::SingleColumn,
                column: "column3".to_string(),
                u_column: "u_column3".to_string(),
                referenced_table: Some("ref_table3".to_string()),
                u_referenced_table: None,
                referenced_column: Some("ref_column3".to_string()),
                u_referenced_column: None,
            }]),
            _marker: PhantomData,
        };

        let combined_fk2 = combined_fk + column3;
        assert_eq!(
            combined_fk2.columns.len(),
            3,
            "Expected 3 columns in combined foreign key, got {:?}",
            combined_fk2
        );
        assert_eq!(
            combined_fk2
                .columns
                .iter()
                .find(|c| c.column == "column3")
                .unwrap()
                .key_type,
            KeyType::SingleColumn,
            "Expected column3 to have key_type SingleColumn {:?}",
            combined_fk2
        );
        assert_eq!(
            combined_fk2
                .columns
                .iter()
                .find(|c| c.column == "column1")
                .unwrap()
                .key_type,
            KeyType::MultiColumn,
            "Expected column1 to have key_type MultiColumn {:?}",
            combined_fk2
        );
        assert_eq!(
            combined_fk2
                .columns
                .iter()
                .find(|c| c.column == "column2")
                .unwrap()
                .key_type,
            KeyType::MultiColumn,
            "Expected column2 to have key_type MultiColumn {:?}",
            combined_fk2
        );
    }

    #[test]
    fn test_abstract_primary_key_addition() {
        let column1 = AbstractPrimaryKey {
            columns: BTreeSet::from([AbstractReferencedTable {
                constraint_name: "pk1".to_string(),
                key_type: KeyType::SingleColumn,
                column: "column1".to_string(),
                u_column: "u_column1".to_string(),
                referenced_table: None,
                u_referenced_table: None,
                referenced_column: None,
                u_referenced_column: None,
            }]),
            _marker: PhantomData,
        };
        assert_eq!(column1.columns.len(), 1);
        assert_eq!(
            column1.columns.iter().next().unwrap().constraint_name,
            "pk1"
        );
        assert_eq!(
            column1.columns.iter().next().unwrap().key_type,
            KeyType::SingleColumn
        );
        // Adding the same foreign key again - it should still be a single column foreign key
        let column1_duplicate = column1.clone();
        let combined_pk = column1.clone() + column1_duplicate;
        assert_eq!(
            combined_pk.columns.len(),
            1,
            "Expected 1 column in combined foreign key, got {:?}",
            combined_pk
        );
        assert_eq!(
            combined_pk.columns.iter().next().unwrap().key_type,
            KeyType::SingleColumn,
            "Expected key_type to be SingleColumn {:?}",
            combined_pk
        );

        let column2 = AbstractPrimaryKey {
            columns: BTreeSet::from([AbstractReferencedTable {
                constraint_name: "pk1".to_string(),
                key_type: KeyType::SingleColumn,
                column: "column2".to_string(),
                u_column: "u_column2".to_string(),
                referenced_table: Some("ref_table2".to_string()),
                u_referenced_table: None,
                referenced_column: Some("ref_column2".to_string()),
                u_referenced_column: None,
            }]),
            _marker: PhantomData,
        };

        let combined_pk = column1 + column2;

        assert_eq!(
            combined_pk.columns.len(),
            2,
            "Expected 2 columns in combined foreign key, got {:?}",
            combined_pk
        );
        assert_eq!(
            combined_pk
                .columns
                .iter()
                .find(|c| c.column == "column1")
                .unwrap()
                .key_type,
            KeyType::MultiColumn,
            "Expected column1 to have key_type MultiColumn {:?}",
            combined_pk
        );
        assert_eq!(
            combined_pk
                .columns
                .iter()
                .find(|c| c.column == "column2")
                .unwrap()
                .key_type,
            KeyType::MultiColumn,
            "Expected column2 to have key_type MultiColumn {:?}",
            combined_pk
        );
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

    #[test]
    fn test_is_nullable_from_str() {
        let yes: IsNullable = IsNullable::Yes;
        assert_eq!(yes, IsNullable::Yes);
        let unknown: IsNullable = IsNullable::Unknown("MAYBE".to_string());
        assert_eq!(unknown, IsNullable::Unknown("MAYBE".to_string()));
    }

    #[test]
    fn test_is_identity_from_str() {
        let yes: IsIdentity = IsIdentity::Yes;
        assert_eq!(yes, IsIdentity::Yes);
        let unknown: IsIdentity = IsIdentity::Unknown("MAYBE".to_string());
        assert_eq!(unknown, IsIdentity::Unknown("MAYBE".to_string()));
    }
}
