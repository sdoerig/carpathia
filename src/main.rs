use clap::Parser;
use log::info;
mod db;

/// Database layer generator for Rust. It generates code for database access based on a given schema.
#[derive(Parser, Debug)]
#[command(
    author = "Stefan Dörig, sdoerig@bluewin.ch",
    version = "1.0.0",
    about = "Datebase layer generator for Rust",
    long_about = "Datebase layer generator for Rust. It generates code for database access based on a given schema."
)]

struct Args {
    /// Database URL in the format - JUST host and port NOT the database name: postgres://user:password@localhost:5432
    #[arg(long)]
    db_url: String,
    /// Database name you would like to generate code for - just the name NOT the full URL: my_database
    #[arg(long)]
    db_name: String,
    /// Forces the generator to overwirite existing files allthough the database schema has not changed. Use this option if you want to update the generated code to the latest version of the generator.
    #[arg(short, long, default_value_t = false)]
    force: bool,
    /// Output format for the generated code - choices are "binary" (default) or "library"
    #[arg(long, default_value = "binary")]
    output_format: String,
    /// Output directory for the generated code   
    #[arg(long, default_value = "./src/db_layer")]
    output_directory: String,
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
    let db_schema_parser = db::parse_db_schema::DbSchemaParser::new(args.db_url, args.db_name).await;
    db_schema_parser.parse_schema().await?;

    Ok(())
}
