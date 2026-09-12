//! This module enriches the AbstractDbRepr with user-defined type mappings
//! based on the configuration provided by the user.
use std::collections::BTreeSet;

use log::debug;

use crate::adr::abstract_db_repr::{
    AbstractAttribute, AbstractDbRepr, AbstractForeignKey, AbstractPrimaryKey,
    AbstractReferencedTable, AbstractTableRepr, ConstraintType, KeyType, TableProperties,
};
use crate::db_type::db_to_user_type_structs::{TypeMapping, Types};

pub fn add_user_mapping_to_adr(conf_types: &Types, adr: &mut AbstractDbRepr) {
    let type_map = &conf_types.type_mapping;
    let db_to_code_names_map = &conf_types.db_to_code_names_mapping;
    for atr in adr.tables.values_mut().chain(adr.views.values_mut()) {
        add_to_atr(type_map, db_to_code_names_map, atr);
    }
}

fn add_to_atr(
    type_map: &std::collections::BTreeMap<String, TypeMapping>,
    db_name_map: &std::collections::BTreeMap<String, String>,
    atr: &mut AbstractTableRepr,
) {
    atr.u_table_name = db_name_map
        .get(&atr.table_name)
        .unwrap_or(&atr.table_name)
        .clone();

    for attribute in &mut atr.attributes.values_mut() {
        map_constraints_to_user_friendly_names(&mut atr.table_properties, db_name_map, attribute);
        // Add a user-friendly mapping for the column name
        // map the user type to the ADR
        let default_type_mapping = TypeMapping {
            u_import: None,
            u_type: attribute.data_type.clone(),
        };
        attribute.u_column_name = db_name_map
            .get(&attribute.column_name)
            .unwrap_or(&attribute.column_name)
            .clone();
        let u_type_map = match type_map.get(&attribute.data_type) {
            Some(t) => t,
            None => &default_type_mapping,
        };
        attribute.u_type = u_type_map.u_type.clone();
        if let Some(import) = u_type_map.u_import.clone()
            && !import.is_empty()
        {
            debug!("insert_u_import {}", import);
            atr.u_imports.insert(import);
        }
    }
}

fn map_constraints_to_user_friendly_names(
    atr_tbl_prop: &mut BTreeSet<TableProperties>,
    db_name_map: &std::collections::BTreeMap<String, String>,
    attribute: &mut AbstractAttribute,
) {
    for (key, constraint) in attribute.constraints.iter_mut() {
        match key {
            ConstraintType::PrimaryKey => {
                let pk: AbstractPrimaryKey = atr_tbl_prop
                    .iter()
                    .find_map(|prop| match prop {
                        TableProperties::PrimaryKey(pk) => Some(pk.clone()),
                        _ => None,
                    })
                    .unwrap_or_else(|| AbstractPrimaryKey {
                        constraint_name: constraint.constraint_name.clone(),
                        columns: std::iter::once(attribute.column_name.clone()).collect(),
                        key_type: KeyType::SingleColumn,
                    });

                atr_tbl_prop.insert(TableProperties::PrimaryKey(
                    pk + AbstractPrimaryKey {
                        constraint_name: constraint.constraint_name.clone(),
                        columns: std::iter::once(attribute.column_name.clone()).collect(),
                        key_type: KeyType::SingleColumn,
                    },
                ));
            }
            ConstraintType::ForeignKey => {
                let fk: AbstractForeignKey = atr_tbl_prop
                    .iter()
                    .find_map(|prop| match prop {
                        TableProperties::ForeignKey(fk) => Some(fk.clone()),
                        _ => None,
                    })
                    .unwrap_or_else(|| AbstractForeignKey {
                        constraint_name: constraint.constraint_name.clone(),
                        columns: std::iter::once(AbstractReferencedTable {
                            column: attribute.column_name.clone(),
                            referenced_table: constraint
                                .referenced_table
                                .clone()
                                .unwrap_or_default(),
                            referenced_column: constraint
                                .referenced_column
                                .clone()
                                .unwrap_or_default(),
                        })
                        .collect(),
                        key_type: KeyType::SingleColumn,
                    });
                atr_tbl_prop.insert(TableProperties::ForeignKey(
                    fk + AbstractForeignKey {
                        constraint_name: constraint.constraint_name.clone(),
                        columns: std::iter::once(AbstractReferencedTable {
                            column: attribute.column_name.clone(),
                            referenced_table: constraint
                                .referenced_table
                                .clone()
                                .unwrap_or_default(),
                            referenced_column: constraint
                                .referenced_column
                                .clone()
                                .unwrap_or_default(),
                        })
                        .collect(),
                        key_type: KeyType::SingleColumn,
                    },
                ));
            }
            _ => {
                // Basically anything is selectable.
            }
        };
        if let Some(referenced_table) = &constraint.referenced_table {
            constraint.u_referenced_table = db_name_map
                .get(referenced_table)
                .unwrap_or(referenced_table)
                .clone()
                .into();
        }
        if let Some(referenced_column) = &constraint.referenced_column {
            constraint.u_referenced_column = db_name_map
                .get(referenced_column)
                .unwrap_or(referenced_column)
                .clone()
                .into();
        }
    }
}
