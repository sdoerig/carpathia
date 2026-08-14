use std::collections::BTreeMap;

use log::debug;

use crate::db::db_schema_structs::{
    AbstractAttribute, AbstractConstraint, ConstraintType, IsNullable,
};

#[derive(sqlx::FromRow, serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PgColumnInfo {
    pub object_type: String,
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
    pub data_type: String,
    pub array_dimensions: Option<i32>,
    pub is_nullable: String,
    pub column_default: Option<String>,
    pub table_is_insertable: String,
    pub column_is_updatable: String,
    pub character_maximum_length: Option<i32>,
    pub numeric_precision: Option<i32>,
    pub numeric_scale: Option<i32>,
    pub is_identity: String,
    pub identity_generation: Option<String>,
    pub is_generated: String,
    pub generation_expression: Option<String>,
    // This field is filled in by the constraint_map method and/the results of SQL_QUERY_CONSTRAINTS.
    // Also keep in mind, one attribute can have multiple constraints. Keeping all in
    // one query would make the query very clupsy and hard to maintain.
    #[sqlx(skip)]
    pub constraints: BTreeMap<ConstraintType, AbstractConstraint>,
    pub table_comment: Option<String>,
    pub column_comment: Option<String>,
}

impl PgColumnInfo {
    pub(crate) fn constraint_map(mut self, constraint_map: &PgConstraintMap) -> Self {
        let key = (
            self.table_schema.clone(),
            self.table_name.clone(),
            self.column_name.clone(),
        );
        debug!("Looking up constraint info for key: {:?}", key);
        if let Some(constraint_info_map) = constraint_map.pg_constraint_info.get(&key) {
            self.constraints = constraint_info_map
                .iter()
                .map(|(constraint_type, constraint_info)| {
                    (
                        constraint_type.clone(),
                        AbstractConstraint {
                            constraint_name: constraint_info.constraint_name.clone(),
                            constraint_value: constraint_info.constraint_value.clone(),
                            referenced_schema_name: constraint_info.foreign_schema_name.clone(),
                            referenced_table: constraint_info.foreign_relation_name.clone(),
                            u_referenced_table: None, // Placeholder, will be filled in by enrich_adr
                            u_referenced_column: None, // Placeholder, will be filled in by enrich_adr
                            referenced_column: constraint_info.foreign_attribute_name.clone(),
                        },
                    )
                })
                .collect();
        } else {
            debug!("No constraint info found for key: {:?}", key);
        }

        self
    }
}

impl From<PgColumnInfo> for AbstractAttribute {
    fn from(pg_column_info: PgColumnInfo) -> Self {
        let data_type = if let Some(dimensions) = pg_column_info.array_dimensions {
            if dimensions != 0 {
                format!("{}[{}]", pg_column_info.data_type, dimensions)
            } else {
                pg_column_info.data_type.clone()
            }
        } else {
            pg_column_info.data_type.clone()
        };
        AbstractAttribute {
            column_name: pg_column_info.column_name.clone(),
            u_column_name: String::new(), // Placeholder, will be filled in by enrich_adr
            data_type,
            u_type: String::new(), // Placeholder, will be filled in by enrich_adr
            is_nullable: pg_column_info
                .is_nullable
                .parse()
                .unwrap_or(IsNullable::Unknown(pg_column_info.is_nullable)),
            column_default: pg_column_info.column_default,

            constraints: pg_column_info.constraints,
            comment: pg_column_info.column_comment,
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub(crate) struct PgConstraintMap {
    pg_constraint_info:
        BTreeMap<(String, String, String), BTreeMap<ConstraintType, PgConstraintInfo>>,
}

impl PgConstraintMap {
    pub(crate) fn new(constraint_infos: Vec<PgConstraintInfo>) -> Self {
        let mut pg_constraint_info = BTreeMap::new();
        for constraint_info in constraint_infos {
            let key = (
                constraint_info.schema_name.clone(),
                constraint_info.relation_name.clone(),
                constraint_info.attribute_name.clone(),
            );
            pg_constraint_info
                .entry(key.clone())
                .or_insert_with(BTreeMap::new)
                .insert(
                    constraint_info
                        .constraint_type
                        .parse()
                        .unwrap_or(ConstraintType::None),
                    constraint_info.clone(),
                );
        }
        PgConstraintMap { pg_constraint_info }
    }
}

#[derive(
    sqlx::FromRow,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
)]
pub(crate) struct PgConstraintInfo {
    pub schema_name: String,
    pub relation_name: String,
    pub attribute_name: String,
    pub constraint_type: String,
    pub constraint_name: String,
    pub constraint_value: String,
    pub foreign_schema_name: Option<String>,
    pub foreign_relation_name: Option<String>,
    pub foreign_attribute_name: Option<String>,
}
