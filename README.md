# General

## Idea of mine

* Command line interface
* Generating programmable DB-Layer to avoid boilerplate code
* Using a templating lib like tera to generate the templates
* Allow the users writing their own templates 
* Motivation of mine: learning

## Proposed CLI:

```
It generates code for database access based on a given schema. You write the templates - it genrates the code. It is based on an abstract database representation (ADR).
The ADR is an intermediate representation of the database schema that is independent of any specific database type. It allows us to decouple the database schema parsing from the code generation, making it easier to support multiple database types in the future. The ADR is defined in the `db_schema_structs` module and consists of two main structs: `AbstractDbRepr` and `AbstractAttribute`. The `AbstractDbRepr` struct represents a database table and contains the table name and a vector of `AbstractAttribute` structs, which represent the columns of the table and their properties.
The generator currently supports PostgreSQL, but one could easily add support for MySQL and SQLite in the future by implementing the necessary logic in the database querier and schema parser.
To enable logging, set the RUST_LOG environment variable to the desired log level (e.g., RUST_LOG=info) before running the application.
Note: It is still in early development and not functional yet.

Usage: carpathia [OPTIONS] --db-url <DB_URL> --db-name <DB_NAME>

Options:
      --db-url <DB_URL>
          Database URL in the format - JUST host and port NOT the database name: <postgres://user:password@localhost:5432>

      --db-name <DB_NAME>
          Database name you would like to generate code for - just the name NOT the full URL: `my_database`

  -f, --force
          Forces the generator to overwirite existing files allthough the database schema has not changed. Use this option if you want to update the generated code to the latest version of the generator

      --output-format <OUTPUT_FORMAT>
          NOT IMPLEMENTED: Output format for the generated code - choices are "binary" (default) or "library"
          
          [default: binary]

      --output-directory <OUTPUT_DIRECTORY>
          NOT IMPLEMENTED: Output directory for the generated code
          
          [default: ./src/db_layer]

      --cache-directory <CACHE_DIRECTORY>
          directory containing the `carpatia_cache.json`. The cache file contains hashes of the previously generated database entities
          
          [default: .]

      --print-schema
          print the extracted database schema to the console in JSON format for debugging purposes

      --print-db-types
          print a json file of the database types to the console. You might need this

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```
