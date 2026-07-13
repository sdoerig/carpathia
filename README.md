![Test Status](https://github.com/sdoerig/carpathia/actions/workflows/test.yml/badge.svg)

# carpathia — Generate Code from PostgreSQL Schemas

> **Write templates. Generate code. Never write boilerplate again.**

`carpathia` is a Rust-based toolkit for automatically generating type-safe database access code from PostgreSQL schemas using customizable Tera templates. It separates your schema definition from your application code — letting you evolve your database and code independently, safely and efficiently.

`carpathia` is not an ORM and it will never be one. It is just a template based code generator.
You decide what you generate and how it looks like. `capthia` only delivers you an Abstract Database
Representation (ADR). Against this you write your templates.

Built as a modular system with a CLI frontend (`carpathia-cli`) and a reusable core library (`carpathia-core`), it’s ideal for teams building production-grade Rust applications with PostgreSQL.

---

## ✅ Why carpathia?

- 🚫 **Just generate from your schema** — you decide what your output looks like.
- 🔁 **Smart caching** — only regenerate code when the schema or templates change.
- 🧩 **Full template control** — use [Tera](https://crates.io/crates/tera) to define exactly how your code looks.
- 📦 **Reusable core** — integrate code generation into your CI, build scripts, or custom tools.
- 🔄 **Type-safe mappings** — map `text` → `String`, `uuid` → `Uuid`, `jsonb` → `serde_json::Value`, etc.


## 📦 Components


| Component | Description |
| --------- | ----------- |
| `carpathia-cli` | Command-line tool for end-users. Run it manually or in CI to generate code. |
| `carpathia-core` | Reusable Rust library. Use it programmatically in your build scripts, CI pipelines, or custom tools. |

Both are published as separate crates and can be used independently.

# Concept

The concept is

- Database introspection
- Transform the schema into an Abstract Database Representation (ADR)
  - Enrich the ADR with the datatype mapping the user eventually provides
- Search for tera templates in the template direcory the user provides
  - Process each database object with the corresponding template.
  - Process only database object which have changed since the last run.
- Remove renderd output files, which
  - do not have a template representing them
  - have been removed from the database schema

# Announcement: Migration to Apache‑2.0 License

This project has been relicensed under the Apache‑2.0 license. As the sole contributor and with the project still in its alpha phase, this transition provides a more robust and widely compatible legal foundation for future development.

- All new releases will be published under Apache‑2.0.
- Existing versions remain available under their original license.

This change aims to support broader adoption and clearer integration with other Rust projects.
