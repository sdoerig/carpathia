// Copyright 2026 Stefan Dörig
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::cache::cache_structs::CacheFile;
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

pub(crate) fn print_db_types_as_json(
    table_info_map: &AbstractDbRepr,
) -> Result<(), CarpathiaError> {
    // Printing the types found in the database this is needed
    // to give the users an overview ot the types found in the database
    // and helping them creating a mapping file for their types they wnat
    // to use in the generated code.
    let mut db_types: BTreeMap<&str, String> = BTreeMap::new();
    for key in table_info_map.tables.keys() {
        for attribute in table_info_map.tables[key].attributes.values() {
            db_types.insert(&attribute.data_type, attribute.data_type.clone());
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
