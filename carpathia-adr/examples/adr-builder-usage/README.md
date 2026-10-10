# Builder API Usage Example

This example shows how to build an **ADR** (Abstract Database Representation) using the **`AbstractDbReprBuilder`** and then enrich it with a user type mapping. The full code can be found in `main.rs`.

> **Note:** The builder API itself is AI-generated from the ADR structs.

## Overview

Compared to constructing the ADR manually (see the basic usage example), the builder API turns the whole schema into a **single declarative expression**: tables, attributes, and constraints are declared directly where they belong — no `get_mut`-chains, no `if let Some(attr)` helper functions.

## What This Example Does

1. Builds a `users` table (version `0.1.0`) with the columns `id`, `name`, and `email`, including comments, defaults, and nullability — all in one fluent builder chain.
2. Declares a **primary key** constraint (`users_pkey`) directly on the `id` attribute.
3. Declares a **foreign key** constraint (`email_fkey`) on `email`, referencing `ldap_users(email)`.
4. Applies a user type mapping via `add_user_mapping_to_adr`, which maps database names and types to user-defined, code-safe names (e.g. `integer` → `MyInteger`, `users` → `My_Code_Safe_User`) and collects the required imports.
5. Converts the ADR into `AdrTemplateData` — the external representation used by the Tera templates.
6. Asserts that the serialized ADR matches the expected JSON representation.

## Notes on `u_type` Values

The `u_` fields (user-defined, code-safe counterparts of database identifiers such as table names, column names, and types) are **set automatically by the builder** — you never have to fill them in by hand. Any `u_type`-style values present during building are merely placeholders; the real mapping is applied afterwards by `add_user_mapping_to_adr`. The mapping configuration (`TypeMapping` / `Types`) holds database-to-code type mappings (with imports) and database-to-code name mappings.

## Running

Run the example like any other binary example in the workspace (e.g. via `cargo run --example <name>`); it prints the ADR before and after the user mapping step, the resulting template data, and validates the round-trip against the expected JSON.