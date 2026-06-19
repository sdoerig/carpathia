/// This module maps CLI enums to core enums so core does not need to depend on clap
///
use carpathia_core::configuration::conf_enums::{CacheModus, DbType};
use carpathia_core::templates::enum_templates::InitTemplate;
use clap::ValueEnum;
#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CacheModusClap {
    BypassCache,
    UseCache,
}

/// Implements the default port per database type
impl From<CacheModusClap> for CacheModus {
    fn from(value: CacheModusClap) -> CacheModus {
        match value {
            CacheModusClap::BypassCache => CacheModus::BypassCache,
            CacheModusClap::UseCache => CacheModus::UseCache,
        }
    }
}

/// Implements the database type.
/// The enum is used to assemble the url to connect to the database.
///
/// - From for i32 is used for the default port
/// - Display for DbType is used as the protocol to connect to the database.
#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DbTypeClap {
    Postgres,
    Dummy,
}

/// Implements the default port per database type
impl From<DbTypeClap> for DbType {
    fn from(value: DbTypeClap) -> DbType {
        match value {
            DbTypeClap::Postgres => DbType::Postgres,
            DbTypeClap::Dummy => DbType::Dummy,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InitTemplateClap {
    RustLib,
    None,
}

impl From<InitTemplateClap> for InitTemplate {
    fn from(value: InitTemplateClap) -> InitTemplate {
        match value {
            InitTemplateClap::RustLib => InitTemplate::RustLib,
            InitTemplateClap::None => InitTemplate::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::conf_enums::CacheModus;
    use crate::configuration::conf_enums::DbType;
    #[test]
    fn test_cache_modus_conversion() {
        let clap_value = CacheModusClap::BypassCache;
        let core_value: CacheModus = clap_value.into();
        assert_eq!(core_value, CacheModus::BypassCache);
        let clap_value = CacheModusClap::UseCache;
        let core_value: CacheModus = clap_value.into();
        assert_eq!(core_value, CacheModus::UseCache);
    }

    #[test]
    fn test_db_type_conversion() {
        let clap_value = DbTypeClap::Postgres;
        let core_value: DbType = clap_value.into();
        assert_eq!(core_value, DbType::Postgres);
        let clap_value = DbTypeClap::Dummy;
        let core_value: DbType = clap_value.into();
        assert_eq!(core_value, DbType::Dummy);
    }

    #[test]
    fn test_init_template_conversion() {
        let clap_value = InitTemplateClap::RustLib;
        let core_value: InitTemplate = clap_value.into();
        assert_eq!(core_value, InitTemplate::RustLib);
    }
}
