use crate::configuration::carpathia_conf::CarpathiaConfig;
use crate::{
    db::db_schema_structs::AbstractDbRepr, return_values::carpathia_errors::CarpathiaError,
};

pub(crate) trait DatabaseQuerier {
    async fn get_schema(config: &CarpathiaConfig) -> Result<AbstractDbRepr, CarpathiaError>;
}
