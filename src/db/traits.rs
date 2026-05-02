use crate::db::db_schema_structs::AbstractDbRepr;
use std::collections::HashMap;

pub(crate) trait DatabaseQuerier {
    async fn get_schema(
        &self,
    ) -> Result<HashMap<String, AbstractDbRepr>, Box<dyn std::error::Error>>;
}
