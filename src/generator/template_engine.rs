use crate::cache::cache_structs::CacheFile;
use crate::configuration::conf_structs::{TypeMapping, Types};
use crate::db::db_schema_structs::AbstractDbRepr;
use crate::return_values::carpathia_errors::CarpathiaError;
use log::{debug, error, info};
use std::collections::BTreeMap;

pub(crate) struct TemplateEngine {
    cache_result: CacheFile,
    db_schema: std::collections::HashMap<String, AbstractDbRepr>,
}

impl TemplateEngine {
    pub(crate) fn new(
        cache_result: CacheFile,
        db_schema: std::collections::HashMap<String, AbstractDbRepr>,
    ) -> Self {
        Self {
            cache_result,
            db_schema,
        }
    }

    pub(crate) fn generate_code(&self) -> Result<(), CarpathiaError> {
        // Here you would implement the logic to generate code based on the database schema and the cache result.
        // This is just a placeholder for demonstration purposes.
        info!("Generating code based on the database schema and cache result...");
        debug!("Cache result: {:?}", self.cache_result);
        debug!("Database schema: {:?}", self.db_schema);
        Ok(())
    }
}

pub(crate) fn get_db_types(table_info_map: &AbstractDbRepr) -> Result<Types, CarpathiaError> {
    // Printing the types found in the database this is needed
    // to give the users an overview ot the types found in the database
    // and helping them creating a mapping file for their types they wnat
    // to use in the generated code.
    let mut types = Types::new();

    for key in table_info_map.tables.keys() {
        for attribute in table_info_map.tables[key].attributes.values() {
            types
                .type_mapping
                .entry(attribute.data_type.clone())
                .or_insert(TypeMapping {
                    u_import: Some("".to_string()),
                    u_type: "".to_string(),
                });
        }
    }
    Ok(types)
    //    match serde_json::to_string_pretty(&db_types) {
    //        Ok(json) => {
    //            println!("{json}");
    //            Ok(())
    //        }
    //        Err(e) => {
    //            error!("Failed to serialize database schema to JSON: {e}");
    //            Err(CarpathiaError {
    //                message: format!("Failed to serialize database schema to JSON: {e}"),
    //                error_type: crate::return_values::carpathia_errors::ErrorNumber::Other,
    //            })
    //        }
    //    }
}
