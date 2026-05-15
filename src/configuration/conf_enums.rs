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
    Dummy,
}
