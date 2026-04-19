# General

## Idea of mine

* Command line interface
* Generating programmable DB-Layer to avoid boilerplate code
* Using a templating lib like tera to generate the templates
* Allow the users writing their own templates 
* Motivation of mine: learning

## Proposed CLI:

```
Datebase layer generator for Rust

Usage: carpathia [OPTIONS] --db-url <DB_URL> --db-name <DB_NAME>

Options:
      --db-url <DB_URL>
          Database URL in the format - JUST host and port NOT the database name: postgres://user:password@localhost:5432
      --db-name <DB_NAME>
          Database name you would like to generate code for - just the name NOT the full URL: my_database
  -f, --force
          Forces the generator to overwirite existing files allthough the database schema has not changed. Use this option if you want to update the generated code to the latest version of the generator
      --output-format <OUTPUT_FORMAT>
          NOT IMPLEMENTED: Output format for the generated code - choices are "binary" (default) or "library" [default: binary]
      --output-directory <OUTPUT_DIRECTORY>
          NOT IMPLEMENTED: Output directory for the generated code [default: ./src/db_layer]
      --cache-directory <CACHE_DIRECTORY>
          directory containing the carpatia_cache.json. The cache file contains hashes of the previously generated database entities [default: .]
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version

```
