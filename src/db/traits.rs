use crate::{
    db::db_schema_structs::AbstractDbRepr, return_values::carpathia_errors::CarpathiaError,
};

pub(crate) trait DatabaseQuerier {
    fn new(db_url: &str, db_name: &str) -> Result<Self, CarpathiaError>
    where
        Self: Sized;
    async fn get_schema(&self) -> Result<AbstractDbRepr, CarpathiaError>;
}
