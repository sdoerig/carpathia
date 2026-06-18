//! This module enriches the AbstractDbRepr with user-defined type mappings
//! based on the configuration provided by the user.

use log::debug;

use crate::configuration::carpathia_conf::CarpathiaConfig;
use crate::configuration::conf_structs::TypeMapping;
use crate::db::db_schema_structs::AbstractDbRepr;

pub(crate) fn add_user_mapping_to_adr(conf: &CarpathiaConfig, adr: &mut AbstractDbRepr) {
    let type_map = &conf.type_map.type_mapping;
    for atr in adr.tables.values_mut().chain(adr.views.values_mut()) {
        add_to_atr(type_map, atr);
    }
}

fn add_to_atr(
    type_map: &std::collections::BTreeMap<String, TypeMapping>,
    atr: &mut super::db_schema_structs::AbstractTableRepr,
) {
    for attribute in &mut atr.attributes.values_mut() {
        // Add a user-friendly mapping for the column name
        // map the user type to the ADR
        let default_type_mapping = TypeMapping {
            u_import: None,
            u_type: attribute.data_type.clone(),
        };
        let u_type_map = match type_map.get(&attribute.data_type) {
            Some(t) => t,
            None => &default_type_mapping,
        };
        attribute.u_type = u_type_map.u_type.clone();
        if let Some(import) = u_type_map.u_import.clone()
            && !import.is_empty()
        {
            debug!("insert_u_import {}", &import);
            atr.u_imports.insert(import);
        }
    }
}
