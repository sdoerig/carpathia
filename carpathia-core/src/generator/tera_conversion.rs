use crate::db::db_schema_structs::{
    AbstractAttribute, AbstractDbRepr, AbstractTableRepr, ObjectType,
};
/// This module contains the data structures used to pass data to Tera templates for generating code from an AbstractDbRepr.
/// The data structures are designed to be serializable with Serde, allowing them to be easily converted into a format that
/// Tera can work with. The main structure is `AdrTemplateData`, which holds the version of the database representation
/// and lists of tables and views.
/// The reason for having this data structure is because tera does not support BTreeMap and BTreeSet. But
/// having a deterministic order of data when executing templates is crucical.
/// Imagine your checking in the gnerated code into a git repo and each run produces the same but different - this would be a very nasty behavior.
/// That what this data sturcture prevents.
/// Now own might wonder, why not just using this data structure all throughout. And the reason is, it is very confortable
/// having BTreeMap when building the AbstractDbRepr.  
/// Or long story short, I'm lazy and like to use comfortable data structures and I do not have a better idea right now. But it
/// should get the job done.
///
use serde::Serialize;
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
    pub attributes: Vec<&'a AbstractAttribute>,
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
            attributes: table.attributes.values().collect(), // BTreeMap<String, AbstractAttribute> → Vec<&AbstractAttribute>
        }
    }
}
