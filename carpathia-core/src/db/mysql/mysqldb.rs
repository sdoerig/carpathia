use crate::{configuration::carpathia_conf::CarpathiaConfig, db::traits::DatabaseQuerier};
use crate::return_values::carpathia_errors::CarpathiaError;
#[allow(unused_imports)]
use carpathia_adr::adr::abstract_db_repr::{
    ABSTRACT_DB_REPR_VERSION, AbstractAttribute, AbstractDbRepr, AbstractTableRepr, ObjectType,
};
use carpathia_adr::adr::abstract_db_repr_builder::AbstractDbReprBuilder;
#[allow(unused_imports)]
use log::{debug, error, info};
pub(crate) struct MySqlQuerier;
#[allow(dead_code)]
const SCHEMA_QUERY: &str = r"
SELECT
    c.TABLE_CATALOG AS database_name,
    c.TABLE_SCHEMA,
    c.TABLE_NAME,
    c.COLUMN_NAME,
    c.ORDINAL_POSITION AS column_position,
    c.DATA_TYPE,
    c.COLUMN_TYPE,
    c.CHARACTER_MAXIMUM_LENGTH,
    c.NUMERIC_PRECISION,
    c.NUMERIC_SCALE,
    c.DATETIME_PRECISION,
    c.IS_NULLABLE,
    c.COLUMN_DEFAULT,
    c.EXTRA,
    c.GENERATION_EXPRESSION,
    c.COLUMN_COMMENT,
    t.TABLE_TYPE,
    t.ENGINE,
    t.TABLE_COLLATION,
    t.TABLE_COMMENT,
    tc.CONSTRAINT_NAME,
    tc.CONSTRAINT_TYPE,
    kcu.ORDINAL_POSITION AS constraint_column_position,
    kcu.POSITION_IN_UNIQUE_CONSTRAINT,
    kcu.REFERENCED_TABLE_SCHEMA,
    kcu.REFERENCED_TABLE_NAME,
    kcu.REFERENCED_COLUMN_NAME,
    rc.UPDATE_RULE,
    rc.DELETE_RULE,
    s.INDEX_NAME,
    s.SEQ_IN_INDEX,
    s.NON_UNIQUE,
    s.INDEX_TYPE,
    chk.CHECK_CLAUSE
FROM information_schema.COLUMNS c
JOIN information_schema.TABLES t
  ON t.TABLE_SCHEMA = c.TABLE_SCHEMA
 AND t.TABLE_NAME = c.TABLE_NAME
LEFT JOIN information_schema.KEY_COLUMN_USAGE kcu
  ON kcu.TABLE_SCHEMA = c.TABLE_SCHEMA
 AND kcu.TABLE_NAME = c.TABLE_NAME
 AND kcu.COLUMN_NAME = c.COLUMN_NAME
LEFT JOIN information_schema.TABLE_CONSTRAINTS tc
  ON tc.CONSTRAINT_SCHEMA = kcu.CONSTRAINT_SCHEMA
 AND tc.TABLE_NAME = kcu.TABLE_NAME
 AND tc.CONSTRAINT_NAME = kcu.CONSTRAINT_NAME
LEFT JOIN information_schema.REFERENTIAL_CONSTRAINTS rc
  ON rc.CONSTRAINT_SCHEMA = tc.CONSTRAINT_SCHEMA
 AND rc.CONSTRAINT_NAME = tc.CONSTRAINT_NAME
LEFT JOIN information_schema.CHECK_CONSTRAINTS chk
  ON chk.CONSTRAINT_SCHEMA = tc.CONSTRAINT_SCHEMA
 AND chk.CONSTRAINT_NAME = tc.CONSTRAINT_NAME
LEFT JOIN information_schema.STATISTICS s
  ON s.TABLE_SCHEMA = c.TABLE_SCHEMA
 AND s.TABLE_NAME = c.TABLE_NAME
 AND s.COLUMN_NAME = c.COLUMN_NAME
WHERE c.TABLE_SCHEMA NOT IN
(
    'information_schema',
    'mysql',
    'performance_schema',
    'sys'
)
ORDER BY
    c.TABLE_SCHEMA,
    c.TABLE_NAME,
    c.ORDINAL_POSITION,
    tc.CONSTRAINT_NAME,
    s.INDEX_NAME,
    s.SEQ_IN_INDEX;
";
#[allow(dead_code)]
impl DatabaseQuerier for MySqlQuerier {
    async fn get_schema(_config: &CarpathiaConfig) -> Result<AbstractDbRepr, CarpathiaError> {
        Ok(AbstractDbReprBuilder::new().build())
    }}
