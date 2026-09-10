use crate::adr::abstract_db_repr::AbstractDbRepr;
use crate::adr_errors::AdrError;
use crate::db_type::db_to_user_type_structs::DEFAULT_TYPE_MAPPING;
use crate::db_type::db_to_user_type_structs::Types;

/// Returning all the types found in the database schema - the users need this to
/// create their own mapping.
///
/// If there is a mapping file provided, the old mapping is merged into the
/// new mapping structure.
pub fn get_db_types(
    config_types: &Types,
    table_info_map: &AbstractDbRepr,
) -> Result<Types, AdrError> {
    if table_info_map.tables.is_empty() {
        return Err(AdrError::NoDbObjectsDiscovered(
            "No tables found".to_string(),
        ));
    }

    let mut types = Types::new();

    // Chaining loops - otherwhise I had to create loop in loop.
    table_info_map
        .tables
        .values()
        .flat_map(|table| table.attributes.values())
        .for_each(|attribute| {
            let u_import_old = config_types
                .type_mapping
                .get(&attribute.data_type)
                .unwrap_or(DEFAULT_TYPE_MAPPING);

            types
                .type_mapping
                .entry(attribute.data_type.clone())
                .or_insert_with(|| u_import_old.clone());
        });

    Ok(types)
}
