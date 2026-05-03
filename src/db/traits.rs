use crate::db::db_schema_structs::AbstractDbRepr;
use std::collections::BTreeMap;

pub(crate) trait DatabaseQuerier {
    async fn get_schema(
        &self,
    ) -> Result<BTreeMap<String, AbstractDbRepr>, Box<dyn std::error::Error>>;
}
