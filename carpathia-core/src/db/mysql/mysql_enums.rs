use std::str::FromStr;

use log::debug;

use carpathia_adr::adr::abstract_db_repr::{
    ConstraintType, IsGenerated, IsIdentity, IsNullable, ObjectType,
};

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub enum MySqlObjectType {
    BaseTable,
    PartitionedTable,
    View,
    MaterializedView,
    Unknown(String),
}

impl FromStr for MySqlObjectType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "base table" => Ok(MySqlObjectType::BaseTable),
            "partitioned table" => Ok(MySqlObjectType::PartitionedTable),
            "view" => Ok(MySqlObjectType::View),
            "materialized view" => Ok(MySqlObjectType::MaterializedView),
            _ => {
                debug!("Invalid object type: {}", s);
                Ok(MySqlObjectType::Unknown(s.to_string()))
            }
        }
    }
}

impl From<&MySqlObjectType> for ObjectType {
    fn from(my_object_type: &MySqlObjectType) -> Self {
        match my_object_type {
            MySqlObjectType::BaseTable => ObjectType::BaseTable,
            MySqlObjectType::PartitionedTable => ObjectType::PartitionedTable,
            MySqlObjectType::View => ObjectType::View,
            MySqlObjectType::MaterializedView => ObjectType::MaterializedView,
            MySqlObjectType::Unknown(s) => ObjectType::Unknown(s.to_owned()),
        }
    }
}

impl From<MySqlObjectType> for ObjectType {
    fn from(my_object_type: MySqlObjectType) -> Self {
        (&my_object_type).into()
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum MySqlConstraintType {
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

impl From<&MySqlConstraintType> for ConstraintType {
    fn from(my_constraint_type: &MySqlConstraintType) -> Self {
        match my_constraint_type {
            MySqlConstraintType::PrimaryKey => ConstraintType::PrimaryKey,
            MySqlConstraintType::ForeignKey => ConstraintType::ForeignKey,
            MySqlConstraintType::Unique => ConstraintType::Unique,
            MySqlConstraintType::Check => ConstraintType::Check,
            MySqlConstraintType::Exclusion => ConstraintType::Exclusion,
            MySqlConstraintType::NotNull => ConstraintType::NotNull,
            MySqlConstraintType::ConstraintTrigger => ConstraintType::ConstraintTrigger,
            MySqlConstraintType::None => ConstraintType::None,
            MySqlConstraintType::Unknown(_) => ConstraintType::Unknown,
        }
    }
}

impl From<MySqlConstraintType> for ConstraintType {
    fn from(my_constraint_type: MySqlConstraintType) -> Self {
        (&my_constraint_type).into()
    }
}

impl FromStr for MySqlConstraintType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "primary key" => Ok(MySqlConstraintType::PrimaryKey),
            "unique" => Ok(MySqlConstraintType::Unique),
            "foreign key" => Ok(MySqlConstraintType::ForeignKey),
            "check" => Ok(MySqlConstraintType::Check),
            "exclusion" => Ok(MySqlConstraintType::Exclusion),
            "is_not_nullable" => Ok(MySqlConstraintType::NotNull),
            "constraint trigger" => Ok(MySqlConstraintType::ConstraintTrigger),
            "" | "is_nullable" | "none" => Ok(MySqlConstraintType::None),
            _ => {
                debug!("Invalid constraint type: {}", s);
                Ok(MySqlConstraintType::Unknown(s.to_string()))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MySqlIsNullable {
    Yes,
    No,
    Unknown(String),
}

impl FromStr for MySqlIsNullable {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "is_nullable" => Ok(MySqlIsNullable::Yes),
            "is_not_nullable" => Ok(MySqlIsNullable::No),
            _ => {
                debug!("Invalid value for is_nullable: {}", s);
                Ok(MySqlIsNullable::Unknown(s.to_string()))
            }
        }
    }
}

impl From<&MySqlIsNullable> for IsNullable {
    fn from(my_is_nullable: &MySqlIsNullable) -> Self {
        match my_is_nullable {
            MySqlIsNullable::Yes => IsNullable::Yes,
            MySqlIsNullable::No => IsNullable::No,
            MySqlIsNullable::Unknown(s) => IsNullable::Unknown(s.to_owned()),
        }
    }
}

impl From<MySqlIsNullable> for IsNullable {
    fn from(my_is_nullable: MySqlIsNullable) -> Self {
        (&my_is_nullable).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MySqlIsIdentity {
    Yes,
    No,
    Unknown(String),
}

impl FromStr for MySqlIsIdentity {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "primary" | "primary key" => Ok(MySqlIsIdentity::Yes),
            "not_primary" => Ok(MySqlIsIdentity::No),
            _ => {
                debug!("Invalid value for is_identity: {}", s);
                Ok(MySqlIsIdentity::Unknown(s.to_string()))
            }
        }
    }
}

impl From<&MySqlIsIdentity> for IsIdentity {
    fn from(my_is_identity: &MySqlIsIdentity) -> Self {
        match my_is_identity {
            MySqlIsIdentity::Yes => IsIdentity::Yes,
            MySqlIsIdentity::No => IsIdentity::No,
            MySqlIsIdentity::Unknown(s) => IsIdentity::Unknown(s.to_owned()),
        }
    }
}

impl From<MySqlIsIdentity> for IsIdentity {
    fn from(my_is_identity: MySqlIsIdentity) -> Self {
        (&my_is_identity).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MySqlIsGenerated {
    Always,
    ByDefault,
    ByDefaultOnNull,
    Never,
    Unknown(String),
}

impl FromStr for MySqlIsGenerated {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "always" => Ok(MySqlIsGenerated::Always),
            "by default" => Ok(MySqlIsGenerated::ByDefault),
            "by default on null" => Ok(MySqlIsGenerated::ByDefaultOnNull),
            "never" => Ok(MySqlIsGenerated::Never),
            _ => {
                debug!("Invalid value for is_generated: {}", s);
                Ok(MySqlIsGenerated::Unknown(s.to_string()))
            }
        }
    }
}

impl From<&MySqlIsGenerated> for IsGenerated {
    fn from(my_is_generated: &MySqlIsGenerated) -> Self {
        match my_is_generated {
            MySqlIsGenerated::Always => IsGenerated::Always,
            MySqlIsGenerated::ByDefault => IsGenerated::ByDefault,
            MySqlIsGenerated::ByDefaultOnNull => IsGenerated::ByDefaultOnNull,
            MySqlIsGenerated::Never => IsGenerated::Never,
            MySqlIsGenerated::Unknown(s) => IsGenerated::Unknown(s.to_owned()),
        }
    }
}

impl From<MySqlIsGenerated> for IsGenerated {
    fn from(my_is_generated: MySqlIsGenerated) -> Self {
        (&my_is_generated).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EnumTest<P, I> {
        // parsing from this string...
        parsed_from: String,
        // must become this enum value...
        my_const_type: P,
        // must cast into this ADR enum value.
        const_type: I,
    }

    #[test]
    fn test_my_object_type_from_str() {
        let test_cases = [
            EnumTest {
                parsed_from: "BASE TABLE".to_string(),
                my_const_type: MySqlObjectType::BaseTable,
                const_type: ObjectType::BaseTable,
            },
            EnumTest {
                parsed_from: "PARTITIONED TABLE".to_string(),
                my_const_type: MySqlObjectType::PartitionedTable,
                const_type: ObjectType::PartitionedTable,
            },
            EnumTest {
                parsed_from: "VIEW".to_string(),
                my_const_type: MySqlObjectType::View,
                const_type: ObjectType::View,
            },
            EnumTest {
                parsed_from: "MATERIALIZED VIEW".to_string(),
                my_const_type: MySqlObjectType::MaterializedView,
                const_type: ObjectType::MaterializedView,
            },
            EnumTest {
                parsed_from: "UNKNOWN".to_string(),
                my_const_type: MySqlObjectType::Unknown("UNKNOWN".to_string()),
                const_type: ObjectType::Unknown("UNKNOWN".to_string()),
            },
        ];
        for enum_test in test_cases.iter() {
            let my_object_type: MySqlObjectType = enum_test.parsed_from.parse().unwrap();
            assert_eq!(my_object_type, enum_test.my_const_type);
            let const_type: ObjectType = (&my_object_type).into();
            assert_eq!(const_type, enum_test.const_type);
        }
    }

    #[test]
    fn test_constraint_type_from_str_and_to_constraint_type() {
        let enum_variants = [
            EnumTest {
                parsed_from: "PRIMARY KEY".to_string(),
                my_const_type: MySqlConstraintType::PrimaryKey,
                const_type: ConstraintType::PrimaryKey,
            },
            EnumTest {
                parsed_from: "UNIQUE".to_string(),
                my_const_type: MySqlConstraintType::Unique,
                const_type: ConstraintType::Unique,
            },
            EnumTest {
                parsed_from: "FOREIGN KEY".to_string(),
                my_const_type: MySqlConstraintType::ForeignKey,
                const_type: ConstraintType::ForeignKey,
            },
            EnumTest {
                parsed_from: "CHECK".to_string(),
                my_const_type: MySqlConstraintType::Check,
                const_type: ConstraintType::Check,
            },
            EnumTest {
                parsed_from: "EXCLUSION".to_string(),
                my_const_type: MySqlConstraintType::Exclusion,
                const_type: ConstraintType::Exclusion,
            },
            EnumTest {
                parsed_from: "is_not_nullable".to_string(),
                my_const_type: MySqlConstraintType::NotNull,
                const_type: ConstraintType::NotNull,
            },
            EnumTest {
                parsed_from: "CONSTRAINT TRIGGER".to_string(),
                my_const_type: MySqlConstraintType::ConstraintTrigger,
                const_type: ConstraintType::ConstraintTrigger,
            },
            EnumTest {
                parsed_from: String::new(),
                my_const_type: MySqlConstraintType::None,
                const_type: ConstraintType::None,
            },
            EnumTest {
                parsed_from: "NONE".to_string(),
                my_const_type: MySqlConstraintType::None,
                const_type: ConstraintType::None,
            },
            EnumTest {
                parsed_from: "UNKNOWN".to_string(),
                my_const_type: MySqlConstraintType::Unknown("UNKNOWN".to_string()),
                const_type: ConstraintType::Unknown,
            },
        ];
        for enum_test in enum_variants.iter() {
            let my_const_type: MySqlConstraintType = enum_test.parsed_from.parse().unwrap();
            assert_eq!(my_const_type, enum_test.my_const_type);
            let const_type: ConstraintType = my_const_type.into();
            assert_eq!(const_type, enum_test.const_type);
        }
    }

    #[test]
    fn test_my_const_type_from_str_and_to_is_nullable() {
        let enum_variants = [
            EnumTest {
                parsed_from: "IS_NULLABLE".to_string(),
                my_const_type: MySqlIsNullable::Yes,
                const_type: IsNullable::Yes,
            },
            EnumTest {
                parsed_from: "IS_NOT_NULLABLE".to_string(),
                my_const_type: MySqlIsNullable::No,
                const_type: IsNullable::No,
            },
            EnumTest {
                parsed_from: "UNKNOWN".to_string(),
                my_const_type: MySqlIsNullable::Unknown("UNKNOWN".to_string()),
                const_type: IsNullable::Unknown("UNKNOWN".to_string()),
            },
        ];
        for enum_test in enum_variants.iter() {
            let my_const_type: MySqlIsNullable = enum_test.parsed_from.parse().unwrap();
            assert_eq!(my_const_type, enum_test.my_const_type);
            let const_type: IsNullable = my_const_type.into();
            assert_eq!(const_type, enum_test.const_type);
        }
    }

    #[test]
    fn test_my_is_identity_from_str_and_to_is_identity() {
        let enum_variants = [
            EnumTest {
                parsed_from: "PRIMARY".to_string(),
                my_const_type: MySqlIsIdentity::Yes,
                const_type: IsIdentity::Yes,
            },
            EnumTest {
                parsed_from: "not_primary".to_string(),
                my_const_type: MySqlIsIdentity::No,
                const_type: IsIdentity::No,
            },
            EnumTest {
                parsed_from: "UNKNOWN".to_string(),
                my_const_type: MySqlIsIdentity::Unknown("UNKNOWN".to_string()),
                const_type: IsIdentity::Unknown("UNKNOWN".to_string()),
            },
        ];
        for enum_test in enum_variants.iter() {
            let my_const_type: MySqlIsIdentity = enum_test.parsed_from.parse().unwrap();
            assert_eq!(my_const_type, enum_test.my_const_type);
            let const_type: IsIdentity = my_const_type.into();
            assert_eq!(const_type, enum_test.const_type);
        }
    }

    #[test]
    fn test_my_is_generated_from_str_and_to_is_generated() {
        let enum_variants = [
            EnumTest {
                parsed_from: "ALWAYS".to_string(),
                my_const_type: MySqlIsGenerated::Always,
                const_type: IsGenerated::Always,
            },
            EnumTest {
                parsed_from: "BY DEFAULT".to_string(),
                my_const_type: MySqlIsGenerated::ByDefault,
                const_type: IsGenerated::ByDefault,
            },
            EnumTest {
                parsed_from: "BY DEFAULT ON NULL".to_string(),
                my_const_type: MySqlIsGenerated::ByDefaultOnNull,
                const_type: IsGenerated::ByDefaultOnNull,
            },
            EnumTest {
                parsed_from: "NEVER".to_string(),
                my_const_type: MySqlIsGenerated::Never,
                const_type: IsGenerated::Never,
            },
            EnumTest {
                parsed_from: "UNKNOWN".to_string(),
                my_const_type: MySqlIsGenerated::Unknown("UNKNOWN".to_string()),
                const_type: IsGenerated::Unknown("UNKNOWN".to_string()),
            },
        ];
        for enum_test in enum_variants.iter() {
            let my_const_type: MySqlIsGenerated = enum_test.parsed_from.parse().unwrap();
            assert_eq!(my_const_type, enum_test.my_const_type);
            let const_type: IsGenerated = my_const_type.into();
            assert_eq!(const_type, enum_test.const_type);
        }
    }
}
