# carpathia-templates — Template Packs for carpathia

> Bundled [Tera](https://github.com/Keats/tera) template packs, embedded into the carpathia binaries. Unpack them, edit them, make them yours.

`carpathia` generates code by rendering Tera templates against the Abstract Database Representation (ADR). This crate ships the **ready-made template packs** that come with carpathia out of the box — so you can start generating code instead of writing templates from scratch.

## Why a separate crate?

Template packs are **content, not engine**. They evolve on their own schedule:

- A template pack is a *suggestion*, not a requirement — you are free to write your own templates from scratch.
- If you find a bug in the bundled templates, you don't need the whole engine to fix or fork them. An own crate keeps templates independently versionable and replaceable.
- New packs (other languages, other output styles) can be added without touching `carpathia-core`.

## What's inside

```rust
/// The embedded `rust_lib` template pack (tar.gz bytes).
pub const RUST_LIB: &[u8] = include_bytes!("../ressources_archive/tera/rust_lib.tar.gz");
```

The crate is intentionally tiny: zero runtime dependencies, one embedded archive per template pack. Each pack is a `tar.gz` that gets unpacked into your template directory on first use.

### The `rust_lib` pack

A starting point for generating a type-safe **sqlx-based Rust data-access library** from your schema:


| Template              | Renders                                                                                                                                    |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `tables.rs.tera`      | One file per table: a `FromRow` struct, forward-relation helpers following foreign keys (including composite keys), and an `update` method |
| `views.rs.tera`       | One file per view: a `FromRow` struct plus a builder-style `Filter` with `fetch_all` for dynamic `WHERE` clauses                           |
| `summary.mod.rs.tera` | One `mod.rs` declaring a module per generated table and view                                                                               |


Generated code uses `sqlx` (`QueryBuilder`, `FromRow`, `PgPool`), respects your type mappings (`u_type`, `u_imports`), and handles composite primary and foreign keys.

## Usage

### Via carpathia (recommended)

You normally don't use this crate directly. Ask carpathia to unpack a template pack for you — via the CLI or `carpathia-core`:

```rust
use carpathia_core::configuration::carpathia_conf::CarpathiaConfigBuilder;
use carpathia_core::templates::enum_templates::InitTemplate;
use carpathia_core::templates::init_templates::extract_to_disk;

let config = CarpathiaConfigBuilder::new()
    // ... database settings ...
    .init_template(InitTemplate::RustLib)
    .template_directory("./templates")
    .build()?;

extract_to_disk(&config)?;
```

From that moment on, the templates in `./templates` are **yours**: carpathia does not re-unpack over your edits. Tweak the pack, replace it file by file, or discard it and write your own — the engine only cares about the [naming conventions](#template-naming-conventions).

### As a dependency

If you build your own tooling on top of carpathia, you can depend on the crate and unpack the pack yourself:

```rust
use carpathia_templates::RUST_LIB;

// RUST_LIB is a tar.gz byte slice — unpack it with flate2 + tar
```

## Template naming conventions

The template engine in `carpathia-core` dispatches by file-name prefix:


| Prefix           | Context                    | Output                   |
| ---------------- | -------------------------- | ------------------------ |
| `tables.*.tera`  | one `table` per rendering  | one file per table       |
| `views.*.tera`   | one `view` per rendering   | one file per view        |
| `summary.*.tera` | `tables` and `views` lists | one file (e.g. `mod.rs`) |


The data available inside templates is defined by the [`carpathia-adr`](https://github.com/sdoerig/carpathia/tree/main/carpathia-adr) crate — the contract between your templates and carpathia.

## Layout &amp; Maintenance

```
carpathia-templates/
├── ressources/tera/rust_lib/       # editable template sources
│   ├── tables.rs.tera
│   ├── views.rs.tera
│   └── summary.mod.rs.tera
├── ressources_archive/tera/       # prebuilt tar.gz, embedded via include_bytes!
│   └── rust_lib.tar.gz
└── src/lib.rs                     # re-exports RUST_LIB
```

To change a bundled template: edit the source under `ressources/tera/rust_lib/`, re-create the `tar.gz` in `ressources_archive/tera/`, and bump the crate version. The archive is embedded at compile time — no runtime file access needed.

## Status

This crate is part of the [carpathia](https://github.com/sdoerig/carpathia) workspace and follows its alpha status. The bundled packs are a starting point, not a polished product — expect them to evolve with the ADR contract.

## License

Licensed under [Apache-2.0](./LICENSE-APACHE) and/or [MIT](./LICENSE-MIT). 

## Contributing

Contributions follow the [Developer Certificate of Origin](https://github.com/sdoerig/carpathia/blob/main/DCO.md) (DCO 1.1). Sign off your commits with:

```bash
git commit -s -m "Your commit message"
```