use std::str::FromStr;

use log::debug;

use crate::db::db_schema_structs::{ConstraintType, ObjectType};

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub enum PgObjectType {
    BaseTable,
    PartitionedTable,
    View,
    MaterializedView,
    Other,
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
            PgObjectType::Other => ObjectType::Other,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pg_object_type_from_str() {
        let test_cases = vec![
            ("BASE TABLE", PgObjectType::BaseTable, ObjectType::BaseTable),
            (
                "PARTITIONED TABLE",
                PgObjectType::PartitionedTable,
                ObjectType::PartitionedTable,
            ),
            ("VIEW", PgObjectType::View, ObjectType::View),
            (
                "MATERIALIZED VIEW",
                PgObjectType::MaterializedView,
                ObjectType::MaterializedView,
            ),
            (
                "OTHER",
                PgObjectType::Unknown("OTHER".to_string()),
                ObjectType::Unknown("OTHER".to_string()),
            ),
        ];
        for (input, expected_pg, expected_adr) in test_cases {
            let result: PgObjectType = input.parse().unwrap();
            assert_eq!(result, expected_pg);
            let adr_result: ObjectType = result.into();
            assert_eq!(adr_result, expected_adr);
        }
    }

    #[test]
    fn test_constraint_type_from_str_and_to_constraint_type() {
        struct EnumTest {
            // parsing from this string...
            parsed_from: String,
            // must become this enum value...
            pg_const_type: PgConstraintType,
            // must cast into this ADR enum value.
            const_type: ConstraintType,
        }

        let enum_variants = vec![
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
}
