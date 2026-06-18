 
# carpathia-core — The Engine Behind Code Generation

> A reusable Rust library for parsing PostgreSQL schemas and generating code via Tera templates.

`carpathia-core` is the brain of the `carpathia-cli` tool. It provides a robust, testable, and extensible API to:

- Connect to PostgreSQL and extract schema metadata
- Represent the schema as a canonical `AbstractDbRepr` (ADR)
- Compare schema versions using cryptographic hashes (Blake3)
- Render templates with full context (tables, views, attributes)
- Support custom type mappings and caching

Use it directly in your own tools, CI pipelines, or build systems — no CLI required.

---

## ✅ Features

- ✅ **Schema Extraction**: Full PostgreSQL schema parsing (tables, views, constraints, comments, arrays)
- ✅ **Canonical Representation**: `AbstractDbRepr` as a stable contract between parser and template engine
- ✅ **Intelligent Caching**: Skip regeneration using file and schema hashes
- ✅ **Template Engine**: Integrates with [Tera](https://tera.netlify.app/) for flexible code generation
- ✅ **Type Mapping**: Map PostgreSQL types (`text`, `uuid`, etc.) to custom Rust types via JSON
- ✅ **Extensible**: Add support for new database types via `DatabaseQuerier` trait
- ✅ **Zero Runtime Dependencies**: Only `sqlx`, `serde`, `tera`, `blake3`, and `log`

---

## 📦 Usage as a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
carpathia-core = { git = "https://github.com/yourusername/carpathia" }
Example: Generate Code Programmatically
rust
use carpathia_core::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CarpathiaConfigBuilder::new()
        .db_host("localhost")
        .db_port(5432)
        .db_user("postgres")
        .db_password("postgres")
        .db_name("carpathia")
        .db_type(DbType::Postgres)
        .cache_modus(CacheModus::UseCache)
        .template_directory(Path::new("./templates/rust_lib"))
        .output_directory(Path::new("./generated"))
        .carpathia_type_mapping("carpathia_type_mapping.json")
        .build()?;

    let schema = DbSchemaParser::parse_schema(&config).await?;
    TemplateEngine::generate_code(&config, &schema).await?;

    println!("✅ Code generation completed!");
    Ok(())
}
🧩 Customizing the Generator
You can extend carpathia-core by:

Implementing DatabaseQuerier for MySQL or SQLite
Adding new TemplateType variants (e.g., Documentation, DTO)
Enhancing TypeMapping with custom imports or macros
🧠 Core Concepts
AbstractDbRepr — The Schema Contract
All schema data is represented as AbstractDbRepr, a deterministic, serializable structure:

rust
pub struct AbstractDbRepr {
    pub version: String,        // ADR version
    pub tables: BTreeMap<String, AbstractTableRepr>,
    pub views: BTreeMap<String, AbstractTableRepr>,
}

pub struct AbstractTableRepr {
    pub table_name: String,
    pub object_type: ObjectType,
    pub attributes: BTreeMap<String, AbstractAttribute>,
    pub u_imports: BTreeSet<String>, // Custom Rust imports per table
}
This structure is exactly what your Tera templates receive — no surprises.

CacheFile — Smart Regeneration
The cache (carpathia_cache.json) stores hashes of:

Each table’s schema
Each view’s schema
Each template file’s content
Only when any of these change is regeneration triggered.

📂 Template Engine Integration
The TemplateEngine supports three template types:

Type  Context Output
tables.*.tera table: AbstractTableRepr  One file per table
views.*.tera  table: AbstractTableRepr  One file per view
summary.*.tera  tables: Vec<...>, views: Vec<...> One summary file (e.g., mod.rs)
Use {{ table.column_name }}, {{ table.u_type }}, {{ table.comment }} — all fields are accessible.

🔧 Configuration
Use CarpathiaConfigBuilder to build your config:

rust
let config = CarpathiaConfigBuilder::new()
    .db_host("localhost")
    .db_port(5432)
    .db_user("postgres")
    .db_password("secret")
    .db_name("mydb")
    .db_type(DbType::Postgres)
    .cache_modus(CacheModus::BypassCache)
    .template_directory("./templates")
    .output_directory("./src/generated")
    .build()?;
🧪 Testing
All components are thoroughly tested. Run:

bash
cargo test -- --test-threads=1
Includes integration tests using the Pagila schema.

📜 License
MIT — See LICENSE for details.

💬 Support & Contributions
Questions? Ideas? Bugs?
Open an issue or PR on GitHub.

“Decouple schema from code. Let templates do the work.” — carpathia-core
