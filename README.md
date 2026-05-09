![Test Status](https://github.com/sdoerig/carpathia/actions/workflows/test.yml/badge.svg)

# General

## Idea of mine

* Command line interface
* Generating programmable DB-Layer to avoid boilerplate code
* Using a templating lib like tera to generate the templates
* Allow the users writing their own templates 
* Motivation of mine: learning

# Current status
- Under development
- PostgreSQL support only at the moment

# Architectur 

```
                                   +----------------------+
                                   |       main.rs        |
                                   |----------------------|
                                   | CLI (clap)           |
                                   | Logging              |
                                   | Orchestrates flow    |
                                   +----------+-----------+
                                              |
                                              v
        +-------------------------------------+--------------------------------------+
        |                                                                            |
        v                                                                            v
+-------------------+                                                    +----------------------+
|       db/         |                                                    |       cache/         |
|-------------------|                                                    |----------------------|
| parse_db_schema   |                                                    | cache_file           |
| postgresql        |                                                    | cache_structs        |
| postgresql_structs|                                                    +----------+-----------+
| traits            |                                                               |
| db_schema_structs |                                                               |
+---------+---------+                                                               |
          |                                                                         |
          | produces ADR                                                            |
          v                                                                         |
+---------------------------+                                                       |
|  AbstractDbRepr (ADR)     |<------------------------------------------------------+
|  AbstractTableRepr        |                 CacheDiff (changed tables only)        
|  AbstractAttribute        |
+-------------+-------------+
              |
              | enriched ADR (after applying user type mapping)
              v
      +----------------------------+
      |   User Type Mapping JSON   |
      |----------------------------|
      | Provided by the user       |
      | Defines custom type rules  |
      +-------------+--------------+
                    |
                    | passed into parser
                    v
      +----------------------------+
      |  Parser integrates mapping |
      |  - DB types → user types   |
      |  - Mapping changes force   |
      |    regeneration            |
      +-------------+--------------+
                    |
                    v
      +----------------------------+
      |     generator/             |
      |----------------------------|
      | template_engine            |
      | - receives ADR + CacheDiff |
      | - loads templates          |
      | - generates code only for  |
      |   changed entities         |
      | - language-agnostic        |
      +-------------+--------------+
                    |
                    v
      +----------------------------+
      |   Output / Filesystem      |
      |   (code generation output) |
      +----------------------------+


```

## main.rs

- Parses CLI arguments
-  Initializes logging

- Calls:
   - DB schema parser
   - Cache system
   - Generator
     
Passes user options (cache mode, output directory, etc.)

## db/ – Database Layer

- Responsible for:
   - Connecting to PostgreSQL
   - Extracting schema metadata 
   - Converting raw DB metadata to ADR
   - Applying user type mapping (not yet implemented)

- ADR (Abstract Database Representation) is the the central data structure:
   - AbstractDbRepr
     Currently supports tables and view. Planned is to implement stored procedures, triggers and so on too.
   - AbstractTableRepr
   - AbstractAttribute

## User Type Mapping JSON

- This is a planned feature but already architecturally important.
   - User provides a JSON file describing how DB types map to code types
   - Parser gets this mapping
   - Parser fits ADR attributes accordingly with the code types
   - If mapping changes → cache invalidation → regeneration

This is a powerful design because it makes Carpathia language‑agnostic. Use --print-db-types to get a JSON printed to STDOUT with the datatypes carpathia discovered.

## cache/ – Change Detection
- Stores per‑table hashes
- Compares old vs. new ADR
- Produces CacheDiff:
   - list of changed tables
   - list of removed tables

This is passed to the generator so it only regenerates what is necessary.

## generator/ – Template Engine

Not implemented yet - will not take place before ADR is settled and done. There will be a CLI switch to generate the intial templates. A planned structure could be:
- tables.rs.tera
- views.rs.tera
- procs.rs.tera
- triggers.rs.tera

To the naming: 
- First part: tables, views, triggers and so on are object types ADR provides, It will be replaced with the object the file represents after generation e.g. films.
- Second part: rs stands for the suffix of the file after generation-
- Third pard: it is a tera template.
To complete the example, after generation one would have a file named films.rs.

- Receives:
   - ADR
   - CacheDiff

- Loads templates
   - Generates code only for changed entities
   - Can output DB types as JSON for debugging
   - Language‑agnostic (Rust, Go, TS, SQL, …)

# Testing and development

For development I used the fine [pagila](https://github.com/devrimgunduz/pagila/blob/master/pagila-schema.sql) schema. As it should ever come to a MySQL/MariaDB implementation I propose the [sakila](https://dev.mysql.com/doc/sakila/en/) sample database. 

# CLI 

Note it is under development. At the moment it looks like this

```
It generates code for database access based on a given schema. You write the templates - it genrates the code. Note: It is still in early development and not functional yet.

Usage: carpathia [OPTIONS] --db-url <DB_URL> --db-name <DB_NAME>

Options:
      --db-url <DB_URL>
          Database URL in the format - JUST host and port NOT the database name: <postgres://user:password@localhost:5432>

      --db-name <DB_NAME>
          Database name you would like to generate code for - just the name NOT the full URL: `my_database`

      --cache-modus <CACHE_MODUS>
          Forces the generator to overwirite existing files allthough the database schema has not changed. Use this option if you want to update the generated code to the latest version of the generator
          
          [default: use-cache]
          [possible values: bypass-cache, use-cache]

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
