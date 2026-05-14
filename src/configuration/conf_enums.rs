use clap::ValueEnum;
use sqlx::{Pool, Postgres};
#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CacheModus {
    BypassCache,
    UseCache,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DbType {
    Postgres,
}

pub enum DbPool {
    Postgres(Pool<Postgres>),
    // Future support for MySQL and SQLite can be added here by adding new variants to this enum and implementing the necessary logic in the database querier and schema parser.
}
