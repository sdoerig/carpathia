# carpathia-cli — Generate Code from PostgreSQL Schemas

> **Write templates. Generate code. Never write boilerplate again.**

`carpathia` is a command-line tool that connects to a PostgreSQL database, extracts its schema (tables, views, materialized views, columns, constraints), and generates code from Tera templates you define. It is language-agnostic: the engine renders whatever your templates describe — Rust, SQL, TypeScript, documentation, anything.

It is **not an ORM** and never will be. carpathia is declarative: you decide what gets generated and what it looks like.

A built-in hash-based cache makes regeneration fast — only entities whose schema or template changed are re-rendered, and rendered files whose template or database object disappeared are cleaned up. Generated output is deterministic, so it checks cleanly into git.

Currently supported: PostgreSQL (MySQL and SQLite are planned). Functional but in beta — use it, test it, and help shape its future! 🚀

## 📦 The carpathia Workspace


| Crate                                                                                       | Role                                                                              |
| ------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| [`carpathia-cli`](https://github.com/sdoerig/carpathia/tree/main/carpathia-cli)             | Command-line tool (this crate, binary name `carpathia`)                           |
| [`carpathia-core`](https://github.com/sdoerig/carpathia/tree/main/carpathia-core)           | Reusable engine: introspection, caching, template execution                       |
| [`carpathia-adr`](https://github.com/sdoerig/carpathia/tree/main/carpathia-adr)             | The Abstract Database Representation — the contract your templates render against |
| [`carpathia-templates`](https://github.com/sdoerig/carpathia/tree/main/carpathia-templates) | Bundled template packs (e.g. the sqlx-based `rust_lib` pack)                      |


Prefer a library? Use `carpathia-core` directly in your build scripts or CI pipelines.

## 🚀 Quick Start

### 1. Install

Build from source:

```bash
git clone https://github.com/sdoerig/carpathia.git
cd carpathia
cargo build --release
# binary: target/release/carpathia
```

You'll also need a PostgreSQL server with a schema to generate from — the [Pagila](https://github.com/devrimgunduz/pagila-src) sample database works great.

### 2. Unpack a Template Pack (Optional)

You don't have to write templates from scratch — carpathia ships an example pack for a sqlx-based Rust data-access library:

```bash
carpathia --init-template rust-lib --template-directory ./templates
```

The unpacked templates are now **yours**: edit them, extend them, or replace them entirely. For the template naming conventions and the data available inside templates, see the [`carpathia-adr`](https://github.com/sdoerig/carpathia/tree/main/carpathia-adr) README.

### 3. Discover Your Types

Generate a skeleton type-mapping file containing every type your schema uses:

```bash
carpathia execute \
    --db-host localhost --db-port 5432 \
    --db-username postgres --db-password postgres \
    --db-name pagila \
    --print-db-types > carpathia_type_mapping.json
```

Fill in the `u_type` (your target type) and `u_import` (the import it needs) pairs:

```json
{
  "version": "0.1.0",
  "type_mapping": {
    "integer": { "u_import": "", "u_type": "i32" },
    "boolean": { "u_import": "", "u_type": "bool" },
    "timestamp with time zone": { "u_import": "chrono::{DateTime, Utc}", "u_type": "DateTime<Utc>" }
  },
  "db_to_code_names_mapping": {
    "match": "match_table"
  }
}
```

`db_to_code_names_mapping` renames database identifiers to language-safe names — every table, column, and constraint gets a `u_`-prefixed counterpart your templates can use.

### 4. Inspect the Template Contract (Optional)

Not sure what data your templates receive? Print it:

```bash
# what templates see (--print-schema)
carpathia execute --db-host localhost --db-port 5432 \
    --db-username postgres --db-password postgres --db-name pagila \
    --print-schema

# the full internal representation, for debugging (--print-internal-schema)
carpathia execute --db-host localhost --db-port 5432 \
    --db-username postgres --db-password postgres --db-name pagila \
    --print-internal-schema
```

The printed structure is a contract: any field prefixed with `u_` is one you define via the type mapping file. Use them or ignore them — up to you.

### 5. Generate

```bash
carpathia execute \
    --db-host localhost --db-port 5432 \
    --db-username postgres --db-password postgres \
    --db-name pagila \
    --template-directory ./templates \
    --output-directory ./generated
```

A minimal example template (`templates/tables.rs.tera`), rendered once per table:

```jinja
#[derive(Debug, Clone)]
pub struct {{ table.u_table_name | capitalize }} {
    {%- for attr in table.attributes %}
    pub {{ attr.u_column_name }}: {{ attr.u_type }},
    {%- endfor %}
}
```

Run it again after changing the schema or a template: only what changed is regenerated. Use `--cache-modus bypass-cache` to force regeneration of everything.

## 📖 Reference

### Global Options

```
carpathia [OPTIONS] [COMMAND]

--output-directory <DIR>          where generated code goes [default: ./generated_files]
--template-directory <DIR>        where your Tera templates live [default: ./tera/rust_lib]
--carpathia-type-mapping-file <FILE>  type mapping JSON [default: carpathia_type_mapping.json]
--cache-file <FILE>               hash cache location [default: ./carpathia_cache.json]
--init-template <TEMPLATE>        unpack a bundled template pack [rust-lib, none]
-h, --help                        print help
-V, --version                     print version
```

### `execute` Subcommand

```
carpathia execute [OPTIONS] --db-host <HOST> --db-port <PORT> \
    --db-username <USER> --db-password <PASSWORD> --db-name <NAME>

--db-host <HOST>           database host
--db-port <PORT>           database port
--db-username <USER>       database user — read-only access is sufficient
--db-password <PASSWORD>   database password
--db-name <NAME>           database to generate code for
--db-type <TYPE>           currently only `postgres` [default: postgres]
--cache-modus <MODUS>      `use-cache` (default) or `bypass-cache`
--print-schema             print the JSON your templates receive, then exit
--print-internal-schema    print the internal representation (debugging), then exit
--print-db-types           print a skeleton type-mapping file, then exit
```

## 📜 License

Licensed under [Apache-2.0](./LICENSE-APACHE) and/or [MIT](./LICENSE-MIT). 

## 🤝 Contributing

Contributions follow the [Developer Certificate of Origin](https://github.com/sdoerig/carpathia/blob/main/DCO.md) (DCO 1.1). Sign off your commits with:

```bash
git commit -s -m "Your commit message"
```