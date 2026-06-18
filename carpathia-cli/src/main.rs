use crate::db::parse_db_schema::DbSchemaParser;
use crate::generator::template_engine;
use crate::return_values::carpathia_errors::ErrorNumber;
use crate::template_engine::TemplateEngine;
mod enums;
use carpathia_core::*;
use clap::Parser;
use configuration::carpathia_conf::CarpathiaConfigBuilder;
use configuration::conf_enums::CacheModus;
use configuration::conf_enums::DbType;
use enums::{CacheModusClap, DbTypeClap};
use log::{error, info};
use std::process::exit;
/// Database layer generator for Rust. It generates code for database access based on a given schema.
#[derive(Parser, Debug)]
#[command(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = "It generates code for database access based on a given schema. You write the templates - it genrates the code. Note: It is funktional but in beta status."
)]
struct Args {
    /// Database host
    #[arg(long)]
    db_host: String,
    /// Database port
    #[arg(long)]
    db_port: i32,
    /// Database user name - read only will do it
    #[arg(long)]
    db_username: String,
    /// Database passwqord
    #[arg(long)]
    db_password: String,
    /// Database name you would like to generate code for - just the name NOT the full URL: `my_database`
    #[arg(long)]
    db_name: String,
    /// Database type - currently only `Postgres` is supported, MySQL  and SQLite planned in the future.
    #[arg(long, value_enum, default_value_t = DbTypeClap::Postgres)]
    db_type: DbTypeClap,
    /// Forces the generator to overwrite existing files allthough the database schema has not changed. Use this option if you want to update the generated code to the latest version of the generator.
    #[arg(long, value_enum, default_value_t = CacheModusClap::UseCache)]
    cache_modus: CacheModusClap,
    /// Output directory for the generated code   
    #[arg(long, default_value = "./generated_files")]
    output_directory: String,
    /// Template directory containing the tera templates   
    #[arg(long, default_value = "./tera/rust_lib")]
    template_directory: String,
    /// JSON mapping file. Here, maps the database types to the users types and imports.    
    #[arg(long, default_value = "carpathia_type_mapping.json")]
    carpathia_type_mapping_file: String,
    /// Where to store carpathias cache file. The cache file contains hashes of the previously generated database entities   
    #[arg(long, default_value = "./carpathia_cache.json")]
    cache_file: String,
    /// print the extracted database schema to the console in JSON format for debugging purposes.
    #[arg(long, default_value_t = false)]
    print_schema: bool,
    /// print a json file of the database types to the console. You might need this.
    #[arg(long, default_value_t = false)]
    print_db_types: bool,
    /// execute templates.
    #[arg(long, default_value_t = false)]
    execute_templates: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    let core_db_type: DbType = args.db_type.into();
    let core_cache_modus: CacheModus = args.cache_modus.into();
    if core_cache_modus == CacheModus::BypassCache {
        info!("Bypassing cache - existing files will be overwritten.");
    } else {
        info!("Using cache - only changed files will be overwritten.");
    }
    info!(
        "Database Type: {} User: {} Port: {} Database name: {}",
        &core_db_type, &args.db_username, args.db_port, &args.db_name
    );
    info!("Database Name: {}", &args.db_name);
    info!("Output Directory: {}", &args.output_directory);

    let config = match CarpathiaConfigBuilder::new()
        .db_host(&args.db_host)
        .db_port(args.db_port)
        .db_user(&args.db_username)
        .db_password(&args.db_password)
        .db_name(&args.db_name)
        .db_type(core_db_type)
        .cache_modus(core_cache_modus)
        .carpathia_type_mapping(args.carpathia_type_mapping_file)
        .output_directory(&args.output_directory)
        .cache_file(&args.cache_file)
        .print_schema(args.print_schema)
        .print_db_types(args.print_db_types)
        .execute_templates(args.execute_templates)
        .build()
    {
        Ok(config) => config,
        Err(e) => {
            error!("Error creating configuration: {}", e);
            exit(i32::from(e.error_type));
        }
    };

    let table_info_map = match DbSchemaParser::parse_schema(&config).await {
        Ok(schema) => schema,
        Err(e) => {
            error!("Error parsing database schema: {}", e);
            exit(i32::from(e.error_type));
        }
    };
    if config.print_schema {
        println!("{}", serde_json::to_string_pretty(&table_info_map)?);
        exit(0);
    }
    if config.print_db_types {
        match template_engine::get_db_types(&config, &table_info_map) {
            Ok(db_types) => match serde_json::to_string_pretty(&db_types) {
                Ok(json) => println!("{json}"),
                Err(e) => {
                    error!("Could not get DB types {e}");
                    exit(i32::from(ErrorNumber::Other));
                }
            },

            Err(e) => {
                error!("Could not print type mapping {}", e);
            }
        }
        exit(0);
    }
    match TemplateEngine::generate_code(&config, &table_info_map) {
        Ok(_) => {
            info!(
                "Successfully parsed database schema. Found {} tables.",
                table_info_map.tables.len()
            );
            exit(0);
        }
        Err(e) => {
            error!("Error while checking for changed entities: {e}");
            exit(i32::from(e.error_type));
        }
    };
}
