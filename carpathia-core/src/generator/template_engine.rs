use crate::cache::cache_file::Cache;
use crate::cache::cache_structs::CacheFile;
use crate::configuration::carpathia_conf::CarpathiaConfig;
use crate::configuration::conf_structs::{DEFAULT_TYPE_MAPPING, Types};
use crate::db::db_schema_structs::AbstractDbRepr;
use crate::generator::generator_structs::{Template, TemplateType};
use crate::return_values::carpathia_errors::{CarpathiaError, ErrorNumber};
use log::{debug, error};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
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
        // make sure output directory exists
        let _ = fs::create_dir_all(&config.output_directory).map_err(|e| {
            error!("Failed to create output directory: {e}");
        });
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

        // Tera-Instanz vorbereiten, um die Templates zu verarbeiten
        let mut tera = Tera::default();
        for (name, path) in &templates {
            tera.add_template_file(path, Some(name))
                .map_err(|e| CarpathiaError {
                    message: format!("Failure to load template {name}: {e}"),
                    error_type: ErrorNumber::Other,
                })?;
        }

        // Zielverzeichnis sicherstellen (falls es noch nicht existiert)
        let output_dir = PathBuf::from(&config.output_directory);

        // Wir loopen durch alle gefundenen Template-Dateien
        for template_file_name in templates.keys() {
            // Wir jagen den Dateinamen durch den neuen Parser des Autors!
            let parsed_template = Template::new(template_file_name);

            match parsed_template.template_type {
                // FALL 1: Es ist ein Template für Tabellen (z.B. "tables.rs.tera")
                TemplateType::Table => {
                    // Wir generieren Code NUR für die Tabellen, die der Cache als "geändert" markiert hat
                    for table_name in adr.tables.keys() {
                        if let Some(table_repr) = adr.tables.get(table_name)
                            && (cache_diff.tables.to_generate.contains(table_name)
                                || cache_diff
                                    .templates
                                    .to_generate
                                    .contains(template_file_name))
                        {
                            Self::render_table_or_view(
                                &tera,
                                &output_dir,
                                template_file_name,
                                &parsed_template,
                                table_name,
                                table_repr,
                            )?;
                        }
                    }
                }

                // FALL 2: Es ist ein Template für Views (z.B. "views.rs.tera")
                TemplateType::View => {
                    for view_name in adr.views.keys() {
                        if let Some(view_repr) = adr.views.get(view_name)
                            && (cache_diff.views.to_generate.contains(view_name)
                                || cache_diff
                                    .templates
                                    .to_generate
                                    .contains(template_file_name))
                        {
                            Self::render_table_or_view(
                                &tera,
                                &output_dir,
                                template_file_name,
                                &parsed_template,
                                view_name,
                                view_repr,
                            )?;
                        }
                    }
                }

                // FALL 3: Es ist ein globales Summary-Template (z.B. "summary.mod.rs.tera")
                TemplateType::Summary => {
                    // Summary wird generiert, wenn sich IRGENDETWAS (Tabelle, View oder das Template selbst) geändert hat
                    if !cache_diff.tables.to_generate.is_empty()
                        || !cache_diff.views.to_generate.is_empty()
                        || cache_diff
                            .templates
                            .to_generate
                            .contains(template_file_name)
                    {
                        //println!("Tera VERSION = {}", tera::Tera::version());
                        //println!("TYPE adr = {}", std::any::type_name_of_val(adr));
                        let rendered = Self::render_from_adr_repr(&tera, template_file_name, adr)
                            .map_err(|e| CarpathiaError {
                            message: e.to_string(),
                            error_type: ErrorNumber::Other,
                        })?;

                        // Nutzt den aus dem Namen parsten Dateinamen: z.B. "./generated_files/mod.rs"
                        let file_path = output_dir.join(format!(
                            "{}.{}",
                            parsed_template.file_name, parsed_template.suffix
                        ));
                        fs::write(file_path, rendered).map_err(|e| CarpathiaError {
                            message: format!("Could not write rendered file {e}"),
                            error_type: ErrorNumber::FileWriteError,
                        })?;
                    }
                }

                TemplateType::Unknown => {
                    // Ignorieren oder loggen – der Parser hat die Datei nicht als Carpathia-Template erkannt
                }
            }
        }

        Ok(())
    }

    fn render_from_repr(
        tera: &Tera,
        template_name: &str,
        repr: &impl serde::Serialize,
        tera_ctx_key: Vec<&str>,
    ) -> Result<String, CarpathiaError> {
        let mut ctx = Context::new();
        for ctx_key in tera_ctx_key {
            ctx.insert(ctx_key, repr);
        }
        match tera.render(template_name, &ctx) {
            Ok(r) => Ok(r),
            Err(e) => {
                error!("TERA ERROR: {:#?} template_name {}", e, template_name);
                Err(CarpathiaError {
                    message: format!("TERA ERROR: {:#?} template_name {}", e, template_name),
                    error_type: ErrorNumber::GenerationError,
                })
            }
        }

        //eprintln!("TERA ERROR: {:#?} template_name {}", res, template_name);
        //res
    }

    fn render_from_adr_repr(
        tera: &Tera,
        template_name: &str,
        repr: &AbstractDbRepr,
    ) -> Result<String, tera::Error> {
        let mut ctx = Context::new();
        ctx.insert("tables", &repr.tables);
        ctx.insert("views", &repr.views);
        let res = tera.render(template_name, &ctx);
        //eprintln!("TERA ERROR: {:#?} template_name {}", res, template_name);
        res
    }

    fn render_table_or_view(
        tera: &Tera,
        output_dir: &Path,
        template_file_name: &str,
        parsed_template: &Template,
        table_name: &str,
        table_repr: &crate::db::db_schema_structs::AbstractTableRepr,
    ) -> Result<(), CarpathiaError> {
        let rendered = Self::render_from_repr(tera, template_file_name, table_repr, vec!["table"])
            .map_err(|e| CarpathiaError {
                message: e.to_string(),
                error_type: ErrorNumber::Other,
            })?;
        let file_path = output_dir.join(format!(
            "{}.{}",
            table_name.to_lowercase(),
            parsed_template.suffix
        ));
        fs::write(file_path, rendered).map_err(|e| CarpathiaError {
            message: format!("Could not write rendered file {e}"),
            error_type: ErrorNumber::FileWriteError,
        })?;
        Ok(())
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
