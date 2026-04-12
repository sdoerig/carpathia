# General

## Idea of mine

* Command line interface
* Generating programmable DB-Layer to avoid boilerplate
* Motivation of mine: intressting problem, learning

## Proposed CLI:

```
Datebase layer generator for Rust. It generates code for database access based on a given schema. To enable logging, set the RUST_LOG environment variable to the desired log level (e.g., RUST_LOG=info) before running the application.
Note: It is still in early development and not functional yet.

Usage: carpathia [OPTIONS] --db-url <DB_URL> --db-name <DB_NAME>

Options:
      --db-url <DB_URL>
          Database URL in the format - JUST host and port NOT the database name: postgres://user:password@localhost:5432

      --db-name <DB_NAME>
          Database name you would like to generate code for - just the name NOT the full URL: my_database

  -f, --force
          NOT IMPLEMENTED:Forces the generator to overwirite existing files allthough the database schema has not changed. Use this option if you want to update the generated code to the latest version of the generator

      --output-format <OUTPUT_FORMAT>
          NOT IMPLEMENTED: Output format for the generated code - choices are "binary" (default) or "library"
          
          [default: binary]

      --output-directory <OUTPUT_DIRECTORY>
          NOT IMPLEMENTED: Output directory for the generated code
          
          [default: ./src/db_layer]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
