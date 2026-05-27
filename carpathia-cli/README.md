# Description

Carpathia is a CLI tool that generates database access code from your schema. You write the Tera templates—it produces the code. Currently supports Postgres (MySQL & SQLite planned). Functional but in beta: use it, test it, and help shape its future! 🚀

CLI

```
It generates code for database access based on a given schema. You write the templates - it genrates the code. Note: It is funktional but in beta status.

Usage: carpathia [OPTIONS] --db-host <DB_HOST> --db-port <DB_PORT> --db-username <DB_USERNAME> --db-password <DB_PASSWORD> --db-name <DB_NAME>

Options:
      --db-host <DB_HOST>
          Database host

      --db-port <DB_PORT>
          Database port

      --db-username <DB_USERNAME>
          Database user name - read only will do it

      --db-password <DB_PASSWORD>
          Database passwqord

      --db-name <DB_NAME>
          Database name you would like to generate code for - just the name NOT the full URL: `my_database`

      --db-type <DB_TYPE>
          Database type - currently only `Postgres` is supported, MySQL  and SQLite planned in the future
          
          [default: postgres]
          [possible values: postgres, dummy]

      --cache-modus <CACHE_MODUS>
          Forces the generator to overwrite existing files allthough the database schema has not changed. Use this option if you want to update the generated code to the latest version of the generator
          
          [default: use-cache]
          [possible values: bypass-cache, use-cache]

      --output-directory <OUTPUT_DIRECTORY>
          Output directory for the generated code
          
          [default: ./generated_files]

      --template-directory <TEMPLATE_DIRECTORY>
          Template directory containing the tera templates
          
          [default: ./tera/rust_lib]

      --carpathia-type-mapping-file <CARPATHIA_TYPE_MAPPING_FILE>
          JSON mapping file. Here, maps the database types to the users types and imports
          
          [default: carpathia_type_mapping.json]

      --cache-file <CACHE_FILE>
          Where to store carpathias cache file. The cache file contains hashes of the previously generated database entities
          
          [default: ./carpathia_cache.json]

      --print-schema
          print the extracted database schema to the console in JSON format for debugging purposes

      --print-db-types
          print a json file of the database types to the console. You might need this

      --execute-templates
          execute templates

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

