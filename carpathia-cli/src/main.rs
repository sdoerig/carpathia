use crate::cache::cache_file::Cache;
use crate::db::parse_db_schema::DbSchemaParser;
use crate::generator::template_engine;
use crate::return_values::carpathia_errors::ErrorNumber;
use carpathia_core::*;
use clap::Parser;
use configuration::carpathia_conf::CarpathiaConfigBuilder;
use configuration::conf_enums::CacheModus;
use configuration::conf_enums::DbType;
use log::{error, info};
use std::process::exit;
/// Database layer generator for Rust. It generates code for database access based on a given schema.
#[derive(Parser, Debug)]
#[command(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = "It generates code for database access based on a given schema. You write the templates - it genrates the code. Note: It is still in early development and not functional yet."
)]
struct Args {
    /// Database URL in the format - JUST host and port NOT the database name: <postgres://user:password@localhost:5432>
    #[arg(long)]
    db_url: String,
    /// Database name you would like to generate code for - just the name NOT the full URL: `my_database`
    #[arg(long)]
    db_name: String,
    /// Database type - currently only `Postgres` is supported, MySQL  and SQLite planned in the future.
    #[arg(long, value_enum, default_value_t = DbType::Postgres)]
    db_type: DbType,
    /// Forces the generator to overwrite existing files allthough the database schema has not changed. Use this option if you want to update the generated code to the latest version of the generator.
    #[arg(long, value_enum, default_value_t = CacheModus::UseCache)]
    cache_modus: CacheModus,
    /// NOT IMPLEMENTED: Output directory for the generated code   
    #[arg(long, default_value = "./generated_files")]
    output_directory: String,
    /// JSON mapping file. Here, maps the database types to the users types and imports.    
    #[arg(long, default_value = "carpathia_type_mapping.json")]
    carpathia_type_mapping_file: String,
    /// directory containing the `carpatia_cache.json`. The cache file contains hashes of the previously generated database entities   
    #[arg(long, default_value = ".")]
    cache_directory: String,
    /// print the extracted database schema to the console in JSON format for debugging purposes.
    #[arg(long, default_value_t = false)]
    print_schema: bool,
    /// print a json file of the database types to the console. You might need this.
    #[arg(long, default_value_t = false)]
    print_db_types: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    if args.cache_modus == CacheModus::BypassCache {
        info!("Bypassing cache - existing files will be overwritten.");
    } else {
        info!("Using cache - only changed files will be overwritten.");
    }
    info!("Database URL: {}", &args.db_url);
    info!("Database Name: {}", &args.db_name);
    info!("Output Directory: {}", &args.output_directory);

    let config = match CarpathiaConfigBuilder::new()
        .db_url(&args.db_url)
        .db_name(&args.db_name)
        .db_type(args.db_type)
        .cache_modus(args.cache_modus)
        .carpathia_type_mapping(args.carpathia_type_mapping_file)
        .output_directory(&args.output_directory)
        .cache_directory(&args.cache_directory)
        .print_schema(args.print_schema)
        .print_db_types(args.print_db_types)
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
            //error!("Error parsing database schema: {}", e);
            exit(i32::from(e.error_type));
        }
    };
    if config.print_schema {
        println!(
            "Extracted database schema in JSON format:\n{}",
            serde_json::to_string_pretty(&table_info_map)?
        );
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
    }
    match Cache::get_changed_entities(&config, &table_info_map) {
        Ok(_changed_entities) => {}
        Err(e) => {
            error!("Error while checking for changed entities: {e}");
            exit(i32::from(e.error_type));
        }
    }
    info!(
        "Successfully parsed database schema. Found {} tables.",
        table_info_map.tables.len()
    );
    exit(0);
}
