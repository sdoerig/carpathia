use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(
    author = "Stefan Dörig, sdoerig@bluewin.ch",
    version = "1.0.0",
    about = "Datebase layer generator for Rust",
    long_about = "Datebase layer generator for Rust. It generates code for database access based on a given schema."
)]

struct Args {
    /// Database URL in the format: postgres://user:password@localhost:5432/database
    #[arg( long)]
    database_url: String,
    /// Forces the generator to overwirite existing files allthough the database schema has not changed. Use this option if you want to update the generated code to the latest version of the generator.
    #[arg(short, long, default_value_t = false)]
    force: bool,
    /// Output format for the generated code - choices are "binary" (default) or "library"
    #[arg( long, default_value = "binary")]
    output_format: String,
    /// Output directory for the generated code   
    #[arg( long, default_value = "./src/db_layer")]
    output_directory: String,
}
fn main() {
    let args = Args::parse();
    if args.force {
        println!("Force option is enabled. Existing files will be overwritten.");
    } else {
        println!("Force option is disabled. Existing files will not be overwritten.");
    }
}
