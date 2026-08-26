use crate::db::parse_db_schema::DbSchemaParser;
use crate::enums::InitTemplateClap;
use crate::generator::template_engine;
use crate::return_values::carpathia_errors::ErrorNumber;
use crate::template_engine::TemplateEngine;
mod enums;
use carpathia_core::generator::tera_conversion::AdrTemplateData;
use carpathia_core::templates::enum_templates::InitTemplate;
use carpathia_core::*;
use clap::{Parser, Subcommand};
use configuration::carpathia_conf::CarpathiaConfigBuilder;
use configuration::conf_enums::{CacheModus, DbType};
use enums::{CacheModusClap, DbTypeClap};
use log::{error, info};
use std::process::exit;

/// Database layer generator for Rust. It generates code for database access based on a given schema.
#[derive(Parser, Debug)]
#[command(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = "It generates code for database access based on a given schema. You write the templates - it generates the code. Note: It is functional but in beta status."
)]
struct Args {
    /// Output directory for the generated code
    #[arg(long, global = true, default_value = "./generated_files")]
    output_directory: String,

    /// Template directory containing the tera templates
    #[arg(long, global = true, default_value = "./tera/rust_lib")]
    template_directory: String,

    /// JSON mapping file. Maps the database types to the user's types and imports.
    #[arg(long, global = true, default_value = "carpathia_type_mapping.json")]
    carpathia_type_mapping_file: String,

    /// Where to store carpathia's cache file. Contains hashes of previously generated database entities.
    #[arg(long, global = true, default_value = "./carpathia_cache.json")]
    cache_file: String,

    /// Writes basic example tera templates into the template_directory.
    #[arg(long, global = true, value_enum, default_value_t = InitTemplateClap::None)]
    init_template: InitTemplateClap,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Execute database operations and code generation
    Execute {
        /// Database host
        #[arg(long, required = true)]
        db_host: String,
        /// Database port
        #[arg(long, required = true)]
        db_port: i32,
        /// Database user name - read-only access is sufficient
        #[arg(long, required = true)]
        db_username: String,
        /// Database password
        #[arg(long, required = true)]
        db_password: String,
        /// Database name you would like to generate code for (e.g., `my_database`)
        #[arg(long, required = true)]
        db_name: String,
        /// Database type - currently only `Postgres` is supported; MySQL and SQLite are planned.
        #[arg(long, value_enum, default_value_t = DbTypeClap::Postgres)]
        db_type: DbTypeClap,
        /// Forces the generator to overwrite existing files even if the database schema has not changed.
        #[arg(long, value_enum, default_value_t = CacheModusClap::UseCache)]
        cache_modus: CacheModusClap,
        /// Print the extracted database schema to the console in JSON format.
        #[arg(long, default_value_t = false)]
        print_schema: bool,
        /// Prints the internal schema representation (for debugging only).
        #[arg(long, default_value_t = false)]
        print_internal_schema: bool,
        /// Print a JSON file of the database types to the console.
        #[arg(long, default_value_t = false)]
        print_db_types: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    // Handle --init-template globally (no subcommand needed)
    if args.init_template != InitTemplateClap::None {
        let config = match CarpathiaConfigBuilder::new()
            .template_directory(args.template_directory.clone())
            .init_template(args.init_template.into())
            .output_directory(&args.output_directory)
            .build()
        {
            Ok(config) => config,
            Err(e) => {
                error!("Error creating configuration - : {}", e);
                exit(i32::from(e.error_type));
            }
        };
        match carpathia_core::templates::init_templates::extract_to_disk(&config) {
            Ok(_) => {
                info!("Successfully initialized template.");
                exit(0);
            }
            Err(e) => {
                error!("Error while initializing template: {e}");
                exit(i32::from(e.error_type));
            }
        };
    }

    // Handle `execute` subcommand if provided
    if let Some(Commands::Execute {
        db_host,
        db_port,
        db_username,
        db_password,
        db_name,
        db_type,
        cache_modus,
        print_schema,
        print_internal_schema,
        print_db_types,
    }) = args.command
    {
        let core_db_type: DbType = db_type.into();
        let core_cache_modus: CacheModus = cache_modus.into();

        if core_cache_modus == CacheModus::BypassCache {
            info!("Bypassing cache - existing files will be overwritten.");
        } else {
            info!("Using cache - only changed files will be overwritten.");
        }
        info!(
            "Database Type: {} User: {} Port: {} Database name: {}",
            core_db_type, db_username, db_port, db_name
        );
        info!("Database Name: {}", db_name);
        info!("Output Directory: {}", args.output_directory);

        let config = match CarpathiaConfigBuilder::new()
            .db_host(&db_host)
            .db_port(db_port)
            .db_user(&db_username)
            .db_password(&db_password)
            .db_name(&db_name)
            .db_type(core_db_type)
            .cache_modus(core_cache_modus)
            .template_directory(args.template_directory)
            .init_template(InitTemplate::None) // Already handled globally
            .carpathia_type_mapping(args.carpathia_type_mapping_file)
            .output_directory(&args.output_directory)
            .cache_file(&args.cache_file)
            .print_schema(print_schema)
            .print_db_types(print_db_types)
            .execute_templates(true)
            .build()
        {
            Ok(config) => config,
            Err(e) => {
                error!("Error creating configuration - : {}", e);
                exit(i32::from(e.error_type));
            }
        };

        let abstr_db_repr = match DbSchemaParser::parse_schema(&config).await {
            Ok(schema) => schema,
            Err(e) => {
                error!("Error parsing database schema: {}", e);
                exit(i32::from(e.error_type));
            }
        };

        if print_internal_schema {
            match serde_json::to_string_pretty(&abstr_db_repr) {
                Ok(json) => println!("{json}"),
                Err(e) => {
                    error!("Could not print internal schema {e}");
                    exit(i32::from(ErrorNumber::Other));
                }
            }
            exit(0);
        }

        if config.print_schema {
            match serde_json::to_string_pretty(&AdrTemplateData::from(&abstr_db_repr)) {
                Ok(json) => println!("{json}"),
                Err(e) => {
                    error!("Could not print schema {e}");
                    exit(i32::from(ErrorNumber::Other));
                }
            }
            exit(0);
        }

        if config.print_db_types {
            match template_engine::get_db_types(&config, &abstr_db_repr) {
                Ok(db_types) => match serde_json::to_string_pretty(&db_types) {
                    Ok(json) => println!("{json}"),
                    Err(e) => {
                        error!("Could not get DB types {e}");
                        exit(i32::from(ErrorNumber::Other));
                    }
                },
                Err(e) => {
                    error!("Could not print type mapping {}", e);
                    exit(i32::from(ErrorNumber::Other));
                }
            }
            exit(0);
        }

        match TemplateEngine::generate_code(&config, &abstr_db_repr) {
            Ok(_) => {
                info!(
                    "Successfully parsed database schema. Found {} tables.",
                    abstr_db_repr.tables.len()
                );
                exit(0);
            }
            Err(e) => {
                error!("Error while checking for changed entities: {e}");
                exit(i32::from(e.error_type));
            }
        };
    }

    Ok(())
}
