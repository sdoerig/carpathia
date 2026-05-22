use crate::cache::cache_structs::CacheFile;
use crate::configuration::carpathia_conf::CarpathiaConfig;
use crate::configuration::conf_structs::{DEFAULT_TYPE_MAPPING, Types};
use crate::db::db_schema_structs::AbstractDbRepr;
use crate::return_values::carpathia_errors::{CarpathiaError, ErrorNumber};
use tera::{Context, Tera};

#[expect(dead_code)]
pub struct TemplateEngine {
    cache_result: CacheFile,
    db_schema: std::collections::HashMap<String, AbstractDbRepr>,
}

impl TemplateEngine {
    #[expect(dead_code)]
    pub(crate) fn new(
        cache_result: CacheFile,
        db_schema: std::collections::HashMap<String, AbstractDbRepr>,
    ) -> Self {
        Self {
            cache_result,
            db_schema,
        }
    }

    pub fn render_from_repr(
        tera: &Tera,
        template_name: &str,
        repr: &AbstractDbRepr,
    ) -> Result<String, tera::Error> {
        let mut ctx = Context::new();
        ctx.insert("version", &repr.version);
        ctx.insert("tables", &repr.tables);
        ctx.insert("views", &repr.views);

        tera.render(template_name, &ctx)
    }
}

/// Returning all the types found in the database schema - the users need this to
/// create their own mapping.
/// 
/// If there is a mapping file provided, the old mapping is merged into the
/// new mapping structure.
pub fn get_db_types(
    config: &CarpathiaConfig,
    table_info_map: &AbstractDbRepr,
) -> Result<Types, CarpathiaError> {
    if table_info_map.tables.is_empty() {
        return Err(CarpathiaError {
            message: "No tables found".to_string(),
            error_type: ErrorNumber::NoDbObjectsDiscovered,
        });
    }

    let mut types = Types::new();

    // Chaining loops - otherwhise I had to create loop in loop.
    table_info_map.tables.values()
        .flat_map(|table| table.attributes.values())
        .for_each(|attribute| {
            let u_import_old = config
                .type_map
                .type_mapping
                .get(&attribute.data_type)
                .unwrap_or(DEFAULT_TYPE_MAPPING);

            types.type_mapping
                .entry(attribute.data_type.clone())
                .or_insert_with(|| u_import_old.clone());
        });

    Ok(types)
}
