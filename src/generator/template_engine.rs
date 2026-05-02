use crate::cache::cache_file::CacheResult;
use crate::db::db_schema_structs::AbstractDbRepr;
use crate::return_values::carpathia_errors::CarpathiaError;
use log::{debug, error, info};
use std::collections::HashMap;

pub(crate) struct TemplateEngine {
    cache_result: CacheResult,
    db_schema: std::collections::HashMap<String, AbstractDbRepr>,
}

impl TemplateEngine {
    pub(crate) fn new(
        cache_result: CacheResult,
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

pub(crate) fn print_schema_as_json(
    table_info_map: &HashMap<String, AbstractDbRepr>,
) -> Result<(), CarpathiaError> {
    let mut db_types: HashMap<&str, String> = HashMap::new();
    for key in table_info_map.keys() {
        for attr in &table_info_map[key].attributes {
            db_types.insert(&attr.data_type, String::new());
        }
    }
    match serde_json::to_string_pretty(&db_types) {
        Ok(json) => {
            println!("{json}");
            Ok(())
        }
        Err(e) => {
            error!("Failed to serialize database schema to JSON: {e}");
            Err(CarpathiaError {
                message: format!("Failed to serialize database schema to JSON: {e}"),
                error_type: crate::return_values::carpathia_errors::ErrorNumber::Other,
            })
        }
    }
}
