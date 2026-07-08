use serde::Serialize;
use crate::db::db_schema_structs::{AbstractAttribute, AbstractDbRepr, AbstractTableRepr, ObjectType};
#[derive(Serialize)]
pub struct AdrTemplateData<'a> {
    pub version: &'a str,
    pub tables: Vec<TableTemplateData<'a>>,
    pub views: Vec<TableTemplateData<'a>>,
}

#[derive(Serialize)]
pub struct TableTemplateData<'a> {
    pub object_type: &'a ObjectType,
    pub table_name: &'a str,
    pub comment: Option<&'a str>,
    pub u_imports: Vec<&'a str>,
    pub attributes: Vec<(&'a str, &'a AbstractAttribute)>,
}

impl<'a> From<&'a AbstractDbRepr> for AdrTemplateData<'a> {
    fn from(adr: &'a AbstractDbRepr) -> Self {
        Self {
            version: &adr.version,
            tables: adr.tables.values().map(|t| t.into()).collect(),
            views: adr.views.values().map(|v| v.into()).collect(),
        }
    }
}

impl<'a> From<&'a AbstractTableRepr> for TableTemplateData<'a> {
    fn from(table: &'a AbstractTableRepr) -> Self {
        Self {
            object_type: &table.object_type,
            table_name: &table.table_name,
            comment: table.comment.as_deref(),
            u_imports: table.u_imports.iter().map(|s| s.as_str()).collect(), // BTreeSet<String> → Vec<&str>
            attributes: table.attributes.iter().map(|(k, v)| (k.as_str(), v)).collect(), // BTreeMap<String, AbstractAttribute> → Vec<(&str, &AbstractAttribute)>
        }
    }
}