use crate::cache::cache_file::Cache;
use crate::cache::cache_structs::{CacheFile};
use crate::configuration::carpathia_conf::CarpathiaConfig;
use crate::configuration::conf_structs::{DEFAULT_TYPE_MAPPING, Types};
use crate::db::db_schema_structs::AbstractDbRepr;
use crate::return_values::carpathia_errors::{CarpathiaError, ErrorNumber};
use log::{debug, error};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use tera::{Context, Tera};



#[expect(dead_code)]
pub struct TemplateEngine {
    cache_result: CacheFile,
    db_schema: std::collections::HashMap<String, AbstractDbRepr>,
}

impl TemplateEngine {
    #[expect(dead_code)]
    pub(crate) fn new(
        cache_result: CacheFile,
        db_schema: std::collections::HashMap<String, AbstractDbRepr>,
    ) -> Self {
        Self {
            cache_result,
            db_schema,
        }
    }

    #[allow(unused_variables)]
    pub fn generate_code(
        config: &CarpathiaConfig,
        adr: &AbstractDbRepr,
    ) -> Result<(), CarpathiaError> {
        debug!("tera template directory is {:?}", config.template_directory);
        debug!("output directory is {:?}", config.output_directory);
        let templates = match list_files(&config.template_directory) {
            Ok(templates) => {
                debug!("templates found {:?}", templates);
                templates
            }
            Err(e) => {
                error!("No templates found");
                return Err(CarpathiaError {
                    message: format!(
                        "No templates found in directory {:?}, error {}",
                        config.template_directory, e
                    ),
                    error_type: ErrorNumber::NoTemplatesFound,
                });
            }
        };
        let cache_diff = match Cache::get_changed_entities(config, adr, &templates) {
            Ok(cache_diff) => cache_diff,
            Err(e) => {
                error!("Error while checking for changed entities: {e}");
                return Err(CarpathiaError {
                    message: format!("Error while checking for changed entities: {}", e),
                    error_type: ErrorNumber::CacheFileError,
                });
            }
        };
        Ok(())
    }

    pub fn render_from_repr(
        tera: &Tera,
        template_name: &str,
        repr: &AbstractDbRepr,
    ) -> Result<String, tera::Error> {
        let mut ctx = Context::new();
        ctx.insert("version", &repr.version);
        ctx.insert("tables", &repr.tables);
        ctx.insert("views", &repr.views);

        tera.render(template_name, &ctx)
    }
}

fn list_files(dir: &PathBuf) -> io::Result<BTreeMap<String, PathBuf>> {
    let mut files: BTreeMap<String, PathBuf> = BTreeMap::new();
    for entry in fs::read_dir(dir)? {
        let file_name = entry?.file_name().to_string_lossy().into_owned();
        if file_name.ends_with(".tera") {
            files.insert(file_name.clone(), dir.to_path_buf().join(file_name));
        }
    }
    Ok(files)
}

/// Returning all the types found in the database schema - the users need this to
/// create their own mapping.
///
/// If there is a mapping file provided, the old mapping is merged into the
/// new mapping structure.
pub fn get_db_types(
    config: &CarpathiaConfig,
    table_info_map: &AbstractDbRepr,
) -> Result<Types, CarpathiaError> {
    if table_info_map.tables.is_empty() {
        return Err(CarpathiaError {
            message: "No tables found".to_string(),
            error_type: ErrorNumber::NoDbObjectsDiscovered,
        });
    }

    let mut types = Types::new();

    // Chaining loops - otherwhise I had to create loop in loop.
    table_info_map
        .tables
        .values()
        .flat_map(|table| table.attributes.values())
        .for_each(|attribute| {
            let u_import_old = config
                .type_map
                .type_mapping
                .get(&attribute.data_type)
                .unwrap_or(DEFAULT_TYPE_MAPPING);

            types
                .type_mapping
                .entry(attribute.data_type.clone())
                .or_insert_with(|| u_import_old.clone());
        });

    Ok(types)
}
