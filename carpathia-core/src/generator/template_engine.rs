use crate::cache::cache_structs::CacheFile;
use crate::configuration::conf_structs::{TypeMapping, Types};
use crate::db::db_schema_structs::AbstractDbRepr;
use crate::return_values::carpathia_errors::CarpathiaError;
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
pub fn get_db_types(table_info_map: &AbstractDbRepr) -> Result<Types, CarpathiaError> {
    let mut types = Types::new();

    for key in table_info_map.tables.keys() {
        for attribute in table_info_map.tables[key].attributes.values() {
            types
                .type_mapping
                .entry(attribute.data_type.clone())
                .or_insert(TypeMapping {
                    u_import: Some("".to_string()),
                    u_type: "".to_string(),
                });
        }
    }
    Ok(types)
}
