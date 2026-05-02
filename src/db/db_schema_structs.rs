// This module defines the intermediate database schema representation that will be used by the schema parser and the code generator. The AbstractDbRepr struct represents a database table, while the AbstractAttribute struct represents a column in a table. The DbType enum represents the supported database types, which can be extended in the future to support more databases.
#[derive(sqlx::FromRow, serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct AbstractDbRepr {
    pub table_name: String,
    pub attributes: Vec<AbstractAttribute>,
}
// This module defines the intermediate database attribute representation.
#[derive(sqlx::FromRow, serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct AbstractAttribute {
    pub column_name: String,
    pub data_type: String,
    pub is_nullable: String,
    pub column_default: Option<String>,
    pub character_maximum_length: Option<i32>,
    pub numeric_precision: Option<i32>,
    pub numeric_scale: Option<i32>,
    pub is_identity: String,
    pub identity_generation: Option<String>,
    pub is_generated: String,
    pub generation_expression: Option<String>,
    pub constraint_name: Option<String>,
    pub constraint_type: Option<String>,
    pub referenced_table: Option<String>,
    pub referenced_column: Option<String>,
}
// This enum represents the supported database types. Currently, only PostgreSQL is supported, but we can easily add support for MySQL and SQLite in the future by adding new variants to this enum and implementing the necessary logic in the database querier and schema parser.
pub(crate) enum DbType {
    Postgres,
    MySql,  // Future support for MySQL
    Sqlite, // Future support for SQLite
}
