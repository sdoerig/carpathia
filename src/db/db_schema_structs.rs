// This module defines the intermediate database schema representation that will be
//used by the schema parser and the code generator. The AbstractDbRepr struct
// represents a database table, while the AbstractAttribute struct represents a column
// in a table.
//The DbType enum represents the supported database types, which can be extended in the future to support more databases.
#[derive(sqlx::FromRow, serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct AbstractDbRepr {
    pub table_name: String,
    pub attributes: Vec<AbstractAttribute>,
}

impl AbstractDbRepr {
    pub(crate) fn unique_push(&mut self, attribute: AbstractAttribute) {
        // Only adding unique attributes to the list of attributes for a table.
        // This is important to avoid duplicates in the generated code.
        if !self.attributes.contains(&attribute) {
            self.attributes.push(attribute);
        }
    }
}

// This module defines the intermediate database attribute representation.
#[derive(sqlx::FromRow, serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct AbstractAttribute {
    pub column_name: String,
    pub data_type: String,
    pub is_nullable: String,
    pub column_default: Option<String>,
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
}
// This enum represents the supported database types. Currently, only PostgreSQL is supported, but we can easily add support for MySQL and SQLite in the future by adding new variants to this enum and implementing the necessary logic in the database querier and schema parser.
pub(crate) enum DbType {
    Postgres,
    #[allow(dead_code)]
    MySql, // Future support for MySQL
    #[allow(dead_code)]
    Sqlite, // Future support for SQLite
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_column_info(column_name: &str) -> AbstractAttribute {
        let attribute = AbstractAttribute {
            column_name: column_name.to_string(),
            data_type: "integer".to_string(),
            is_nullable: "NO".to_string(),
            column_default: Some("nextval('users_id_seq'::regclass)".to_string()),
            character_maximum_length: None,
            numeric_precision: Some(32),
            numeric_scale: Some(0),
            is_identity: "NO".to_string(),
            identity_generation: None,
            is_generated: "NO".to_string(),
            generation_expression: None,
            constraint_name: Some("users_pkey".to_string()),
            constraint_type: Some("PRIMARY KEY".to_string()),
            referenced_table: None,
            referenced_column: None,
        };
        attribute
    }
    fn create_table_info(table_name: &str) -> AbstractDbRepr {
        let mut table_info = AbstractDbRepr {
            table_name: table_name.to_string(),
            attributes: Vec::new(),
        };
        table_info
    }

    #[test]
    fn test_abstract_db_repr() {
        let mut table_info = create_table_info("users");
        assert_eq!(table_info.table_name, "users");
        table_info.unique_push(create_column_info("id")); // Attempt to add a first attribute
        assert_eq!(table_info.attributes.len(), 1);
        table_info.unique_push(create_column_info("id")); // Attempt to add a duplicate attribute
        assert_eq!(table_info.attributes.len(), 1);
        table_info.unique_push(create_column_info("name")); // Add a new attribute
        assert_eq!(table_info.attributes.len(), 2);
        table_info.unique_push(create_column_info("name")); // Attempt to add a duplicate attribute again
        assert_eq!(table_info.attributes.len(), 2);
    }
}
