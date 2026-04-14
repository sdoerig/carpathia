use clap::Parser;
use log::info;
mod db;
mod cache;

/// Database layer generator for Rust. It generates code for database access based on a given schema.
#[derive(Parser, Debug)]
#[command(
    author = "Stefan Dörig, sdoerig@bluewin.ch",
    version = "0.2.0",
    about = "Datebase layer generator for Rust",
    long_about = "Datebase layer generator for Rust. It generates code for database access based on a given schema. To enable logging, set the RUST_LOG environment variable to the desired log level (e.g., RUST_LOG=info) before running the application.
Note: It is still in early development and not functional yet."
)]
struct Args {
    /// Database URL in the format - JUST host and port NOT the database name: postgres://user:password@localhost:5432
    #[arg(long)]
    db_url: String,
    /// Database name you would like to generate code for - just the name NOT the full URL: my_database
    #[arg(long)]
    db_name: String,
    /// NOT IMPLEMENTED:Forces the generator to overwirite existing files allthough the database schema has not changed. Use this option if you want to update the generated code to the latest version of the generator.
    #[arg(short, long, default_value_t = false)]
    force: bool,
    /// NOT IMPLEMENTED: Output format for the generated code - choices are "binary" (default) or "library"
    #[arg(long, default_value = "binary")]
    output_format: String,
    /// NOT IMPLEMENTED: Output directory for the generated code   
    #[arg(long, default_value = "./src/db_layer")]
    output_directory: String,
    /// directory containing the carpatia_cache.json. The cache file contains hashes of the previously generated database entities   
    #[arg(long, default_value = ".")]
    cache_directory: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    if args.force {
        info!("Force option is enabled. Existing files will be overwritten.");
    } else {
        info!("Force option is disabled. Existing files will not be overwritten.");
    }
    info!("Database URL: {}", &args.db_url);
    info!("Database Name: {}", &args.db_name);
    info!("Output Format: {}", &args.output_format);
    info!("Output Directory: {}", &args.output_directory);
    let db_schema_parser =
        db::parse_db_schema::DbSchemaParser::new(args.db_url, args.db_name).await;
    let table_info_map = db_schema_parser.parse_schema().await?;

    info!(
        "Successfully parsed database schema. Found {} tables.",
        table_info_map.len()
    );
    Ok(())
}
