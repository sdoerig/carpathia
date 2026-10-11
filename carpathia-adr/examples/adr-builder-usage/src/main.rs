//! Example usage of the `AbstractDbReprBuilder` to build an ADR and enrich it with user type mapping.
//! Note that the builder API is AI generated from the ADR itself.
//!
use carpathia_adr::adr::abstract_db_repr::{AbstractDbRepr, ConstraintType};
use carpathia_adr::adr::abstract_db_repr_builder::{
    AbstractConstraintBuilder,
    AbstractDbReprBuilder,
    //AbstractForeignKeyBuilder, AbstractPrimaryKeyBuilder,
    AbstractTableReprBuilder as TableBuilder,
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

fn main() {
    // The whole schema is now a single declarative expression. Note that:
    // - `u_type` values are placeholders here ("whatever"-style); the real
    //   mapping is applied afterwards by `add_user_mapping_to_adr`.
    // - Primary/foreign keys are declared directly where they belong - no
    //   `get_mut`-chains, no `if let Some(attr)` helper functions.
    let mut adr: AbstractDbRepr = AbstractDbReprBuilder::with_version("0.1.0")
        .table("users", |table: TableBuilder| {
            table
                .comment("Users table")
                // --- attributes -------------------------------------------------
                .attribute("id", |a| {
                    a.data_type("integer")
                        .not_nullable()
                        .is_primary_key(true)
                        .default("nextval('users_id_seq'::regclass)")
                        .comment("Primary key for users table")
                        .constraint(
                            ConstraintType::PrimaryKey,
                            AbstractConstraintBuilder::new("users_pkey")
                                .value("PRIMARY KEY (id)")
                                .build(),
                        )
                })
                .attribute("name", |a| {
                    a.data_type("text").nullable().comment("Name of the user")
                })
                .attribute("email", |a| {
                    a.data_type("text")
                        .nullable()
                        .comment("Email of the user")
                        .constraint(
                            ConstraintType::ForeignKey,
                            AbstractConstraintBuilder::new("email_fkey")
                                .value("FOREIGN KEY (email) REFERENCES ldap_users(email)")
                                .referenced_table("ldap_users")
                                .referenced_column("email")
                                .build(),
                        )
                })
        })
        .build();

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
