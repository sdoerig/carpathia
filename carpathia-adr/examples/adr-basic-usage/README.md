# Basic ADR Usage Example

This example demonstrates the basic usage of the **ADR** (Abstract Database Representation) by building it **manually** — the full code can be found in `main.rs`.

## Overview

The ADR is a database-agnostic, intermediate representation of a database schema (tables, views, columns, and constraints). Once built, it can be enriched with user-defined type and name mappings and finally converted into template-ready data.

## Manual Construction vs. Alternatives

Constructing the ADR manually, as done in this example, is one valid approach — but it is admittedly a bit tedious. There are several ways to build an ADR, so feel free to pick whichever suits you:

- **Builder API** — see `carpathia-adr/src/adr/abstract_db_repr_builder.rs`
- **`From` trait** — implemented for PostgreSQL; see [`carpathia-core/src/db/postgres/postgresql_structs.rs`](https://github.com/sdoerig/carpathia/blob/main/carpathia-core/src/db/postgres/postgresql_structs.rs)

## What This Example Does

1. Creates a `users` table with the columns `id`, `name`, and `email`, including their comments, defaults, and nullability.
2. Adds a **primary key** constraint (`users_pkey`) on `id`.
3. Adds a **foreign key** constraint (`email_fkey`) on `email`, referencing `ldap_users(email)`.
4. Applies a user type mapping via `add_user_mapping_to_adr`, which maps database names and types to user-defined, code-safe names (e.g. `integer` → `MyInteger`, `users` → `My_Code_Safe_User`) and collects the required imports.
5. Converts the ADR into `AdrTemplateData` — the external representation used by the Tera templates.
6. Asserts that the serialized ADR matches the expected JSON representation.

## Key Concepts

- **`u_` fields** — user-defined, code-safe counterparts of database identifiers (table names, column names, types). They are populated during the user mapping step and are what ends up in generated code.
- **`TypeMapping` / `Types`** — the mapping configuration holding database-to-code type mappings (with imports) and database-to-code name mappings.
- **`AdrTemplateData`** — the view of the ADR that template engines consume; created via `From<&AbstractDbRepr>`.

## Running

Run the example like any other binary example in the workspace (e.g. via `cargo run --example <name>`); it prints the ADR before and after the user mapping step, the resulting template data, and validates the round-trip against the expected JSON.