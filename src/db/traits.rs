use crate::db::db_schema_structs::AbstractDbRepr;


pub(crate) trait DatabaseQuerier {
    async fn get_schema(
        &self,
    ) -> Result<AbstractDbRepr, Box<dyn std::error::Error>>;
}
