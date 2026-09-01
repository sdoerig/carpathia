use crate::configuration::carpathia_conf::CarpathiaConfig;
use crate::configuration::conf_enums::DbPool;
/// PostgreSQL schema querieer. Currently implemented
/// - Basic tables
/// - Views
/// - Materialized Views
use crate::db::db_schema_structs::{
    ABSTRACT_DB_REPR_VERSION, AbstractAttribute, AbstractDbRepr, AbstractTableRepr, ObjectType,
};
use crate::db::postgres::postgresql_structs::{
    PgColumnInfo, PgConstraintInfo, PgConstraintMap, PgObjectType,
};
use crate::db::traits::DatabaseQuerier;
use crate::return_values::carpathia_errors::CarpathiaError;
use log::{debug, error, info};
use std::collections::{BTreeMap, BTreeSet};
pub(crate) struct PostgresQuerier;

const LIMIT: i64 = 1000;

const CONSTRAINT_QUERY: &str = r"
(
    SELECT
        ns.nspname AS schema_name,
        tbl.relname AS relation_name,
        att.attname AS attribute_name,
        CASE con.contype
            WHEN 'p' THEN 'PRIMARY KEY'
            WHEN 'f' THEN 'FOREIGN KEY'
            WHEN 'u' THEN 'UNIQUE'
            WHEN 'c' THEN 'CHECK'
            WHEN 'x' THEN 'EXCLUSION'
            WHEN 'n' THEN 'NOT NULL'
            WHEN 't' THEN 'CONSTRAINT TRIGGER'
            ELSE con.contype::text
            END 
        AS constraint_type,
        con.conname AS constraint_name,
        pg_get_constraintdef(con.oid, TRUE) AS constraint_value,
        -- Additional columns for referenced tables/attributes (only relevant for FK)
        f_ns.nspname AS foreign_schema_name,
        f_tbl.relname AS foreign_relation_name,
        f_att.attname AS foreign_attribute_name
    FROM 
        pg_constraint con
    JOIN pg_class tbl
        ON tbl.oid = con.conrelid
    JOIN pg_namespace ns
        ON ns.oid = tbl.relnamespace
    LEFT JOIN LATERAL unnest(con.conkey) AS k(attnum)
        ON TRUE
    LEFT JOIN pg_attribute att
        ON att.attrelid = tbl.oid
        AND att.attnum = k.attnum
    -- Joins for foreign key references
    LEFT JOIN pg_class f_tbl 
        ON f_tbl.oid = con.confrelid
    LEFT JOIN pg_namespace f_ns 
        ON f_ns.oid = f_tbl.relnamespace
    -- confkey is an Array, so we use a lateral join to map the corresponding attribute
    LEFT JOIN LATERAL unnest(con.confkey) WITH ORDINALITY AS fk(attnum, ord)
        ON TRUE
    LEFT JOIN LATERAL unnest(con.conkey) WITH ORDINALITY AS ck(attnum, ord)
        ON ck.ord = fk.ord
    LEFT JOIN pg_attribute f_att 
        ON f_att.attrelid = con.confrelid 
        AND f_att.attnum = fk.attnum
    WHERE 
        ck.attnum = att.attnum OR ck.attnum IS NULL
    ORDER BY
        ns.nspname,
    tbl.relname,
    att.attname,
    con.conname
)
UNION ALL
(
    SELECT
        ns.nspname,
        tbl.relname,
        att.attname,
        'NOT NULL' AS constraint_type,
        att.attname || '_not_null' AS constraint_name,
        'NOT NULL' AS constraint_value,
        NULL AS foreign_schema_name,
        NULL AS foreign_relation_name,
        NULL AS foreign_attribute_name
    FROM 
        pg_attribute att
    JOIN pg_class tbl
        ON tbl.oid = att.attrelid
    JOIN pg_namespace ns
        ON ns.oid = tbl.relnamespace
    WHERE 
        att.attnotnull
        AND att.attnum > 0
        AND NOT att.attisdropped
);";

const SCHEMA_QUERY: &str = r"
WITH cols AS (
    SELECT
        n.nspname AS table_schema,
        c.relname AS table_name,
        a.attname AS column_name,
        a.atttypmod AS atttypmod,
        a.atttypid as atttypid,
        format_type(a.atttypid, a.atttypmod) AS data_type,
        a.attndims::int4 AS array_dimensions,
        NOT a.attnotnull AS is_nullable,
        pg_get_expr(ad.adbin, ad.adrelid) AS column_default,
        a.attnum,
        c.oid AS table_oid,
        a.attrelid AS attrelid,
        a.attidentity::text AS identity_generation,
        a.attgenerated
    FROM pg_class c
    JOIN pg_namespace n ON n.oid = c.relnamespace
    JOIN pg_attribute a 
        ON a.attrelid = c.oid 
       AND a.attnum > 0 
       AND NOT a.attisdropped
    LEFT JOIN pg_attrdef ad 
        ON ad.adrelid = c.oid 
       AND ad.adnum = a.attnum
    WHERE n.nspname = 'public'
      AND c.relkind IN ('r','v', 'p')
),

pk_constraints AS (
    SELECT
        con.conname AS constraint_name,
        con.conrelid AS table_oid,
        unnest(con.conkey) AS column_attnum
    FROM pg_constraint con
    WHERE con.contype = 'p'
),

index_info AS (
    SELECT
        t.oid AS table_oid,
        array_agg(pg_get_indexdef(i.indexrelid)) AS index_definitions
    FROM pg_class t
    JOIN pg_index i ON i.indrelid = t.oid
    WHERE t.relnamespace = (SELECT oid FROM pg_namespace WHERE nspname = 'public')
    GROUP BY t.oid
),

trigger_info AS (
    SELECT
        t.oid AS table_oid,
        array_agg(
            tg.tgname || ' ' ||
            pg_get_triggerdef(tg.oid, true)
        ) AS trigger_definitions
    FROM pg_class t
    JOIN pg_trigger tg ON tg.tgrelid = t.oid
    WHERE tg.tgisinternal = false
      AND t.relnamespace = (SELECT oid FROM pg_namespace WHERE nspname = 'public')
    GROUP BY t.oid
)

SELECT
    CASE c.relkind
        WHEN 'r' THEN 'BASE TABLE'
        WHEN 'v' THEN 'VIEW'
        WHEN 'p' THEN 'PARTITIONED TABLE'
        ELSE 'OTHER'
    END AS object_type,
    col.table_schema,
    col.table_name,
    col.column_name,
    col.data_type,
    col.array_dimensions,
    CASE WHEN col.is_nullable THEN 'YES' ELSE 'NO' END AS is_nullable,
    col.column_default,
    CASE WHEN c.relkind = 'r' THEN 'YES' ELSE 'NO' END AS table_is_insertable,
    CASE WHEN c.relkind = 'r' THEN 'YES' ELSE 'NO' END AS column_is_updatable,
    information_schema._pg_char_max_length(col.atttypid, col.atttypmod) AS character_maximum_length,
    information_schema._pg_numeric_precision(col.atttypid, col.atttypmod) AS numeric_precision,
    information_schema._pg_numeric_scale(col.atttypid, col.atttypmod) AS numeric_scale,

    CASE WHEN col.identity_generation <> '' THEN 'YES' ELSE 'NO' END AS is_identity,
    col.identity_generation,

    CASE WHEN col.attgenerated <> '' THEN 'ALWAYS' ELSE 'NEVER' END AS is_generated,
    CASE 
        WHEN col.attgenerated <> '' THEN pg_get_expr(ad.adbin, ad.adrelid)
        ELSE NULL
    END AS generation_expression,

    '' AS constraint_name,

    '' AS constraint_type,
    '' AS referenced_table,
    '' AS referenced_column,
        
    --rt.relname AS referenced_table,
    --ra.attname AS referenced_column,

    obj_description(col.table_oid) AS table_comment,
    col_description(col.attrelid, col.attnum) AS column_comment,

    idx.index_definitions,
    trg.trigger_definitions

FROM cols col
JOIN pg_class c ON c.oid = col.table_oid

LEFT JOIN pg_attrdef ad 
    ON ad.adrelid = col.attrelid 
   AND ad.adnum = col.attnum

LEFT JOIN pk_constraints pk
    ON pk.table_oid = col.table_oid
   AND pk.column_attnum = col.attnum
LEFT JOIN index_info idx ON idx.table_oid = col.table_oid
LEFT JOIN trigger_info trg ON trg.table_oid = col.table_oid

UNION ALL

-- MATERIALIZED VIEWS (keine Indizes/Trigger)
SELECT
    'MATERIALIZED VIEW',
    mat.schemaname,
    mat.matviewname,
    a.attname,
    format_type(a.atttypid, a.atttypmod),
    a.attndims::int4,
    CASE WHEN a.attnotnull THEN 'NO' ELSE 'YES' END,
    NULL,
    'NO',
    'NO',
    NULL,
    NULL,
    NULL,
    'NO',
    NULL,
    'NEVER',
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    obj_description((quote_ident(mat.schemaname) || '.' || quote_ident(mat.matviewname))::regclass),
    col_description(a.attrelid, a.attnum),
    NULL,
    NULL
FROM pg_matviews mat
JOIN pg_attribute a 
    ON a.attrelid = (quote_ident(mat.schemaname) || '.' || quote_ident(mat.matviewname))::regclass
WHERE mat.schemaname = 'public'
  AND a.attnum > 0
  AND NOT a.attisdropped

ORDER BY table_name, column_name
LIMIT $1
OFFSET $2;
    ";

impl PostgresQuerier {
    async fn get_constraints(config: &CarpathiaConfig) -> Result<PgConstraintMap, CarpathiaError> {
        let pool = match &config.db_pool {
            DbPool::Postgres(pool) => pool,
            _ => {
                return Err(CarpathiaError {
                    message: "Invalid database pool type for PostgreSQL querier".to_string(),
                    error_type:
                        crate::return_values::carpathia_errors::ErrorNumber::InvalidPoolType,
                });
            }
        };
        let rows: Vec<PgConstraintInfo> = sqlx::query_as::<_, PgConstraintInfo>(CONSTRAINT_QUERY)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                debug!("Error executing constraint query: {e}");
                CarpathiaError {
                    message: format!("Failed to execute constraint query: {e}"),
                    error_type:
                        crate::return_values::carpathia_errors::ErrorNumber::DatabaseConnectionError,
                }
            })?;

        Ok(PgConstraintMap::new(rows))
    }
}

impl DatabaseQuerier for PostgresQuerier {
    async fn get_schema(config: &CarpathiaConfig) -> Result<AbstractDbRepr, CarpathiaError> {
        // Here you would implement the logic to query the database for its schema
        // and populate your data structures with the extracted information.
        // This is just a placeholder for demonstration purposes.
        info!("Parsing schema for PostgreSQL database:");
        let mut table_info_map: std::collections::BTreeMap<String, AbstractTableRepr> =
            std::collections::BTreeMap::new();
        let mut view_info_map: std::collections::BTreeMap<String, AbstractTableRepr> =
            std::collections::BTreeMap::new();
        let mut offset = 0;
        let pool = match config.db_pool {
            DbPool::Postgres(ref pool) => pool,
            _ => {
                return Err(CarpathiaError {
                    message: "Invalid database pool type for PostgreSQL querier".to_string(),
                    error_type:
                        crate::return_values::carpathia_errors::ErrorNumber::InvalidPoolType,
                });
            }
        };
        let constraint_map = Self::get_constraints(config).await?;
        debug!("Constraint map {:?}", constraint_map);
        // let type_map = &config.type_map.type_mapping;
        loop {
            let rows: Vec<PgColumnInfo> = sqlx::query_as::<_, PgColumnInfo>(SCHEMA_QUERY)
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
            for mut row in rows {
                let pg_object_type: PgObjectType = row.object_type.parse().unwrap_or_else(|_| {
                    debug!("Unknown object type: {}", row.object_type);
                    PgObjectType::Other
                });
                row = row.constraint_map(&constraint_map);
                debug!("Processing column: {}.{}", row.table_name, row.column_name);
                let table_name = row.table_name.clone();
                let object_type: ObjectType = pg_object_type.into();
                let attribute = AbstractAttribute::from(row.clone());
                match object_type {
                    ObjectType::BaseTable | ObjectType::PartitionedTable => {
                        table_info_map
                            .entry(table_name.clone())
                            .or_insert_with(|| AbstractTableRepr {
                                table_name: row.table_name.clone(),
                                u_table_name: String::new(),
                                u_imports: BTreeSet::new(),
                                object_type,
                                table_properties: BTreeSet::new(),
                                comment: row.table_comment.clone(),
                                attributes: BTreeMap::new(),
                            })
                            .attributes
                            .insert(attribute.column_name.clone(), attribute);
                        //insert_u_import(&mut table_info_map, &row.table_name, u_type_map);
                    }
                    ObjectType::View | ObjectType::MaterializedView => {
                        view_info_map
                            .entry(table_name.clone())
                            .or_insert_with(|| AbstractTableRepr {
                                table_name: row.table_name,
                                u_table_name: String::new(),
                                u_imports: BTreeSet::new(),
                                object_type,
                                table_properties: BTreeSet::new(),
                                comment: row.table_comment.clone(),
                                attributes: BTreeMap::new(),
                            })
                            .attributes
                            .insert(attribute.column_name.clone(), attribute);
                    }
                    _ => {
                        error!(
                            "Skipping unsupported object type: {:?} for table {}",
                            object_type, table_name
                        );
                    }
                }
            }
            offset += LIMIT;
            if num_rows < LIMIT as usize {
                break;
            }
        }

        Ok(AbstractDbRepr {
            version: ABSTRACT_DB_REPR_VERSION.to_string(),
            tables: table_info_map,
            views: view_info_map,
        })
    }
}
