use clap::ValueEnum;
use sqlx::{Pool, Postgres};
use std::{fmt::Display, str::FromStr};
#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CacheModus {
    BypassCache,
    UseCache,
}

/// Implements the database type.
/// The enum is used to assemble the url to connect to the database.
///
/// - From for i32 is used for the default port
/// - Display for DbType is used as the protocol to connect to the database.
#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DbType {
    Postgres,
    Dummy,
}

/// Implements the default port per database type
impl From<DbType> for i32 {
    fn from(value: DbType) -> i32 {
        match value {
            DbType::Postgres => 5432,
            DbType::Dummy => -1,
        }
    }
}

// only postgre currently supported
impl FromStr for DbType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "postgres" => Ok(DbType::Postgres),
            _ => Ok(DbType::Dummy),
        }
    }
}
/// Implements the "protocol" per database type.
impl Display for DbType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbType::Postgres => write!(f, "postgres"),
            DbType::Dummy => write!(f, "dummy"),
        }
    }
}

pub enum DbPool {
    Postgres(Pool<Postgres>),
    Dummy,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_type_to_string() {
        assert_eq!(format!("{}", DbType::Postgres), "postgres");
        assert_eq!(format!("{}", DbType::Dummy), "dummy");
    }
    #[test]
    fn test_db_type_to_int() {
        assert_eq!(i32::from(DbType::Postgres), 5432);
        assert_eq!(i32::from(DbType::Dummy), -1);
    }
    #[test]
    fn test_string_do_db_type() {
        let pg: DbType = "postgres".parse().unwrap();
        assert!(pg == DbType::Postgres);
        let dummy: DbType = "rubbish".parse().unwrap();
        assert!(dummy == DbType::Dummy);
    }
}
