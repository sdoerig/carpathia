use std::str::FromStr;

use log::debug;

use crate::db::db_schema_structs::{
    ConstraintType, IsGenerated, IsIdentity, IsNullable, ObjectType,
};

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub enum PgObjectType {
    BaseTable,
    PartitionedTable,
    View,
    MaterializedView,
    Unknown(String),
}

impl FromStr for PgObjectType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "base table" => Ok(PgObjectType::BaseTable),
            "partitioned table" => Ok(PgObjectType::PartitionedTable),
            "view" => Ok(PgObjectType::View),
            "materialized view" => Ok(PgObjectType::MaterializedView),
            _ => {
                debug!("Invalid object type: {}", s);
                Ok(PgObjectType::Unknown(s.to_string()))
            }
        }
    }
}

impl From<&PgObjectType> for ObjectType {
    fn from(pg_object_type: &PgObjectType) -> Self {
        match pg_object_type {
            PgObjectType::BaseTable => ObjectType::BaseTable,
            PgObjectType::PartitionedTable => ObjectType::PartitionedTable,
            PgObjectType::View => ObjectType::View,
            PgObjectType::MaterializedView => ObjectType::MaterializedView,
            PgObjectType::Unknown(s) => ObjectType::Unknown(s.to_owned()),
        }
    }
}

impl From<PgObjectType> for ObjectType {
    fn from(pg_object_type: PgObjectType) -> Self {
        (&pg_object_type).into()
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum PgConstraintType {
    PrimaryKey,
    ForeignKey,
    Unique,
    Check,
    Exclusion,
    NotNull,
    ConstraintTrigger,
    None,
    Unknown(String),
}

impl From<&PgConstraintType> for ConstraintType {
    fn from(pg_constraint_type: &PgConstraintType) -> Self {
        match pg_constraint_type {
            PgConstraintType::PrimaryKey => ConstraintType::PrimaryKey,
            PgConstraintType::ForeignKey => ConstraintType::ForeignKey,
            PgConstraintType::Unique => ConstraintType::Unique,
            PgConstraintType::Check => ConstraintType::Check,
            PgConstraintType::Exclusion => ConstraintType::Exclusion,
            PgConstraintType::NotNull => ConstraintType::NotNull,
            PgConstraintType::ConstraintTrigger => ConstraintType::ConstraintTrigger,
            PgConstraintType::None => ConstraintType::None,
            PgConstraintType::Unknown(_) => ConstraintType::Unknown,
        }
    }
}

impl From<PgConstraintType> for ConstraintType {
    fn from(pg_constraint_type: PgConstraintType) -> Self {
        (&pg_constraint_type).into()
    }
}

impl FromStr for PgConstraintType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "p" | "primary key" => Ok(PgConstraintType::PrimaryKey),
            "u" | "unique" => Ok(PgConstraintType::Unique),
            "f" | "foreign key" => Ok(PgConstraintType::ForeignKey),
            "c" | "check" => Ok(PgConstraintType::Check),
            "x" | "exclusion" => Ok(PgConstraintType::Exclusion),
            "n" | "not null" => Ok(PgConstraintType::NotNull),
            "t" | "constraint trigger" => Ok(PgConstraintType::ConstraintTrigger),
            "" | "none" => Ok(PgConstraintType::None),
            _ => {
                debug!("Invalid constraint type: {}", s);
                Ok(PgConstraintType::Unknown(s.to_string()))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PgIsNullable {
    Yes,
    No,
    Unknown(String),
}

impl FromStr for PgIsNullable {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "yes" => Ok(PgIsNullable::Yes),
            "no" => Ok(PgIsNullable::No),
            _ => {
                debug!("Invalid value for is_nullable: {}", s);
                Ok(PgIsNullable::Unknown(s.to_string()))
            }
        }
    }
}

impl From<&PgIsNullable> for IsNullable {
    fn from(pg_is_nullable: &PgIsNullable) -> Self {
        match pg_is_nullable {
            PgIsNullable::Yes => IsNullable::Yes,
            PgIsNullable::No => IsNullable::No,
            PgIsNullable::Unknown(s) => IsNullable::Unknown(s.to_owned()),
        }
    }
}

impl From<PgIsNullable> for IsNullable {
    fn from(pg_is_nullable: PgIsNullable) -> Self {
        (&pg_is_nullable).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PgIsIdentity {
    Yes,
    No,
    Unknown(String),
}

impl FromStr for PgIsIdentity {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "yes" => Ok(PgIsIdentity::Yes),
            "no" => Ok(PgIsIdentity::No),
            _ => {
                debug!("Invalid value for is_identity: {}", s);
                Ok(PgIsIdentity::Unknown(s.to_string()))
            }
        }
    }
}

impl From<&PgIsIdentity> for IsIdentity {
    fn from(pg_is_identity: &PgIsIdentity) -> Self {
        match pg_is_identity {
            PgIsIdentity::Yes => IsIdentity::Yes,
            PgIsIdentity::No => IsIdentity::No,
            PgIsIdentity::Unknown(s) => IsIdentity::Unknown(s.to_owned()),
        }
    }
}

impl From<PgIsIdentity> for IsIdentity {
    fn from(pg_is_identity: PgIsIdentity) -> Self {
        (&pg_is_identity).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PgIsGenerated {
    Always,
    ByDefault,
    ByDefaultOnNull,
    Never,
    Unknown(String),
}

impl FromStr for PgIsGenerated {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "always" => Ok(PgIsGenerated::Always),
            "by default" => Ok(PgIsGenerated::ByDefault),
            "by default on null" => Ok(PgIsGenerated::ByDefaultOnNull),
            "never" => Ok(PgIsGenerated::Never),
            _ => {
                debug!("Invalid value for is_generated: {}", s);
                Ok(PgIsGenerated::Unknown(s.to_string()))
            }
        }
    }
}

impl From<&PgIsGenerated> for IsGenerated {
    fn from(pg_is_generated: &PgIsGenerated) -> Self {
        match pg_is_generated {
            PgIsGenerated::Always => IsGenerated::Always,
            PgIsGenerated::ByDefault => IsGenerated::ByDefault,
            PgIsGenerated::ByDefaultOnNull => IsGenerated::ByDefaultOnNull,
            PgIsGenerated::Never => IsGenerated::Never,
            PgIsGenerated::Unknown(s) => IsGenerated::Unknown(s.to_owned()),
        }
    }
}

impl From<PgIsGenerated> for IsGenerated {
    fn from(pg_is_generated: PgIsGenerated) -> Self {
        (&pg_is_generated).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EnumTest<P, I> {
        // parsing from this string...
        parsed_from: String,
        // must become this enum value...
        pg_const_type: P,
        // must cast into this ADR enum value.
        const_type: I,
    }

    #[test]
    fn test_pg_object_type_from_str() {
        let test_cases = [
            EnumTest {
                parsed_from: "BASE TABLE".to_string(),
                pg_const_type: PgObjectType::BaseTable,
                const_type: ObjectType::BaseTable,
            },
            EnumTest {
                parsed_from: "PARTITIONED TABLE".to_string(),
                pg_const_type: PgObjectType::PartitionedTable,
                const_type: ObjectType::PartitionedTable,
            },
            EnumTest {
                parsed_from: "VIEW".to_string(),
                pg_const_type: PgObjectType::View,
                const_type: ObjectType::View,
            },
            EnumTest {
                parsed_from: "MATERIALIZED VIEW".to_string(),
                pg_const_type: PgObjectType::MaterializedView,
                const_type: ObjectType::MaterializedView,
            },
            EnumTest {
                parsed_from: "UNKNOWN".to_string(),
                pg_const_type: PgObjectType::Unknown("UNKNOWN".to_string()),
                const_type: ObjectType::Unknown("UNKNOWN".to_string()),
            },
        ];
        for enum_test in test_cases.iter() {
            let pg_object_type: PgObjectType = enum_test.parsed_from.parse().unwrap();
            assert_eq!(pg_object_type, enum_test.pg_const_type);
            let const_type: ObjectType = (&pg_object_type).into();
            assert_eq!(const_type, enum_test.const_type);
        }
    }

    #[test]
    fn test_constraint_type_from_str_and_to_constraint_type() {
        let enum_variants = [
            EnumTest {
                parsed_from: "PRIMARY KEY".to_string(),
                pg_const_type: PgConstraintType::PrimaryKey,
                const_type: ConstraintType::PrimaryKey,
            },
            EnumTest {
                parsed_from: "UNIQUE".to_string(),
                pg_const_type: PgConstraintType::Unique,
                const_type: ConstraintType::Unique,
            },
            EnumTest {
                parsed_from: "FOREIGN KEY".to_string(),
                pg_const_type: PgConstraintType::ForeignKey,
                const_type: ConstraintType::ForeignKey,
            },
            EnumTest {
                parsed_from: "CHECK".to_string(),
                pg_const_type: PgConstraintType::Check,
                const_type: ConstraintType::Check,
            },
            EnumTest {
                parsed_from: "EXCLUSION".to_string(),
                pg_const_type: PgConstraintType::Exclusion,
                const_type: ConstraintType::Exclusion,
            },
            EnumTest {
                parsed_from: "NOT NULL".to_string(),
                pg_const_type: PgConstraintType::NotNull,
                const_type: ConstraintType::NotNull,
            },
            EnumTest {
                parsed_from: "CONSTRAINT TRIGGER".to_string(),
                pg_const_type: PgConstraintType::ConstraintTrigger,
                const_type: ConstraintType::ConstraintTrigger,
            },
            EnumTest {
                parsed_from: String::new(),
                pg_const_type: PgConstraintType::None,
                const_type: ConstraintType::None,
            },
            EnumTest {
                parsed_from: "NONE".to_string(),
                pg_const_type: PgConstraintType::None,
                const_type: ConstraintType::None,
            },
            EnumTest {
                parsed_from: "UNKNOWN".to_string(),
                pg_const_type: PgConstraintType::Unknown("UNKNOWN".to_string()),
                const_type: ConstraintType::Unknown,
            },
        ];
        for enum_test in enum_variants.iter() {
            let pg_const_type: PgConstraintType = enum_test.parsed_from.parse().unwrap();
            assert_eq!(pg_const_type, enum_test.pg_const_type);
            let const_type: ConstraintType = pg_const_type.into();
            assert_eq!(const_type, enum_test.const_type);
        }
    }

    #[test]
    fn test_pg_const_type_from_str_and_to_is_nullable() {
        let enum_variants = [
            EnumTest {
                parsed_from: "YES".to_string(),
                pg_const_type: PgIsNullable::Yes,
                const_type: IsNullable::Yes,
            },
            EnumTest {
                parsed_from: "NO".to_string(),
                pg_const_type: PgIsNullable::No,
                const_type: IsNullable::No,
            },
            EnumTest {
                parsed_from: "UNKNOWN".to_string(),
                pg_const_type: PgIsNullable::Unknown("UNKNOWN".to_string()),
                const_type: IsNullable::Unknown("UNKNOWN".to_string()),
            },
        ];
        for enum_test in enum_variants.iter() {
            let pg_const_type: PgIsNullable = enum_test.parsed_from.parse().unwrap();
            assert_eq!(pg_const_type, enum_test.pg_const_type);
            let const_type: IsNullable = pg_const_type.into();
            assert_eq!(const_type, enum_test.const_type);
        }
    }

    #[test]
    fn test_pg_is_identity_from_str_and_to_is_identity() {
        let enum_variants = [
            EnumTest {
                parsed_from: "YES".to_string(),
                pg_const_type: PgIsIdentity::Yes,
                const_type: IsIdentity::Yes,
            },
            EnumTest {
                parsed_from: "NO".to_string(),
                pg_const_type: PgIsIdentity::No,
                const_type: IsIdentity::No,
            },
            EnumTest {
                parsed_from: "UNKNOWN".to_string(),
                pg_const_type: PgIsIdentity::Unknown("UNKNOWN".to_string()),
                const_type: IsIdentity::Unknown("UNKNOWN".to_string()),
            },
        ];
        for enum_test in enum_variants.iter() {
            let pg_const_type: PgIsIdentity = enum_test.parsed_from.parse().unwrap();
            assert_eq!(pg_const_type, enum_test.pg_const_type);
            let const_type: IsIdentity = pg_const_type.into();
            assert_eq!(const_type, enum_test.const_type);
        }
    }

    #[test]
    fn test_pg_is_generated_from_str_and_to_is_generated() {
        let enum_variants = [
            EnumTest {
                parsed_from: "ALWAYS".to_string(),
                pg_const_type: PgIsGenerated::Always,
                const_type: IsGenerated::Always,
            },
            EnumTest {
                parsed_from: "BY DEFAULT".to_string(),
                pg_const_type: PgIsGenerated::ByDefault,
                const_type: IsGenerated::ByDefault,
            },
            EnumTest {
                parsed_from: "BY DEFAULT ON NULL".to_string(),
                pg_const_type: PgIsGenerated::ByDefaultOnNull,
                const_type: IsGenerated::ByDefaultOnNull,
            },
            EnumTest {
                parsed_from: "NEVER".to_string(),
                pg_const_type: PgIsGenerated::Never,
                const_type: IsGenerated::Never,
            },
            EnumTest {
                parsed_from: "UNKNOWN".to_string(),
                pg_const_type: PgIsGenerated::Unknown("UNKNOWN".to_string()),
                const_type: IsGenerated::Unknown("UNKNOWN".to_string()),
            },
        ];
        for enum_test in enum_variants.iter() {
            let pg_const_type: PgIsGenerated = enum_test.parsed_from.parse().unwrap();
            assert_eq!(pg_const_type, enum_test.pg_const_type);
            let const_type: IsGenerated = pg_const_type.into();
            assert_eq!(const_type, enum_test.const_type);
        }
    }
}
