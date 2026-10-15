use crate::configuration::conf_enums::DbPool;
use crate::db::mysql::mysql_structs::MySqlInfo;
use crate::return_values::carpathia_errors::CarpathiaError;
use crate::{configuration::carpathia_conf::CarpathiaConfig, db::traits::DatabaseQuerier};
#[allow(unused_imports)]
use carpathia_adr::adr::abstract_db_repr::{
    ABSTRACT_DB_REPR_VERSION, AbstractAttribute, AbstractDbRepr, AbstractTableRepr, ObjectType,
};
use carpathia_adr::adr::abstract_db_repr_builder::AbstractDbReprBuilder;
#[allow(unused_imports)]
use log::{debug, error, info};
pub(crate) struct MySqlQuerier;

const LIMIT: i32 = 1000;
const SCHEMA_QUERY: &str = r"
select
    c.table_catalog as database_name,
    c.table_schema,
    c.table_name,
    c.column_name,
    c.ordinal_position as column_position,
    c.data_type,
    c.column_type,
    c.character_maximum_length,
    c.numeric_precision,
    c.numeric_scale,
    c.datetime_precision,
    c.is_nullable,
    c.column_default,
    c.extra,
    c.generation_expression,
    c.column_comment,
    t.table_type,
    t.engine,
    t.table_collation,
    t.table_comment,
    tc.constraint_name,
    tc.constraint_type,
    kcu.ordinal_position as constraint_column_position,
    kcu.position_in_unique_constraint,
    kcu.referenced_table_schema,
    kcu.referenced_table_name,
    kcu.referenced_column_name,
    rc.update_rule,
    rc.delete_rule,
    s.index_name,
    s.seq_in_index,
    s.non_unique,
    s.index_type,
    chk.check_clause
from information_schema.columns c
join information_schema.tables t
  on t.table_schema = c.table_schema
 and t.table_name = c.table_name
left join information_schema.key_column_usage kcu
  on kcu.table_schema = c.table_schema
 and kcu.table_name = c.table_name
 and kcu.column_name = c.column_name
left join information_schema.table_constraints tc
  on tc.constraint_schema = kcu.constraint_schema
 and tc.table_name = kcu.table_name
 and tc.constraint_name = kcu.constraint_name
left join information_schema.referential_constraints rc
  on rc.constraint_schema = tc.constraint_schema
 and rc.constraint_name = tc.constraint_name
left join information_schema.check_constraints chk
  on chk.constraint_schema = tc.constraint_schema
 and chk.constraint_name = tc.constraint_name
left join information_schema.statistics s
  on s.table_schema = c.table_schema
 and s.table_name = c.table_name
 and s.column_name = c.column_name
where c.table_schema not in
(
    'information_schema',
    'mysql',
    'performance_schema',
    'sys'
)
order by
    c.table_schema,
    c.table_name,
    c.ordinal_position,
    tc.constraint_name,
    s.index_name,
    s.seq_in_index
limit ?
offset ?
    ;
";
#[allow(dead_code)]
impl DatabaseQuerier for MySqlQuerier {
    async fn get_schema(config: &CarpathiaConfig) -> Result<AbstractDbRepr, CarpathiaError> {
        #[allow(unused_variables)]
        let table_info_map: std::collections::BTreeMap<String, AbstractTableRepr> =
            std::collections::BTreeMap::new();
        #[allow(unused_variables)]   
        let view_info_map: std::collections::BTreeMap<String, AbstractTableRepr> =
            std::collections::BTreeMap::new();
        let mut offset = 0;
        let pool = match config.db_pool {
            DbPool::MySql(ref pool) => pool,
            _ => {
                return Err(CarpathiaError {
                    message: "Invalid database pool type for PostgreSQL querier".to_string(),
                    error_type:
                        crate::return_values::carpathia_errors::ErrorNumber::InvalidPoolType,
                });
            }
        };
        loop {
            let rows: Vec<MySqlInfo> = sqlx::query_as::<_, MySqlInfo>(SCHEMA_QUERY)
                .bind(LIMIT)
                .bind(offset)
                .fetch_all(pool)
                .await
                .map_err(|e| {
                    debug!("Error executing schema query: {e}");
                    CarpathiaError {
                        message: format!("Failed to execute schema query: {e}"),
                        error_type: crate::return_values::carpathia_errors::ErrorNumber::DatabaseConnectionError,
                    }
                })?;
            let num_rows = rows.len();
            debug!("Fetched {num_rows} rows from schema query with offset {offset}");
            for row in rows {
                info!("{:#?}", row);
            }
            offset += LIMIT;
            if num_rows < LIMIT as usize {
                break;
            }
        }
        Ok(AbstractDbReprBuilder::new().build())
    }
}
