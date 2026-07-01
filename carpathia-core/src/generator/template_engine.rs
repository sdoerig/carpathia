use crate::cache::cache_file::Cache;
use crate::configuration::carpathia_conf::CarpathiaConfig;
use crate::configuration::conf_structs::{DEFAULT_TYPE_MAPPING, Types};
use crate::db::db_schema_structs::AbstractDbRepr;
use crate::generator::generator_structs::{Template, TemplateType};
use crate::return_values::carpathia_errors::{CarpathiaError, ErrorNumber};
use log::{debug, error, info};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

pub struct TemplateEngine {}

impl TemplateEngine {
    pub fn generate_code(
        config: &CarpathiaConfig,
        adr: &AbstractDbRepr,
    ) -> Result<(), CarpathiaError> {
        if !config.execute_templates {
            return Ok(());
        }
        debug!("tera template directory is {:?}", config.template_directory);
        debug!("output directory is {:?}", config.output_directory);

        let templates = match list_files(
            &config.template_directory,
            &config.template_directory,
            "tera",
        ) {
            Ok(templates) => {
                info!("templates found {:?}", templates);
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
        }; // Convert template keys from PathBuf to String for cache function
        let template_keys: BTreeMap<String, PathBuf> = templates
            .iter()
            .map(|(k, v)| (k.to_string_lossy().to_string(), v.clone()))
            .collect();
        let mut cache = Cache::new(config).map_err(|e| CarpathiaError {
            message: format!("Failed to create cache: {}", e),
            error_type: ErrorNumber::CacheFileError,
        })?;
        let cache_diff = match cache.get_changed_entities(adr, &template_keys) {
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
            let name_str = name.to_string_lossy();
            tera.add_template_file(path, Some(&name_str))
                .map_err(|e| CarpathiaError {
                    message: format!("Failure to load template {name_str}: {e}"),
                    error_type: ErrorNumber::Other,
                })?;
        }

        // Found some templates, let's loop through them
        for template_file_name in templates.keys() {
            let parsed_template = match Template::new(&config.output_directory, template_file_name)
            {
                Ok(template) => template,
                Err(e) => {
                    error!(
                        "Error while parsing template {}: {}",
                        template_file_name.display(),
                        e
                    );
                    return Err(CarpathiaError {
                        message: format!(
                            "Error while parsing template {}: {}",
                            template_file_name.display(),
                            e
                        ),
                        error_type: ErrorNumber::InvalidConfiguration,
                    });
                }
            };

            match parsed_template.template_type {
                // tables.*.tera
                TemplateType::Table => {
                    // Generate code for all tables, if the template is a table template and
                    // either the table itself or the template has changed.
                    // Each AbstractTableRepr is passed separately to each template.
                    // This because the user can create as may templates of this type as feeling in need of.
                    for table_name in adr.tables.keys() {
                        if let Some(table_repr) = adr.tables.get(table_name)
                            && (cache_diff.tables.to_generate.contains(table_name)
                                || cache_diff
                                    .templates
                                    .to_generate
                                    .contains(&template_file_name.to_string_lossy().to_string()))
                        {
                            Self::render_table_or_view(
                                &tera,
                                template_file_name,
                                &parsed_template,
                                table_name,
                                table_repr,
                            )?;
                        }
                    }
                }

                // views.*.tera
                TemplateType::View => {
                    // Generate code for all views, if the template is a view template and
                    // either the view itself or the template has changed.
                    // Each AbstractTableRepr is passed separately to each template.
                    // This because the user can create as may templates of this type as feeling in need of.
                    for view_name in adr.views.keys() {
                        if let Some(view_repr) = adr.views.get(view_name)
                            && (cache_diff.views.to_generate.contains(view_name)
                                || cache_diff
                                    .templates
                                    .to_generate
                                    .contains(&template_file_name.to_string_lossy().to_string()))
                        {
                            Self::render_table_or_view(
                                &tera,
                                template_file_name,
                                &parsed_template,
                                view_name,
                                view_repr,
                            )?;
                        }
                    }
                }

                // summary.*.tera
                TemplateType::Summary => {
                    // Generate summary, if the template is a summary template and if either a table, a view or the template itself has changed.
                    // Passing the whole ADR to the template, so that the template can decide on its own what to render. This is necessary, because the summary template might want to render a summary of all tables and views, so it needs the whole ADR to do so.
                    if !cache_diff.tables.to_generate.is_empty()
                        || !cache_diff.views.to_generate.is_empty()
                        || cache_diff
                            .templates
                            .to_generate
                            .contains(&template_file_name.to_string_lossy().to_string())
                    {
                        //println!("Tera VERSION = {}", tera::Tera::version());
                        //println!("TYPE adr = {}", std::any::type_name_of_val(adr));
                        let rendered = Self::render_from_repr(
                            &tera,
                            &template_file_name.to_string_lossy(),
                            &adr,
                            vec!["tables", "views"],
                        )
                        .map_err(|e| CarpathiaError {
                            message: e.to_string(),
                            error_type: ErrorNumber::Other,
                        })?;

                        parsed_template.write_rendered_template(&rendered, "")?;
                    }
                }

                TemplateType::Unknown => {
                    // Unknown template type, skippinging
                    info!(
                        "Unknown template type for template {:?}, skipping",
                        template_file_name
                    );
                }
            }
        }
        cache.write_cache().map_err(|e| CarpathiaError {
            message: format!("Failed to write cache: {}", e),
            error_type: ErrorNumber::CacheFileError,
        })?;
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
    }

    fn render_table_or_view(
        tera: &Tera,
        template_file_name: &Path,
        parsed_template: &Template,
        table_name: &str,
        table_repr: &crate::db::db_schema_structs::AbstractTableRepr,
    ) -> Result<(), CarpathiaError> {
        let rendered = Self::render_from_repr(
            tera,
            &template_file_name.to_string_lossy(),
            table_repr,
            vec!["table"],
        )
        .map_err(|e| CarpathiaError {
            message: e.to_string(),
            error_type: ErrorNumber::Other,
        })?;

        parsed_template.write_rendered_template(&rendered, &table_name.to_lowercase())?;
        Ok(())
    }
}

fn list_files(
    super_dir: &Path,
    dir: &Path,
    suffix: &str,
) -> io::Result<BTreeMap<PathBuf, PathBuf>> {
    let mut files = BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];

    while let Some(current_dir) = stack.pop() {
        for entry in fs::read_dir(&current_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|ext| ext.to_str()) == Some(suffix)
                && let Ok(stripped) = path.strip_prefix(super_dir)
            {
                files.insert(stripped.to_path_buf(), path);
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::carpathia_conf::CarpathiaConfigBuilder;
    use crate::configuration::conf_enums::CacheModus;
    use crate::configuration::conf_enums::DbType;
    use crate::db::parse_db_schema::DbSchemaParser;
    use crate::templates::enum_templates::InitTemplate;
    use crate::templates::init_templates::extract_to_disk;

    fn prepare_temp_file() -> CarpathiaConfig {
        let temp_dir = tempfile::tempdir().unwrap();

        let template_dir = temp_dir.path().to_path_buf().join("a").join("b").join("c");
        let cache_file_path = template_dir.clone().join("cache.json");
        let output_dir = temp_dir.path().to_path_buf().join("output");
        std::fs::create_dir_all(&template_dir)
            .map_err(|e| panic!("could not create template_dir {}", e));
        std::fs::create_dir_all(&output_dir)
            .map_err(|e| panic!("could not create output_dir {}", e));
        std::fs::write(&cache_file_path, "{}")
            .map_err(|e| panic!("could not create output_dir {}", e));

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

        CarpathiaConfigBuilder::new()
            .db_type(db_type)
            .db_host(db_host)
            .db_port(db_port)
            .db_user(db_user)
            .db_password(db_password)
            .db_name(&db_name)
            .db_type(DbType::Postgres)
            .cache_modus(CacheModus::BypassCache)
            .init_template(InitTemplate::RustLib)
            .execute_templates(true)
            .carpathia_type_mapping("carpathia_type_mapping.json".to_string())
            .output_directory(output_dir.as_path().to_string_lossy().to_string())
            .template_directory(temp_dir.path().to_string_lossy().to_string())
            .cache_file(cache_file_path.as_path().to_string_lossy().to_string())
            .print_schema(false)
            .print_db_types(false)
            .build()
            .expect("Config building failed...")
    }

    #[tokio::test]
    async fn test_extract_templates_to_fs() {
        let templates = [
            PathBuf::from("rust_lib/summary.mod.rs.tera"),
            PathBuf::from("rust_lib/tables.rs.tera"),
            PathBuf::from("rust_lib/views.rs.tera"),
        ];
        let conf = prepare_temp_file();

        extract_to_disk(&conf).map_err(|e| panic!("Could not extract templates {}", e));

        match list_files(&conf.template_directory, &conf.template_directory, "tera") {
            // silly test proofes nothing - create it later
            Ok(files) => {
                for p in templates.as_ref() {
                    assert!(
                        files.contains_key(p),
                        "{}",
                        format!("Could not find template on fs {:?}", p)
                    )
                }
            }
            Err(_) => todo!(),
        }
    }

    #[tokio::test]
    async fn test_template_generation() {
        let generated_files = [
            PathBuf::from("rust_lib/actor.rs"),
            PathBuf::from("rust_lib/actor_info.rs"),
            PathBuf::from("rust_lib/address.rs"),
            PathBuf::from("rust_lib/category.rs"),
            PathBuf::from("rust_lib/city.rs"),
            PathBuf::from("rust_lib/country.rs"),
            PathBuf::from("rust_lib/customer.rs"),
            PathBuf::from("rust_lib/customer_list.rs"),
            PathBuf::from("rust_lib/film.rs"),
            PathBuf::from("rust_lib/film_actor.rs"),
            PathBuf::from("rust_lib/film_category.rs"),
            PathBuf::from("rust_lib/film_list.rs"),
            PathBuf::from("rust_lib/inventory.rs"),
            PathBuf::from("rust_lib/language.rs"),
            PathBuf::from("rust_lib/mod.rs"),
            PathBuf::from("rust_lib/nicer_but_slower_film_list.rs"),
            PathBuf::from("rust_lib/payment_p2022_01.rs"),
            PathBuf::from("rust_lib/payment_p2022_02.rs"),
            PathBuf::from("rust_lib/payment_p2022_03.rs"),
            PathBuf::from("rust_lib/payment_p2022_04.rs"),
            PathBuf::from("rust_lib/payment_p2022_05.rs"),
            PathBuf::from("rust_lib/payment_p2022_06.rs"),
            PathBuf::from("rust_lib/payment_p2022_07.rs"),
            PathBuf::from("rust_lib/rental.rs"),
            PathBuf::from("rust_lib/rental_by_category.rs"),
            PathBuf::from("rust_lib/sales_by_film_category.rs"),
            PathBuf::from("rust_lib/sales_by_store.rs"),
            PathBuf::from("rust_lib/staff.rs"),
            PathBuf::from("rust_lib/staff_list.rs"),
            PathBuf::from("rust_lib/store.rs"),
        ];
        let conf = prepare_temp_file();
        extract_to_disk(&conf).map_err(|e| panic!("Could not extract templates {}", e));
        let abstr_db_repr = match DbSchemaParser::parse_schema(&conf).await {
            Ok(schema) => schema,
            Err(_e) => todo!(),
        };

        match TemplateEngine::generate_code(&conf, &abstr_db_repr) {
            Ok(_) => {
                info!(
                    "Successfully parsed database schema. Found {} tables.",
                    abstr_db_repr.tables.len()
                );
            }
            Err(_e) => todo!(),
        };

        let mut files = BTreeMap::new();
        files.insert(PathBuf::from("a"), PathBuf::from("a/b"));
        match list_files(&conf.output_directory, &conf.output_directory, "rs") {
            Ok(output_files) => {
                for gen_file in generated_files {
                    assert!(output_files.contains_key(&gen_file))
                }
            }
            Err(_) => todo!(),
        }
    }
}
