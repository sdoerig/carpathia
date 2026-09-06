use crate::configuration::carpathia_conf::CarpathiaConfig;
use crate::return_values::carpathia_errors::CarpathiaError;
use carpathia_adr::adr::abstract_db_repr::AbstractDbRepr;

pub(crate) trait DatabaseQuerier {
    async fn get_schema(config: &CarpathiaConfig) -> Result<AbstractDbRepr, CarpathiaError>;
}
