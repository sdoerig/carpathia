//! This module extracts the datebase schema from a `PostgreSQL` database and
//! generates a Rust struct for each table in the database. It also proviedes the
//! intermeditate data structures to hold the extracted schema information.
use crate::configuration::carpathia_conf::CarpathiaConfig;
use crate::configuration::conf_enums::DbPool;
use crate::db::db_schema_structs::AbstractDbRepr;
use crate::db::enrich_adr::add_user_mapping_to_adr;
use crate::db::postgres::postgresql::PostgresQuerier;
use crate::db::traits::DatabaseQuerier;
use crate::return_values::carpathia_errors::CarpathiaError;
pub struct DbSchemaParser {
    // You can add fields here if needed, for example, to hold configuration or state
}

impl DbSchemaParser {
    pub async fn parse_schema(config: &CarpathiaConfig) -> Result<AbstractDbRepr, CarpathiaError> {
        match config.db_pool {
            DbPool::Postgres(_) => match PostgresQuerier::get_schema(config).await {
                Ok(mut schema) => {
                    add_user_mapping_to_adr(config, &mut schema);
                    Ok(schema)
                }
                Err(e) => Err(e),
            },
            DbPool::Dummy => todo!("Dummy database pool not implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use super::*;
    use crate::configuration::carpathia_conf::CarpathiaConfigBuilder;
    use crate::configuration::conf_enums::DbType;
    use crate::configuration::conf_structs::Types;
    use crate::db::db_schema_structs::AbstractTableRepr;
    use crate::generator::template_engine::get_db_types;

    fn setup_test_config(with_type_mapping: bool) -> CarpathiaConfig {
        // Load .env.test (if available)
        dotenv::from_filename(".env.test").ok();

        let db_type = match std::env::var("TEST_DB_TYPE") {
            Ok(s) => s.parse::<DbType>().unwrap_or(DbType::Postgres),
            Err(_) => DbType::Postgres,
        };
        let db_host = std::env::var("TEST_DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let db_port = match std::env::var("TEST_DB_PORT") {
            Ok(s) => s.parse::<i32>().unwrap_or(5432),
            Err(_) => 5432,
        };
        let db_user = std::env::var("TEST_DB_USER").unwrap_or_else(|_| "postgres".to_string());
        let db_password =
            std::env::var("TEST_DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string());

        let db_name = std::env::var("TEST_DB_NAME").unwrap_or_else(|_| "carpathia".to_string());
        let db_type_mapping = if with_type_mapping {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("fixtures/carpathia_type_mapping.json")
        } else {
            PathBuf::from("I-do-not-exist.json")
        };
        CarpathiaConfigBuilder::new()
            .db_type(db_type)
            .db_host(db_host)
            .db_port(db_port)
            .db_user(db_user)
            .db_password(db_password)
            .db_name(&db_name)
            .db_type(DbType::Postgres)
            .cache_modus(crate::configuration::conf_enums::CacheModus::BypassCache)
            .carpathia_type_mapping(db_type_mapping)
            .output_directory("./output".to_string())
            .cache_file("./cache/carpathia_cache.json".to_string())
            .print_schema(false)
            .print_db_types(false)
            .execute_templates(true)
            .build()
            .expect("Config building failed...")
    }

    fn load_fixture<T>(fixture: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let file_path = PathBuf::from(manifest_dir)
            .parent()
            .unwrap() // carpathia/
            .join(fixture);
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let map: T = serde_json::from_reader(reader)?;
        Ok(map)
    }

    fn test_schema(
        retrieved_atr: &BTreeMap<String, AbstractTableRepr>,
        reference_atr: &BTreeMap<String, AbstractTableRepr>,
    ) {
        for reference_atr in reference_atr.values() {
            if let Some(test_atr) = retrieved_atr.get(&reference_atr.table_name) {
                assert!(
                    test_atr.table_name == reference_atr.table_name,
                    "DB object names do not match"
                );
                assert_eq!(
                    test_atr.u_imports, reference_atr.u_imports,
                    "DB object {} u_imports must be equal",
                    reference_atr.table_name
                );
                assert_eq!(
                    test_atr.attributes.len(),
                    reference_atr.attributes.len(),
                    "DB object {} attributes length must be equal",
                    reference_atr.table_name
                );
                assert_eq!(
                    test_atr.object_type, reference_atr.object_type,
                    "DB object {} object_type must be equal",
                    reference_atr.table_name
                );
                assert_eq!(
                    test_atr.u_table_name, reference_atr.u_table_name,
                    "DB object {} u_table_name must be equal",
                    reference_atr.table_name
                );

                for reference_attr in reference_atr.attributes.values() {
                    if let Some(test_attr) = test_atr.attributes.get(&reference_attr.column_name) {
                        let attr_name = &reference_attr.column_name;
                        assert_eq!(
                            test_attr.u_type, reference_attr.u_type,
                            "DB object {} attribute {} u_type must be equal",
                            reference_atr.table_name, attr_name
                        );
                        assert_eq!(
                            test_attr.u_column_name, reference_attr.u_column_name,
                            "DB object {} attribute {} u_column_name must be equal",
                            reference_atr.table_name, attr_name
                        );
                        assert_eq!(
                            test_attr.data_type, reference_attr.data_type,
                            "DB object {} attribute {} data_type must be equal",
                            reference_atr.table_name, attr_name
                        );
                        assert_eq!(
                            test_attr.is_nullable, reference_attr.is_nullable,
                            "DB object {} attribute {} is_nullable must be equal",
                            reference_atr.table_name, attr_name
                        );
                        for (reference_constraint_name, reference_constraint) in
                            &reference_attr.constraints
                        {
                            if let Some(test_constraint) =
                                test_attr.constraints.get(reference_constraint_name)
                            {
                                assert_eq!(
                                    test_constraint.constraint_name,
                                    reference_constraint.constraint_name,
                                    "DB object {} attribute {} constraint {} constraint_name must be equal",
                                    reference_atr.table_name,
                                    attr_name,
                                    reference_constraint.constraint_name
                                );
                                assert_eq!(
                                    test_constraint.referenced_column,
                                    reference_constraint.referenced_column,
                                    "DB object {} attribute {} constraint {} referenced_column must be equal",
                                    reference_atr.table_name,
                                    attr_name,
                                    reference_constraint.constraint_name
                                );
                                assert_eq!(
                                    test_constraint.referenced_table,
                                    reference_constraint.referenced_table,
                                    "DB object {} attribute {} constraint {} referenced_table must be equal",
                                    reference_atr.table_name,
                                    attr_name,
                                    reference_constraint.constraint_name
                                );
                            } else {
                                panic!(
                                    "DB object {} attribute {} constraint {:?} not found in test schema",
                                    reference_atr.table_name, attr_name, reference_constraint_name
                                );
                            }
                        }
                    } else {
                        // No exprected DB object - something is seriously wrong. Now do panic...
                        panic!("DB object {} not found", reference_atr.table_name)
                    }
                }
            }
        }
    }

    #[cfg(feature = "postgres")]
    #[tokio::test]
    async fn test_db_schema_parser_no_type_mapping() {
        // load .env.test if available.
        dotenv::from_filename(".env.test").ok();

        let config = setup_test_config(false);
        let schema = DbSchemaParser::parse_schema(&config).await.unwrap();
        assert!(
            !schema.tables.is_empty(),
            "Schema tables should not be empty"
        );
        assert!(!schema.views.is_empty(), "Schema views should not be empty");

        let test_adr_no_type_mapping: AbstractDbRepr =
            match load_fixture("fixtures/pagila_schema_no_user_type_mapping.json") {
                Ok(expected_schema) => expected_schema,
                Err(e) => panic!("Failed to load expected schema: {}", e),
            };
        test_schema(&schema.tables, &test_adr_no_type_mapping.tables);
        test_schema(&schema.views, &test_adr_no_type_mapping.views);
    }

    #[cfg(feature = "postgres")]
    #[tokio::test]
    async fn test_db_schema_parser_with_type_mapping() {
        // load .env.test if available.
        dotenv::from_filename(".env.test").ok();

        let config = setup_test_config(true);
        let schema = DbSchemaParser::parse_schema(&config).await.unwrap();
        assert!(
            !schema.tables.is_empty(),
            "Schema tables should not be empty"
        );
        assert!(!schema.views.is_empty(), "Schema views should not be empty");

        let test_adr_with_type_mapping: AbstractDbRepr =
            match load_fixture("fixtures/pagila_schema_user_type_mapping.json") {
                Ok(expected_schema) => expected_schema,
                Err(e) => panic!("Failed to load expected schema: {}", e),
            };
        test_schema(&schema.tables, &test_adr_with_type_mapping.tables);
        test_schema(&schema.views, &test_adr_with_type_mapping.views);
    }

    #[cfg(feature = "postgres")]
    #[tokio::test]
    async fn test_db_types() {
        dotenv::from_filename(".env.test").ok();

        let mut config = setup_test_config(false);
        config.print_db_types = true;
        let abstr_db_repr = DbSchemaParser::parse_schema(&config).await.unwrap();
        let db_types = match get_db_types(&config, &abstr_db_repr) {
            Ok(t) => t,
            Err(e) => panic!("Must have db types got error {}", e),
        };

        let db_types_fixtures: Types = match load_fixture("fixtures/db_types_not_mapped.json") {
            Ok(t) => t,
            Err(e) => panic!("Could not read {:?}", e),
        };

        assert_eq!(db_types, db_types_fixtures, "Db_types must be the same");
    }
}
