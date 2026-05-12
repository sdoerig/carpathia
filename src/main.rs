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
    /// Forces the generator to overwrite existing files allthough the database schema has not changed. Use this option if you want to update the generated code to the latest version of the generator.
    #[arg(long, value_enum, default_value_t = CacheModus::UseCache)]
    cache_modus: CacheModus,
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
        Ok(_changed_entities) => {
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
