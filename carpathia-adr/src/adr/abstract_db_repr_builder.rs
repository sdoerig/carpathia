//! Builder-Pattern for AbstractDbRepr (ADR).
//!
//! The builders are intentionally designed so that only the "raw" fields
//! (without the `u_` prefixes) need to be set - the `u_` attributes
//! are filled as usual by `enrich_adr`.
//!
//! Note on AI: The builder API in its basic form is AI generated from the ADR.
//! I think for this kind of code, AI genertion is a good idea but as a matter of 
//! intellectual fairness, this must be mentioned. 
//! The code itself is repetitive and the structure is regular and I'm 
//! not good at writing repetitive code. 
//! Moreover, for a less complicated builder one would use e.g. the `derive_builder` crate.
//! I think using this crate would also be possible but I would have to 
//! write a syntactical sugar layer on top of it to get the same Closure style as here.
//! -- Stefan Dörig, sdoerig@bluewin.ch, 2026-10-10
//! 
//! Typical usage (Closure style for nesting):
//! 
//! ```ignore
//! use carpathia_adr::adr::abstract_db_repr::*;
//! use carpathia_adr::adr::abstract_db_repr::builder::AbstractDbReprBuilder;
//!
//! let adr = AbstractDbReprBuilder::new()
//!     .table("users", |t| t
//!         .comment("Users table")
//!         .attribute("id", |a| a
//!             .data_type("integer")
//!             .not_nullable()
//!             .primary_key()
//!             .default("nextval('users_id_seq'::regclass)")
//!             .constraint(
//!                 ConstraintType::PrimaryKey,
//!                 AbstractConstraintBuilder::new("users_pkey").build(),
//!             )
//!             .comment("Primary key for users table")))
//!     .build();
//! ```
//!
//! The builders are intentionally designed so that only the "raw" fields
//! (without the `u_` prefixes) need to be set - the `u_` attributes
//! are filled as usual by `enrich_adr`.

use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;

use crate::adr::abstract_db_repr::{
    ABSTRACT_DB_REPR_VERSION, AbstractAttribute, AbstractConstraint, AbstractDbRepr,
    AbstractForeignKey, AbstractKey, AbstractPrimaryKey, AbstractReferencedTable,
    AbstractTableRepr, ConstraintType, ForeignKeySemantics, IsNullable, KeyType, ObjectType,
    PrimaryKeySemantics, TableProperties,
};

// ---------------------------------------------------------------------------
// AbstractDbRepr
// ---------------------------------------------------------------------------

/// Builder for [`AbstractDbRepr`].
#[derive(Debug, Clone)]
pub struct AbstractDbReprBuilder {
    version: String,
    tables: BTreeMap<String, AbstractTableRepr>,
    views: BTreeMap<String, AbstractTableRepr>,
}

impl Default for AbstractDbReprBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AbstractDbReprBuilder {
    /// New builder with the current ADR version (`ABSTRACT_DB_REPR_VERSION`).
    pub fn new() -> Self {
        Self {
            version: ABSTRACT_DB_REPR_VERSION.to_string(),
            tables: BTreeMap::new(),
            views: BTreeMap::new(),
        }
    }

    /// New builder with explicit ADR version.
    pub fn with_version(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            tables: BTreeMap::new(),
            views: BTreeMap::new(),
        }
    }

    /// Override the version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Add a table via a nested Table-Builder.
    /// (Closure style, for fluid readable construction).
    pub fn table(
        mut self,
        name: impl Into<String>,
        f: impl FnOnce(AbstractTableReprBuilder) -> AbstractTableReprBuilder,
    ) -> Self {
        let name = name.into();
        let table = f(AbstractTableReprBuilder::new(name.clone())).build();
        self.tables.insert(name, table);
        self
    }

    /// Add a view via a nested Table-Builder.
    pub fn view(
        mut self,
        name: impl Into<String>,
        f: impl FnOnce(AbstractTableReprBuilder) -> AbstractTableReprBuilder,
    ) -> Self {
        let name = name.into();
        let view =
            f(AbstractTableReprBuilder::new(name.clone()).object_type(ObjectType::View)).build();
        self.views.insert(name, view);
        self
    }

    /// Add a completed `AbstractTableRepr` as a table.
    pub fn with_table(mut self, table: AbstractTableRepr) -> Self {
        self.tables.insert(table.table_name.clone(), table);
        self
    }

    /// Add a completed `AbstractTableRepr` as a view.
    pub fn with_view(mut self, view: AbstractTableRepr) -> Self {
        self.views.insert(view.table_name.clone(), view);
        self
    }

    /// Build the `AbstractDbRepr`.
    pub fn build(self) -> AbstractDbRepr {
        AbstractDbRepr {
            version: self.version,
            tables: self.tables,
            views: self.views,
        }
    }
}

// ---------------------------------------------------------------------------
// AbstractTableRepr
// ---------------------------------------------------------------------------

/// Builder for [`AbstractTableRepr`].
///
/// The `u_table_name` value is initially set to the table name and
/// can - but doesn't have to - be overridden. Usually, one leaves this
/// to `enrich_adr`.
#[derive(Debug, Clone)]
pub struct AbstractTableReprBuilder {
    object_type: ObjectType,
    u_imports: BTreeSet<String>,
    table_name: String,
    u_table_name: String,
    table_properties: BTreeSet<TableProperties>,
    comment: Option<String>,
    attributes: BTreeMap<String, AbstractAttribute>,
}

impl AbstractTableReprBuilder {
    /// New table builder with `ObjectType::BaseTable` and no comment.
    pub fn new(table_name: impl Into<String>) -> Self {
        let table_name = table_name.into();
        let u_table_name = table_name.clone();
        Self {
            object_type: ObjectType::BaseTable,
            u_imports: BTreeSet::new(),
            table_name,
            u_table_name,
            table_properties: BTreeSet::new(),
            comment: None,
            attributes: BTreeMap::new(),
        }
    }

    /// Set the object type (BaseTable, View, MaterializedView, ...).
    pub fn object_type(mut self, object_type: ObjectType) -> Self {
        self.object_type = object_type;
        self
    }

    /// Set the comment (replaces an existing one).
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// Remove the comment.
    pub fn without_comment(mut self) -> Self {
        self.comment = None;
        self
    }

    /// Set the language-safe name - usually not necessary, `enrich_adr`
    /// takes care of the mapping.
    pub fn u_table_name(mut self, u_table_name: impl Into<String>) -> Self {
        self.u_table_name = u_table_name.into();
        self
    }

    /// Add an import (type mapping).
    pub fn u_import(mut self, u_import: impl Into<String>) -> Self {
        self.u_imports.insert(u_import.into());
        self
    }

    /// Add multiple imports (type mappings).
    pub fn u_imports<I, S>(mut self, u_imports: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.u_imports.extend(u_imports.into_iter().map(Into::into));
        self
    }

    /// Add a table property (e.g., `TableProperties::Selectable`).
    pub fn table_property(mut self, property: TableProperties) -> Self {
        self.table_properties.insert(property);
        self
    }

    /// Add a primary key property.
    pub fn primary_key(mut self, pk: AbstractPrimaryKey) -> Self {
        self.table_properties
            .insert(TableProperties::PrimaryKey(pk));
        self
    }

    /// Add a foreign key property.
    pub fn foreign_key(mut self, fk: AbstractForeignKey) -> Self {
        self.table_properties
            .insert(TableProperties::ForeignKey(fk));
        self
    }

    /// Add an attribute via a nested Attribute-Builder
    /// (Closure-Stil).
    pub fn attribute(
        mut self,
        name: impl Into<String>,
        f: impl FnOnce(AbstractAttributeBuilder) -> AbstractAttributeBuilder,
    ) -> Self {
        let name = name.into();
        let attribute = f(AbstractAttributeBuilder::new(name.clone())).build();
        self.attributes.insert(name, attribute);
        self
    }

    /// Add a pre-built `AbstractAttribute`.
    pub fn with_attribute(mut self, attribute: AbstractAttribute) -> Self {
        self.attributes
            .insert(attribute.column_name.clone(), attribute);
        self
    }

    /// Build the `AbstractTableRepr`.
    pub fn build(self) -> AbstractTableRepr {
        AbstractTableRepr {
            object_type: self.object_type,
            u_imports: self.u_imports,
            table_name: self.table_name,
            u_table_name: self.u_table_name,
            table_properties: self.table_properties,
            comment: self.comment,
            attributes: self.attributes,
        }
    }
}

// ---------------------------------------------------------------------------
// AbstractAttribute
// ---------------------------------------------------------------------------

/// Builder for [`AbstractAttribute`].
///
/// - `data_type` and `u_type` are required fields (`build` panics or
///   `try_build` returns a `MissingFieldError` if they are missing).
/// - `is_nullable` defaults to `IsNullable::No`; with `nullable()` it is
///   explicitly set to `Yes`.
/// - The `u_`-fields (`u_column_name`, `u_referenced_*`) are only
///   initially mirrored and are usually left to `enrich_adr`.
#[derive(Debug, Clone)]
pub struct AbstractAttributeBuilder {
    column_name: String,
    u_column_name: String,
    data_type: Option<String>,
    u_type: Option<String>,
    is_nullable: IsNullable,
    column_default: Option<String>,
    is_primary_key: bool,
    character_maximum_length: Option<i32>,
    numeric_precision: Option<i32>,
    numeric_scale: Option<i32>,
    constraints: BTreeMap<ConstraintType, AbstractConstraint>,
    comment: Option<String>,
}

#[derive(Debug)]
pub struct MissingFieldError {
    pub field: &'static str,
}

impl std::fmt::Display for MissingFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "abstract attribute is missing required field: {}",
            self.field
        )
    }
}

impl std::error::Error for MissingFieldError {}

impl AbstractAttributeBuilder {
    /// Create a new attribute builder.
    pub fn new(column_name: impl Into<String>) -> Self {
        let column_name = column_name.into();
        let u_column_name = column_name.clone();
        Self {
            column_name,
            u_column_name,
            data_type: None,
            u_type: None,
            is_nullable: IsNullable::No,
            column_default: None,
            is_primary_key: false,
            character_maximum_length: None,
            numeric_precision: None,
            numeric_scale: None,
            constraints: BTreeMap::new(),
            comment: None,
        }
    }

    /// Set the data type (mandatory field).
    pub fn data_type(mut self, data_type: impl Into<String>) -> Self {
        self.data_type = Some(data_type.into());
        self
    }

    /// Set the mapped type according to the type mapping (mandatory field).
    pub fn u_type(mut self, u_type: impl Into<String>) -> Self {
        self.u_type = Some(u_type.into());
        self
    }

    /// Set the nullable status to `Yes`.
    pub fn nullable(mut self) -> Self {
        self.is_nullable = IsNullable::Yes;
        self
    }

    /// Set the nullable status to `No` (Default des Builders).
    pub fn not_nullable(mut self) -> Self {
        self.is_nullable = IsNullable::No;
        self
    }

    /// Set the nullable status explizit (inkl. `Unknown`).
    pub fn is_nullable(mut self, is_nullable: IsNullable) -> Self {
        self.is_nullable = is_nullable;
        self
    }

    /// Set the default value of the column.
    pub fn default(mut self, default: impl Into<String>) -> Self {
        self.column_default = Some(default.into());
        self
    }

    /// Remove the default value.
    pub fn without_default(mut self) -> Self {
        self.column_default = None;
        self
    }

    /// Mark the column as a primary key column.
    pub fn primary_key(mut self) -> Self {
        self.is_primary_key = true;
        self
    }

    /// Set the primary key flag explicitly.
    pub fn is_primary_key(mut self, is_primary_key: bool) -> Self {
        self.is_primary_key = is_primary_key;
        self
    }

    /// Maximum character length.
    pub fn character_maximum_length(mut self, len: i32) -> Self {
        self.character_maximum_length = Some(len);
        self
    }

    /// Numeric precision.
    pub fn numeric_precision(mut self, precision: i32) -> Self {
        self.numeric_precision = Some(precision);
        self
    }

    /// Numeric scale.
    pub fn numeric_scale(mut self, scale: i32) -> Self {
        self.numeric_scale = Some(scale);
        self
    }

    /// Set the comment for the column.
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// Add constraints / replace existing constraints of the same type.
    pub fn constraint(
        mut self,
        constraint_type: ConstraintType,
        constraint: AbstractConstraint,
    ) -> Self {
        self.constraints.insert(constraint_type, constraint);
        self
    }

    /// Add constraint via Closure-Builder.
    pub fn constraint_with(
        mut self,
        constraint_type: ConstraintType,
        f: impl FnOnce(AbstractConstraintBuilder) -> AbstractConstraintBuilder,
    ) -> Self {
        let constraint = f(AbstractConstraintBuilder::empty()).build();
        self.constraints.insert(constraint_type, constraint);
        self
    }

    /// Build the `AbstractAttribute`, panicking if required fields are missing.
    /// you should normally use `try_build` and handle the error.
    pub fn build(self) -> AbstractAttribute {
        self.try_build()
            .expect("AbstractAttributeBuilder: `data_type` and `u_type` are required")
    }

    /// Catching error-handling variant of `build`.
    pub fn try_build(self) -> Result<AbstractAttribute, MissingFieldError> {
        let data_type = self
            .data_type
            .ok_or(MissingFieldError { field: "data_type" })?;
        let u_type = self.u_type.ok_or(MissingFieldError { field: "u_type" })?;
        Ok(AbstractAttribute {
            column_name: self.column_name,
            u_column_name: self.u_column_name,
            data_type,
            u_type,
            is_nullable: self.is_nullable,
            column_default: self.column_default,
            is_primary_key: self.is_primary_key,
            character_maximum_length: self.character_maximum_length,
            numeric_precision: self.numeric_precision,
            numeric_scale: self.numeric_scale,
            constraints: self.constraints,
            comment: self.comment,
        })
    }
}

// ---------------------------------------------------------------------------
// AbstractConstraint
// ---------------------------------------------------------------------------

/// Builder for [`AbstractConstraint`].
///
/// Name and Value are required, the referenced fields are optional -
/// exactly analogous to the `skip_serializing_if` fields in the struct.
#[derive(Debug, Clone)]
pub struct AbstractConstraintBuilder {
    constraint_name: String,
    constraint_value: String,
    referenced_schema_name: Option<String>,
    referenced_table: Option<String>,
    u_referenced_table: Option<String>,
    referenced_column: Option<String>,
    u_referenced_column: Option<String>,
}

impl AbstractConstraintBuilder {
    /// New constraint builder with name and value.
    pub fn new(constraint_name: impl Into<String>) -> Self {
        Self {
            constraint_name: constraint_name.into(),
            constraint_value: String::new(),
            referenced_schema_name: None,
            referenced_table: None,
            u_referenced_table: None,
            referenced_column: None,
            u_referenced_column: None,
        }
    }

    /// Empty builder (Name/Value are set via `with_*`/Closure).
    pub fn empty() -> Self {
        Self::new("")
    }

    /// Constraint value (e.g., `PRIMARY KEY (actor_id)`).
    pub fn value(mut self, constraint_value: impl Into<String>) -> Self {
        self.constraint_value = constraint_value.into();
        self
    }

    /// Referenced schema.
    pub fn referenced_schema(mut self, schema: impl Into<String>) -> Self {
        self.referenced_schema_name = Some(schema.into());
        self
    }

    /// Referenced table (including the `u_` variant).
    pub fn referenced_table(mut self, table: impl Into<String>) -> Self {
        self.referenced_table = Some(table.into());
        self
    }

    /// Language-safe name of the referenced table (optional,
    /// usually the responsibility of `enrich_adr`).
    pub fn u_referenced_table(mut self, u_table: impl Into<String>) -> Self {
        self.u_referenced_table = Some(u_table.into());
        self
    }

    /// Referenced column.
    pub fn referenced_column(mut self, column: impl Into<String>) -> Self {
        self.referenced_column = Some(column.into());
        self
    }

    /// Language-safe name of the referenced column (optional).
    pub fn u_referenced_column(mut self, u_column: impl Into<String>) -> Self {
        self.u_referenced_column = Some(u_column.into());
        self
    }

    /// Builds the `AbstractConstraint`.
    pub fn build(self) -> AbstractConstraint {
        AbstractConstraint {
            constraint_name: self.constraint_name,
            constraint_value: self.constraint_value,
            referenced_schema_name: self.referenced_schema_name,
            referenced_table: self.referenced_table,
            u_referenced_table: self.u_referenced_table,
            referenced_column: self.referenced_column,
            u_referenced_column: self.u_referenced_column,
        }
    }
}

// ---------------------------------------------------------------------------
// AbstractKey (Primary / Foreign)
// ---------------------------------------------------------------------------

/// Builder for [`AbstractPrimaryKey`] and [`AbstractForeignKey`].
///
/// A key consists of a set of `AbstractReferencedTable` entries
/// (columns), which are added here row by row.
#[derive(Debug, Clone)]
pub struct AbstractKeyBuilder<K> {
    columns: BTreeSet<AbstractReferencedTable>,
    _marker: PhantomData<K>,
}

/// Builder for a primary key.
pub type AbstractPrimaryKeyBuilder = AbstractKeyBuilder<PrimaryKeySemantics>;
/// Builder for a foreign key.
pub type AbstractForeignKeyBuilder = AbstractKeyBuilder<ForeignKeySemantics>;

impl<K> AbstractKeyBuilder<K> {
    /// Empty key builder.
    pub fn new() -> Self {
        Self {
            columns: BTreeSet::new(),
            _marker: PhantomData,
        }
    }

    /// Adds a key column. `key_type` is set to `SingleColumn`;
    /// a subsequent `+` (Add) corrects it to
    /// `MultiColumn`, as specified in the ADR.
    pub fn column(mut self, constraint_name: impl Into<String>, column: impl Into<String>) -> Self {
        let column_name = column.into();
        self.columns.insert(AbstractReferencedTable {
            constraint_name: constraint_name.into(),
            key_type: KeyType::SingleColumn,
            column: column_name.clone(),
            u_column: column_name,
            referenced_table: None,
            u_referenced_table: None,
            referenced_column: None,
            u_referenced_column: None,
        });
        self
    }

    /// Adds a key column including a reference (for Foreign Keys).
    pub fn referencing_column(
        mut self,
        constraint_name: impl Into<String>,
        column: impl Into<String>,
        referenced_table: impl Into<String>,
        referenced_column: impl Into<String>,
    ) -> Self {
        let column_name = column.into();
        self.columns.insert(AbstractReferencedTable {
            constraint_name: constraint_name.into(),
            key_type: KeyType::SingleColumn,
            column: column_name.clone(),
            u_column: column_name,
            referenced_table: Some(referenced_table.into()),
            u_referenced_table: None,
            referenced_column: Some(referenced_column.into()),
            u_referenced_column: None,
        });
        self
    }

    /// Builds the key.
    pub fn build(self) -> AbstractKey<K> {
        AbstractKey {
            columns: self.columns,
            _marker: PhantomData,
        }
    }
}

impl<K> Default for AbstractKeyBuilder<K> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::adr::abstract_db_repr::*;
    use crate::adr::abstract_db_repr_builder::*;

    #[test]
    fn builds_db_repr_like_the_doc_example() {
        let adr = AbstractDbReprBuilder::new()
            .table("users", |t| {
                t.comment("Users table")
                    .attribute("id", |a| {
                        a.data_type("integer")
                            .u_type("whatever")
                            .not_nullable()
                            .primary_key()
                            .default("nextval('users_id_seq'::regclass)")
                            .comment("Primary key for users table")
                            .constraint(
                                ConstraintType::PrimaryKey,
                                AbstractConstraintBuilder::new("users_pkey").build(),
                            )
                    })
                    .attribute("name", |a| {
                        a.data_type("varchar").u_type("string").nullable()
                    })
            })
            .build();

        assert_eq!(adr.version, ABSTRACT_DB_REPR_VERSION);
        assert_eq!(adr.tables.len(), 1);
        assert!(adr.views.is_empty());

        let users = adr.tables.get("users").unwrap();
        assert_eq!(users.table_name, "users");
        assert_eq!(users.comment.as_deref(), Some("Users table"));
        assert_eq!(users.attributes.len(), 2);

        let id = users.attributes.get("id").unwrap();
        assert!(id.is_primary_key);
        assert_eq!(id.is_nullable, IsNullable::No);
        assert!(id.constraints.contains_key(&ConstraintType::PrimaryKey));

        let name = users.attributes.get("name").unwrap();
        assert_eq!(name.is_nullable, IsNullable::Yes);
        assert!(!name.is_primary_key);
        assert_eq!(name.column_default, None);
    }

    #[test]
    fn builds_views() {
        let adr = AbstractDbReprBuilder::with_version("0.2.0")
            .view("v_users", |v| v.comment("all users"))
            .build();

        assert_eq!(adr.version, "0.2.0");
        let view = adr.views.get("v_users").unwrap();
        assert_eq!(view.object_type, ObjectType::View);
    }

    #[test]
    fn builds_primary_key_property() {
        let pk = AbstractPrimaryKeyBuilder::new()
            .column("users_pkey", "id")
            .build();

        let adr = AbstractDbReprBuilder::new()
            .table("users", |t| t.primary_key(pk))
            .build();

        let users = adr.tables.get("users").unwrap();
        assert!(
            users
                .table_properties
                .iter()
                .any(|p| matches!(p, TableProperties::PrimaryKey(_)))
        );
    }

    #[test]
    fn builds_foreign_key_property() {
        let fk = AbstractForeignKeyBuilder::new()
            .referencing_column("fk_address", "address_id", "addresses", "id")
            .build();

        assert_eq!(fk.columns.len(), 1);
        let col = fk.columns.iter().next().unwrap();
        assert_eq!(col.constraint_name, "fk_address");
        assert_eq!(col.referenced_table.as_deref(), Some("addresses"));
        assert_eq!(col.key_type, KeyType::SingleColumn);

        let adr = AbstractDbReprBuilder::new()
            .table("users", |t| t.foreign_key(fk))
            .build();
        let users = adr.tables.get("users").unwrap();
        assert!(
            users
                .table_properties
                .iter()
                .any(|p| matches!(p, TableProperties::ForeignKey(_)))
        );
    }

    #[test]
    fn try_build_reports_missing_required_fields() {
        let result = AbstractAttributeBuilder::new("id")
            .u_type("i32")
            .try_build();
        assert!(result.is_err());
        let result = AbstractAttributeBuilder::new("id")
            .data_type("integer")
            .try_build();
        assert!(result.is_err());
        assert!(
            AbstractAttributeBuilder::new("id")
                .data_type("integer")
                .u_type("i32")
                .try_build()
                .is_ok()
        );
    }

    #[test]
    fn table_builder_defaults() {
        let t = AbstractTableReprBuilder::new("orders").build();
        assert_eq!(t.object_type, ObjectType::BaseTable);
        assert_eq!(t.u_table_name, "orders");
        assert!(t.attributes.is_empty());
        assert!(t.table_properties.is_empty());
        assert!(t.u_imports.is_empty());
        assert!(t.comment.is_none());
    }

    #[test]
    fn key_builder_single_and_multi_column() {
        let key = AbstractForeignKeyBuilder::new()
            .column("fk", "a")
            .column("fk", "b")
            .build();
        // Wie im ADR-Add: gleicher Constraint-Name über zwei Spalten
        let merged = key.clone() + key;
        assert_eq!(merged.columns.len(), 2);
    }
}
