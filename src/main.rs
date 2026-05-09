use clap::Parser;
use log::{error, info};
use std::path::PathBuf;
use std::process::exit;
mod cache;
mod db;
mod generator;
mod return_values;

use crate::generator::template_engine;
use cache::cache_structs::CacheModus;
use db::db_schema_structs::DbType;
/// Database layer generator for Rust. It generates code for database access based on a given schema.
#[derive(Parser, Debug)]
#[command(
    author = "Stefan Dörig, sdoerig@bluewin.ch",
    version = "0.2.0",
    about = "Template based, language-agnostic database layer generator.",
    long_about = "It generates code for database access based on a given schema. You write the templates - it genrates the code. It is based on an abstract database representation (ADR).
The ADR is an intermediate representation of the database schema that is independent of any specific database type. It allows us to decouple the database schema parsing from the code generation, making it easier to support multiple database types in the future. The ADR is defined in the `db_schema_structs` module and consists of two main structs: `AbstractDbRepr` and `AbstractAttribute`. The `AbstractDbRepr` struct represents a database table and contains the table name and a vector of `AbstractAttribute` structs, which represent the columns of the table and their properties.
The generator currently supports PostgreSQL, but one could easily add support for MySQL and SQLite in the future by implementing the necessary logic in the database querier and schema parser.
To enable logging, set the RUST_LOG environment variable to the desired log level (e.g., RUST_LOG=info) before running the application.
Note: It is still in early development and not functional yet."
)]
struct Args {
    /// Database URL in the format - JUST host and port NOT the database name: <postgres://user:password@localhost:5432>
    #[arg(long)]
    db_url: String,
    /// Database name you would like to generate code for - just the name NOT the full URL: `my_database`
    #[arg(long)]
    db_name: String,
    /// Forces the generator to overwirite existing files allthough the database schema has not changed. Use this option if you want to update the generated code to the latest version of the generator.
    #[arg(long, value_enum, default_value_t = CacheModus::UseCache)]
    cache_modus: CacheModus,
    /// NOT IMPLEMENTED: Output format for the generated code - choices are "binary" (default) or "library"
    #[arg(long, default_value = "binary")]
    output_format: String,
    /// NOT IMPLEMENTED: Output directory for the generated code   
    #[arg(long, default_value = "./src/db_layer")]
    output_directory: String,
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
    info!("Output Format: {}", &args.output_format);
    info!("Output Directory: {}", &args.output_directory);
    let db_schema_parser =
        db::parse_db_schema::DbSchemaParser::new(args.db_url, args.db_name, DbType::Postgres);
    let table_info_map = match db_schema_parser.parse_schema().await {
        Ok(schema) => schema,
        Err(e) => {
            //error!("Error parsing database schema: {}", e);
            exit(i32::from(e.error_type));
        }
    };
    let cache =
        cache::cache_file::Cache::new(PathBuf::from(args.cache_directory), args.cache_modus);
    match cache.get_changed_entities(&table_info_map) {
        Ok(changed_entities) => {
            if args.print_schema {
                println!(
                    "Extracted database schema in JSON format:\n{}",
                    serde_json::to_string_pretty(&table_info_map)?
                );
            }
            if args.print_db_types {
                template_engine::print_db_types_as_json(&table_info_map)?;
            }
        }
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
