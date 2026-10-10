//! Example for basic usage of the ADR. The ADR is built manually - decide by yourself if you want to do it this way or use the builder API.
//! In my imlementation for PostgreSQL, I have implemtend the From trait. To me constructing the ADR manually is like this
//! is bothersome, but there are many ways doing this.
//! - Bulder API (see `carpathia-adr/src/adr/abstract_db_repr_builder.rs`)
//! - From trait (see (PostgreSQL implementation)[https://github.com/sdoerig/carpathia/blob/main/carpathia-core/src/db/postgres/postgresql_structs.rs]
use carpathia_adr::adr::abstract_db_repr::{
    AbstractAttribute, AbstractConstraint, AbstractDbRepr, AbstractTableRepr, ConstraintType,
    IsNullable, ObjectType,
};
use carpathia_adr::adr::enrich_adr::add_user_mapping_to_adr;
use carpathia_adr::adr::tera_conversion::AdrTemplateData;
use carpathia_adr::db_type::db_to_user_type_structs::{TypeMapping, Types};
use std::collections::BTreeMap;

const ADR_JSON: &str = r#"{
  "version": "0.1.0",
  "tables": {
    "users": {
      "object_type": "BaseTable",
      "u_imports": [
        "use crate::types::MyInteger"
      ],
      "table_name": "users",
      "u_table_name": "My_Code_Safe_User",
      "table_properties": [
        {
          "PrimaryKey": {
            "columns": [
              {
                "constraint_name": "users_pkey",
                "key_type": "SingleColumn",
                "column": "id",
                "u_column": "My_Code_Safe_Id"
              }
            ]
          }
        },
        {
          "ForeignKey": {
            "columns": [
              {
                "constraint_name": "email_fkey",
                "key_type": "SingleColumn",
                "column": "email",
                "u_column": "email",
                "referenced_table": "ldap_users",
                "u_referenced_table": "ldap_users",
                "referenced_column": "email",
                "u_referenced_column": "email"
              }
            ]
          }
        }
      ],
      "comment": "Users table",
      "attributes": {
        "email": {
          "column_name": "email",
          "u_column_name": "email",
          "data_type": "text",
          "u_type": "text",
          "is_nullable": "Yes",
          "column_default": null,
          "is_primary_key": false,
          "character_maximum_length": null,
          "numeric_precision": null,
          "numeric_scale": null,
          "constraints": {
            "ForeignKey": {
              "constraint_name": "email_fkey",
              "constraint_value": "FOREIGN KEY (email) REFERENCES ldap_users(email)",
              "referenced_table": "ldap_users",
              "u_referenced_table": "ldap_users",
              "referenced_column": "email",
              "u_referenced_column": "email"
            }
          },
          "comment": "Email of the user"
        },
        "id": {
          "column_name": "id",
          "u_column_name": "My_Code_Safe_Id",
          "data_type": "integer",
          "u_type": "MyInteger",
          "is_nullable": "No",
          "column_default": "nextval('users_id_seq'::regclass)",
          "is_primary_key": true,
          "character_maximum_length": null,
          "numeric_precision": null,
          "numeric_scale": null,
          "constraints": {
            "PrimaryKey": {
              "constraint_name": "users_pkey",
              "constraint_value": "PRIMARY KEY (id)"
            }
          },
          "comment": "Primary key for users table"
        },
        "name": {
          "column_name": "name",
          "u_column_name": "name",
          "data_type": "text",
          "u_type": "text",
          "is_nullable": "Yes",
          "column_default": null,
          "is_primary_key": false,
          "character_maximum_length": null,
          "numeric_precision": null,
          "numeric_scale": null,
          "constraints": {},
          "comment": "Name of the user"
        }
      }
    }
  },
  "views": {}
}"#;

fn build_default_type_mapping() -> Types {
    let mut db_to_code_names_mapping: BTreeMap<String, String> = BTreeMap::new();
    db_to_code_names_mapping.insert("users".to_string(), "My_Code_Safe_User".to_string());
    db_to_code_names_mapping.insert("id".to_string(), "My_Code_Safe_Id".to_string());
    let mut types = Types::new();
    types.type_mapping.insert(
        "integer".to_string(),
        TypeMapping {
            u_import: Some("use crate::types::MyInteger".to_string()),
            u_type: "MyInteger".to_string(),
        },
    );
    types.db_to_code_names_mapping = db_to_code_names_mapping;
    types
}

fn add_attribute(
    table: &mut AbstractTableRepr,
    column_name: &str,
    data_type: &str,
    is_nullable: IsNullable,
    column_default: Option<&str>,
    comment: Option<&str>,
) {
    let attribute = AbstractAttribute {
        column_name: column_name.to_string(),
        u_column_name: column_name.to_string(),
        data_type: data_type.to_string(),
        u_type: data_type.to_string(),
        is_nullable,
        is_primary_key: false,
        column_default: column_default.map(|s| s.to_string()),
        numeric_precision: None,
        numeric_scale: None,
        character_maximum_length: None,
        constraints: BTreeMap::new(),
        comment: comment.map(|s| s.to_string()),
    };
    table.attributes.insert(column_name.to_string(), attribute);
}

fn add_primary_key(table: &mut AbstractTableRepr, column_name: &str, constraint_name: &str) {
    if let Some(attr) = table.attributes.get_mut(column_name) {
        attr.is_primary_key = true;
        attr.constraints.insert(
            ConstraintType::PrimaryKey,
            AbstractConstraint {
                constraint_name: constraint_name.to_string(),
                constraint_value: format!("PRIMARY KEY ({})", column_name),
                referenced_schema_name: None,
                referenced_table: None,
                u_referenced_table: None,
                referenced_column: None,
                u_referenced_column: None,
            },
        );
    }
}

fn add_foreign_key(
    table: &mut AbstractTableRepr,
    column_name: &str,
    constraint_name: &str,
    referenced_table: &str,
    referenced_column: &str,
) {
    if let Some(attr) = table.attributes.get_mut(column_name) {
        attr.constraints.insert(
            ConstraintType::ForeignKey,
            AbstractConstraint {
                constraint_name: constraint_name.to_string(),
                constraint_value: format!(
                    "FOREIGN KEY ({}) REFERENCES {}({})",
                    column_name, referenced_table, referenced_column
                ),
                referenced_schema_name: None,
                referenced_table: Some(referenced_table.to_string()),
                u_referenced_table: Some(referenced_table.to_string()),
                referenced_column: Some(referenced_column.to_string()),
                u_referenced_column: Some(referenced_column.to_string()),
            },
        );
    }
}

fn main() {
    let mut adr = AbstractDbRepr {
        version: "0.1.0".to_string(),
        tables: BTreeMap::<String, AbstractTableRepr>::new(),
        views: BTreeMap::<String, AbstractTableRepr>::new(),
    };
    adr.tables.insert(
        "users".to_string(),
        AbstractTableRepr::new(
            ObjectType::BaseTable,
            "users".to_string(),
            Some("Users table".to_string()),
        ),
    );
    add_attribute(
        adr.tables.get_mut("users").unwrap(),
        "id",
        "integer",
        IsNullable::No,
        Some("nextval('users_id_seq'::regclass)"),
        Some("Primary key for users table"),
    );
    add_attribute(
        adr.tables.get_mut("users").unwrap(),
        "name",
        "text",
        IsNullable::Yes,
        None,
        Some("Name of the user"),
    );
    add_attribute(
        adr.tables.get_mut("users").unwrap(),
        "email",
        "text",
        IsNullable::Yes,
        None,
        Some("Email of the user"),
    );
    add_primary_key(adr.tables.get_mut("users").unwrap(), "id", "users_pkey");
    add_foreign_key(
        adr.tables.get_mut("users").unwrap(),
        "email",
        "email_fkey",
        "ldap_users",
        "email",
    );

    println!("Internal ADR before adding user type mapping: {:#?}", adr);
    add_user_mapping_to_adr(&build_default_type_mapping(), &mut adr);
    println!("Internal ADR after adding user type mapping: {:#?}", adr);
    let adr_template_data = AdrTemplateData::from(&adr);
    println!("External ADR for the templates: {:#?}", adr_template_data);

    let adr_json = serde_json::to_string_pretty(&adr).unwrap();
    let expected_adr: AbstractDbRepr = serde_json::from_str(ADR_JSON).unwrap();

    assert_eq!(
        adr_json,
        serde_json::to_string_pretty(&expected_adr).unwrap()
    );
}
