use std::{borrow::Cow, collections::HashMap};

use facet::Facet;

#[derive(Facet, Clone)]
pub struct Schema {
    /// Schema name (e.g. "public", "pg_catalog")
    pub name: String,
    pub tables: HashMap<String, Table>,
    pub views: HashMap<String, View>,
    pub materialized_views: HashMap<String, MaterializedView>,
    pub enums: HashMap<String, EnumType>,
    pub domains: HashMap<String, DomainType>,
    pub composite_types: HashMap<String, CompositeType>,
    pub sequences: HashMap<String, Sequence>,
    pub collations: HashMap<String, Collation>,
    pub functions: HashMap<String, FunctionSignature>, // lightweight representation
    /// Optional schema-level comments or metadata.
    pub comment: Option<String>,
}

#[derive(Facet, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    /// Primary key if present
    pub primary_key: Option<PrimaryKey>,
    /// Other unique constraints
    pub uniques: Vec<UniqueConstraint>,
    /// Foreign keys
    pub foreign_keys: Vec<ForeignKey>,
    /// Check constraints
    pub checks: Vec<CheckConstraint>,
    /// Indexes (including partials)
    pub indexes: Vec<Index>,
    /// Table-level options (like partitioned, inherits, tablespace)
    pub options: TableOptions,
    /// Table-level comment
    pub comment: Option<String>,
    /// Owned sequences (name -> owned_by column)
    pub owned_sequences: Vec<String>,
}

#[derive(Facet, Clone)]
pub struct View {
    pub name: String,
    pub columns: Vec<Column>,
    pub definition: String, // the SQL SELECT or view definition
    pub materialized: bool,
    pub check_option: Option<ViewCheckOption>,
    pub comment: Option<String>,
}

#[derive(Facet, Clone)]
pub struct MaterializedView {
    pub name: String,
    pub columns: Vec<Column>,
    pub definition: String,
    pub comment: Option<String>,
}

/// Column definition
#[derive(Facet, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub default: Option<String>, // raw SQL expression for default
    pub nullable: bool,
    pub collation: Option<String>,
    pub is_generated: bool, // stored/generated column
    pub generation_expression: Option<String>,
    pub is_identity: bool,
    pub identity_generation: Option<IdentityGeneration>, // ALWAYS or BY DEFAULT
    pub comment: Option<String>,
    pub privileges: Option<Privileges>,
}

/// PostgreSQL data types: builtins, arrays, enums, composite, domains, user-defined
#[derive(Facet, Clone)]
#[repr(C)]
pub enum DataType {
    // Common scalar types
    Boolean,
    SmallInt,
    Integer,
    BigInt,
    Real,
    DoublePrecision,
    Numeric {
        precision: Option<u32>,
        scale: Option<u32>,
    },
    Serial,
    BigSerial,
    Text,
    Varchar(Option<u32>),
    Char(Option<u32>),
    Bytea,
    Timestamp {
        with_time_zone: bool,
    },
    Date,
    Time {
        with_time_zone: bool,
    },
    Interval,
    Json,
    Jsonb,
    Uuid,
    Inet,
    MacAddr,
    TsVector,
    // Arrays of other types
    Array(Box<DataType>),
    // User-created enum type in a schema
    Enum {
        schema: Option<String>,
        name: String,
    },
    // Composite (row) type defined by name
    Composite {
        schema: Option<String>,
        name: String,
    },
    // Domain type
    Domain {
        schema: Option<String>,
        name: String,
    },
    // Range, custom, or extension types by name
    Custom {
        schema: Option<String>,
        name: String,
    },
    // Any or Unknown
    Any,
    Unknown,
}

/// Identity generation options for `GENERATED { ALWAYS | BY DEFAULT } AS IDENTITY`
#[derive(Facet, Clone)]
#[repr(C)]
pub enum IdentityGeneration {
    Always,
    ByDefault,
}

/// Primary key representation
#[derive(Facet, Clone)]
pub struct PrimaryKey {
    pub name: Option<String>,
    /// Column names in PK order
    pub columns: Vec<String>,
    /// Whether the PK is deferrable (rare) and initial state
    pub deferrable: Option<Deferrability>,
    /// Optional storage parameters or using clause (rare)
    pub using: Option<String>,
}

/// UNIQUE constraint
#[derive(Facet, Clone)]
pub struct UniqueConstraint {
    pub name: Option<String>,
    pub columns: Vec<String>,
    pub deferrable: Option<Deferrability>,
}

/// Foreign key
#[derive(Facet, Clone)]
pub struct ForeignKey {
    pub name: Option<String>,
    /// referencing local columns (in order)
    pub columns: Vec<String>,
    /// referenced table (schema, name)
    pub referenced_table: QualifiedName,
    /// referenced columns
    pub referenced_columns: Option<Vec<String>>, // None -> primary key
    pub on_delete: Option<ReferentialAction>,
    pub on_update: Option<ReferentialAction>,
    pub match_type: Option<MatchType>,
    pub deferrable: Option<Deferrability>,
    pub initially: Option<Initially>, // INITIALLY DEFERRED/IMMEDIATE
}

/// Helper representing qualified object names
#[derive(Facet, Clone)]
pub struct QualifiedName {
    pub schema: Option<String>,
    pub name: String,
}

#[derive(Facet, Clone)]
#[repr(C)]
pub enum ReferentialAction {
    NoAction,
    Restrict,
    Cascade,
    SetNull,
    SetDefault,
}

#[derive(Facet, Clone)]
#[repr(C)]
pub enum MatchType {
    Simple,
    Full,
    Partial,
}

#[derive(Facet, Clone)]
#[repr(C)]
pub enum Initially {
    Deferred,
    Immediate,
}

#[derive(Facet, Clone)]
#[repr(C)]
pub enum Deferrability {
    Deferrable,
    NotDeferrable,
}

/// Check constraint
#[derive(Facet, Clone)]
pub struct CheckConstraint {
    pub name: Option<String>,
    pub expression: String, // raw SQL CHECK expression
    pub no_inherit: bool,   // NO INHERIT option for some use cases
}

/// Index definition
#[derive(Facet, Clone)]
pub struct Index {
    pub name: String,
    pub columns: Vec<IndexColumn>,
    pub unique: bool,
    pub method: Option<String>,    // e.g. "btree", "gin", "gist"
    pub predicate: Option<String>, // partial index predicate (WHERE ...)
    pub include: Vec<String>,      // included columns (Postgres INCLUDE)
    pub tablespace: Option<String>,
    pub concurrently: bool,
    pub is_primary: bool, // sometimes indexes back PKs
    pub is_valid: bool,
}

/// A column within an index: either a plain column, expression or operator class
#[derive(Facet, Clone)]
pub struct IndexColumn {
    pub expr: IndexExpr,
    pub collate: Option<String>,
    pub opclass: Option<String>,
    pub order: Option<SortOrder>,
    pub nulls_order: Option<NullsOrder>,
}

#[derive(Facet, Clone)]
#[repr(C)]
pub enum IndexExpr {
    Column(String),
    Expression(String),
}

#[derive(Facet, Clone)]
#[repr(C)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Facet, Clone)]
#[repr(C)]
pub enum NullsOrder {
    First,
    Last,
}

/// Sequence definition
#[derive(Facet, Clone)]
pub struct Sequence {
    pub name: String,
    pub schema: Option<String>,
    pub owned_by: Option<QualifiedColumn>, // table.column if owned
    pub start: Option<i64>,
    pub increment: Option<i64>,
    pub min_value: Option<i64>,
    pub max_value: Option<i64>,
    pub cache: Option<i64>,
    pub cycle: bool,
    pub comment: Option<String>,
}

#[derive(Facet, Clone)]
pub struct QualifiedColumn {
    pub schema: Option<String>,
    pub table: String,
    pub column: String,
}

/// Enum type definition
#[derive(Facet, Clone)]
pub struct EnumType {
    pub schema: Option<String>,
    pub name: String,
    pub variants: Vec<String>,
    pub comment: Option<String>,
}

/// Domain type (wraps a base type with constraints)
#[derive(Facet, Clone)]
pub struct DomainType {
    pub schema: Option<String>,
    pub name: String,
    pub base_type: DataType,
    pub default: Option<String>,
    pub not_null: bool,
    pub constraints: Vec<CheckConstraint>,
    pub comment: Option<String>,
}

/// Composite type (record)
#[derive(Facet, Clone)]
pub struct CompositeType {
    pub schema: Option<String>,
    pub name: String,
    pub fields: Vec<Column>,
    pub comment: Option<String>,
}

#[derive(Facet, Clone)]
pub struct Collation {
    pub schema: Option<String>,
    pub name: String,
    pub provider: Option<String>,
    pub locale: Option<String>,
    pub deterministic: Option<bool>,
}

/// Lightweight function signature representation
#[derive(Facet, Clone)]
pub struct FunctionSignature {
    pub schema: Option<String>,
    pub name: String,
    pub args: Vec<DataType>,
    pub return_type: DataType,
    pub language: Option<String>,
    pub volatile: Option<FunctionVolatility>,
}

#[derive(Facet, Clone)]
#[repr(C)]
pub enum FunctionVolatility {
    Immutable,
    Stable,
    Volatile,
}

/// Table options (storage, inheritance, partitioning)
#[derive(Facet, Clone)]
pub struct TableOptions {
    pub inherits: Vec<QualifiedName>,
    pub temporary: bool,
    pub unlogged: bool,
    pub partitioned: Option<bool>,
    pub tablespace: Option<String>,
    pub with_storage_params: HashMap<String, String>,
}

/// View check option
#[derive(Facet, Clone)]
#[repr(C)]
pub enum ViewCheckOption {
    Local,
    Cascaded,
}

/// Privileges (grants) -- simplified
#[derive(Facet, Clone)]
pub struct Privileges {
    pub owner: Option<String>,
    /// Map of grantee -> list of privileges like SELECT, INSERT, UPDATE...
    pub grants: HashMap<String, Vec<String>>,
}

/// Basic representation for things like sequences owned by a column
#[derive(Facet, Clone)]
pub struct QualifiedObject {
    pub schema: Option<String>,
    pub name: String,
}

/// Example: small helper trait (not required) for convenience
impl QualifiedName {
    pub fn to_string(&self) -> String {
        match &self.schema {
            Some(s) => format!("{}.{}", s, self.name),
            None => self.name.clone(),
        }
    }
}

impl Schema {
    /// Render a simplistic SQL DDL representation of this schema.
    ///
    /// This is not a full-featured DDL generator for every Postgres nuance,
    /// but it attempts to emit reasonable CREATE statements for:
    /// - CREATE SCHEMA
    /// - types (enum, composite), domains, sequences
    /// - CREATE TABLE with columns and primary key (uniques/checks/fks added with ALTER TABLE)
    /// - views / materialized views
    /// - comments
    ///
    /// The output is deterministic (maps are iterated in sorted order).
    pub fn to_ddl(&self) -> String {
        fn esc(s: &str) -> String {
            s.replace('\'', "''")
        }

        fn render_data_type(dt: &DataType) -> String {
            match dt {
                DataType::Boolean => "boolean".into(),
                DataType::SmallInt => "smallint".into(),
                DataType::Integer => "integer".into(),
                DataType::BigInt => "bigint".into(),
                DataType::Real => "real".into(),
                DataType::DoublePrecision => "double precision".into(),
                DataType::Numeric { precision, scale } => match (precision, scale) {
                    (Some(p), Some(s)) => format!("numeric({},{})", p, s),
                    (Some(p), None) => format!("numeric({})", p),
                    _ => "numeric".into(),
                },
                DataType::Serial => "serial".into(),
                DataType::BigSerial => "bigserial".into(),
                DataType::Text => "text".into(),
                DataType::Varchar(opt) => match opt {
                    Some(n) => format!("varchar({})", n),
                    None => "varchar".into(),
                },
                DataType::Char(opt) => match opt {
                    Some(n) => format!("char({})", n),
                    None => "char".into(),
                },
                DataType::Bytea => "bytea".into(),
                DataType::Timestamp { with_time_zone } => {
                    if *with_time_zone {
                        "timestamp with time zone".into()
                    } else {
                        "timestamp without time zone".into()
                    }
                }
                DataType::Date => "date".into(),
                DataType::Time { with_time_zone } => {
                    if *with_time_zone {
                        "time with time zone".into()
                    } else {
                        "time without time zone".into()
                    }
                }
                DataType::Interval => "interval".into(),
                DataType::Json => "json".into(),
                DataType::Jsonb => "jsonb".into(),
                DataType::Uuid => "uuid".into(),
                DataType::Inet => "inet".into(),
                DataType::MacAddr => "macaddr".into(),
                DataType::TsVector => "tsvector".into(),
                DataType::Array(inner) => format!("{}[]", render_data_type(inner)),
                DataType::Enum { schema, name } => match schema {
                    Some(s) => format!("{}.{}", s, name),
                    None => name.clone(),
                },
                DataType::Composite { schema, name } => match schema {
                    Some(s) => format!("{}.{}", s, name),
                    None => name.clone(),
                },
                DataType::Domain { schema, name } => match schema {
                    Some(s) => format!("{}.{}", s, name),
                    None => name.clone(),
                },
                DataType::Custom { schema, name } => match schema {
                    Some(s) => format!("{}.{}", s, name),
                    None => name.clone(),
                },
                DataType::Any => "any".into(),
                DataType::Unknown => "unknown".into(),
            }
        }

        let mut stmts: Vec<String> = Vec::new();

        stmts.push(format!("CREATE SCHEMA IF NOT EXISTS {};", self.name));
        if let Some(c) = &self.comment {
            stmts.push(format!("COMMENT ON SCHEMA {} IS '{}';", self.name, esc(c)));
        }

        // Enums
        let mut enum_keys: Vec<_> = self.enums.keys().cloned().collect();
        enum_keys.sort();
        for k in enum_keys {
            if let Some(e) = self.enums.get(&k) {
                let vars = e
                    .variants
                    .iter()
                    .map(|v| format!("'{}'", esc(v)))
                    .collect::<Vec<_>>()
                    .join(", ");
                let qname = if let Some(s) = &e.schema {
                    format!("{}.{}", s, e.name)
                } else {
                    e.name.clone()
                };
                stmts.push(format!("CREATE TYPE {} AS ENUM ({});", qname, vars));
                if let Some(c) = &e.comment {
                    stmts.push(format!("COMMENT ON TYPE {} IS '{}';", qname, esc(c)));
                }
            }
        }

        // Sequences
        let mut seq_keys: Vec<_> = self.sequences.keys().cloned().collect();
        seq_keys.sort();
        for k in seq_keys {
            if let Some(seq) = self.sequences.get(&k) {
                let q = if let Some(s) = &seq.schema {
                    format!("{}.{}", s, seq.name)
                } else {
                    seq.name.clone()
                };
                let mut parts: Vec<String> = vec![format!("CREATE SEQUENCE {}", q)];
                if let Some(start) = seq.start {
                    parts.push(format!("START WITH {}", start));
                }
                if let Some(inc) = seq.increment {
                    parts.push(format!("INCREMENT BY {}", inc));
                }
                if let Some(minv) = seq.min_value {
                    parts.push(format!("MINVALUE {}", minv));
                }
                if let Some(maxv) = seq.max_value {
                    parts.push(format!("MAXVALUE {}", maxv));
                }
                if let Some(cache) = seq.cache {
                    parts.push(format!("CACHE {}", cache));
                }
                if seq.cycle {
                    parts.push("CYCLE".into());
                } else {
                    parts.push("NO CYCLE".into());
                }
                let stmt = format!("{};", parts.join(" "));
                stmts.push(stmt);
                if let Some(c) = &seq.comment {
                    stmts.push(format!("COMMENT ON SEQUENCE {} IS '{}';", q, esc(c)));
                }
            }
        }

        // Composite types
        let mut comp_keys: Vec<_> = self.composite_types.keys().cloned().collect();
        comp_keys.sort();
        for k in comp_keys {
            if let Some(ct) = self.composite_types.get(&k) {
                let q = if let Some(s) = &ct.schema {
                    format!("{}.{}", s, ct.name)
                } else {
                    ct.name.clone()
                };
                let fields = ct
                    .fields
                    .iter()
                    .map(|f| format!("{} {}", f.name, render_data_type(&f.data_type)))
                    .collect::<Vec<_>>()
                    .join(", ");
                stmts.push(format!("CREATE TYPE {} AS ({});", q, fields));
                if let Some(c) = &ct.comment {
                    stmts.push(format!("COMMENT ON TYPE {} IS '{}';", q, esc(c)));
                }
            }
        }

        // Domains
        let mut dom_keys: Vec<_> = self.domains.keys().cloned().collect();
        dom_keys.sort();
        for k in dom_keys {
            if let Some(dom) = self.domains.get(&k) {
                let q = if let Some(s) = &dom.schema {
                    format!("{}.{}", s, dom.name)
                } else {
                    dom.name.clone()
                };
                let mut line = format!(
                    "CREATE DOMAIN {} AS {}",
                    q,
                    render_data_type(&dom.base_type)
                );
                if dom.not_null {
                    line.push_str(" NOT NULL");
                }
                if let Some(d) = &dom.default {
                    line.push_str(&format!(" DEFAULT {}", d));
                }
                line.push(';');
                stmts.push(line);
                if let Some(c) = &dom.comment {
                    stmts.push(format!("COMMENT ON DOMAIN {} IS '{}';", q, esc(c)));
                }
            }
        }

        // Tables
        let mut tbl_keys: Vec<_> = self.tables.keys().cloned().collect();
        tbl_keys.sort();
        for k in tbl_keys {
            if let Some(t) = self.tables.get(&k) {
                let q = format!("{}.{}", self.name, t.name);
                let cols = t
                    .columns
                    .iter()
                    .map(|c| {
                        let mut col = format!("{} {}", c.name, render_data_type(&c.data_type));
                        if let Some(coll) = &c.collation {
                            col.push_str(&format!(" COLLATE {}", coll));
                        }
                        if c.is_identity {
                            let r#gen = match c.identity_generation {
                                Some(IdentityGeneration::Always) => "ALWAYS",
                                Some(IdentityGeneration::ByDefault) => "BY DEFAULT",
                                None => "BY DEFAULT",
                            };
                            col.push_str(&format!(" GENERATED {} AS IDENTITY", r#gen));
                        } else if c.is_generated {
                            if let Some(expr) = &c.generation_expression {
                                col.push_str(&format!(" GENERATED ALWAYS AS ({}) STORED", expr));
                            }
                        } else if let Some(def) = &c.default {
                            col.push_str(&format!(" DEFAULT {}", def));
                        }
                        if !c.nullable {
                            col.push_str(" NOT NULL");
                        }
                        col
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let mut table_stmt = format!("CREATE TABLE {} ({})", q, cols);
                if let Some(pk) = &t.primary_key {
                    let cols = pk.columns.join(", ");
                    table_stmt.push_str(&format!(", PRIMARY KEY ({})", cols));
                }
                table_stmt.push(';');
                stmts.push(table_stmt);

                // Unique constraints
                if !t.uniques.is_empty() {
                    for u in &t.uniques {
                        let name =
                            u.name
                                .as_deref()
                                .map(|x| Cow::Borrowed(x))
                                .unwrap_or_else(|| {
                                    format!("{}_{}_key", t.name, u.columns.join("_")).into()
                                });
                        stmts.push(format!(
                            "ALTER TABLE {} ADD CONSTRAINT {} UNIQUE ({});",
                            q,
                            name,
                            u.columns.join(", ")
                        ));
                    }
                }

                // Check constraints
                if !t.checks.is_empty() {
                    for ck in &t.checks {
                        if let Some(nm) = &ck.name {
                            stmts.push(format!(
                                "ALTER TABLE {} ADD CONSTRAINT {} CHECK ({});",
                                q, nm, ck.expression
                            ));
                        } else {
                            stmts.push(format!("ALTER TABLE {} ADD CHECK ({});", q, ck.expression));
                        }
                    }
                }

                // Foreign keys
                if !t.foreign_keys.is_empty() {
                    for fk in &t.foreign_keys {
                        let name =
                            fk.name
                                .as_deref()
                                .map(|x| Cow::Borrowed(x))
                                .unwrap_or_else(|| {
                                    format!("{}_{}_fkey", t.name, fk.columns.join("_")).into()
                                });
                        let ref_t = fk.referenced_table.to_string();
                        let cols = fk.columns.join(", ");
                        let refcols = match &fk.referenced_columns {
                            Some(v) => format!("({})", v.join(", ")),
                            None => String::new(),
                        };
                        let mut stmt = format!(
                            "ALTER TABLE {} ADD CONSTRAINT {} FOREIGN KEY ({}) REFERENCES {}{}",
                            q,
                            name,
                            cols,
                            ref_t,
                            if refcols.is_empty() {
                                "".into()
                            } else {
                                format!(" {}", refcols)
                            }
                        );
                        if let Some(action) = &fk.on_delete {
                            let a = match action {
                                ReferentialAction::NoAction => "NO ACTION",
                                ReferentialAction::Restrict => "RESTRICT",
                                ReferentialAction::Cascade => "CASCADE",
                                ReferentialAction::SetNull => "SET NULL",
                                ReferentialAction::SetDefault => "SET DEFAULT",
                            };
                            stmt.push_str(&format!(" ON DELETE {}", a));
                        }
                        if let Some(action) = &fk.on_update {
                            let a = match action {
                                ReferentialAction::NoAction => "NO ACTION",
                                ReferentialAction::Restrict => "RESTRICT",
                                ReferentialAction::Cascade => "CASCADE",
                                ReferentialAction::SetNull => "SET NULL",
                                ReferentialAction::SetDefault => "SET DEFAULT",
                            };
                            stmt.push_str(&format!(" ON UPDATE {}", a));
                        }
                        stmt.push(';');
                        stmts.push(stmt);
                    }
                }
            }
        }

        // Views (from views map)
        let mut view_keys: Vec<_> = self.views.keys().cloned().collect();
        view_keys.sort();
        for k in view_keys {
            if let Some(v) = self.views.get(&k) {
                let q = format!("{}.{}", self.name, v.name);
                let stmt = if v.materialized {
                    format!("CREATE MATERIALIZED VIEW {} AS\n{};", q, v.definition)
                } else {
                    format!("CREATE VIEW {} AS\n{};", q, v.definition)
                };
                stmts.push(stmt);
                if let Some(c) = &v.comment {
                    stmts.push(format!("COMMENT ON VIEW {} IS '{}';", q, esc(c)));
                }
            }
        }

        // materialized_views map (if separate)
        let mut mview_keys: Vec<_> = self.materialized_views.keys().cloned().collect();
        mview_keys.sort();
        for k in mview_keys {
            if let Some(mv) = self.materialized_views.get(&k) {
                let q = format!("{}.{}", self.name, mv.name);
                stmts.push(format!(
                    "CREATE MATERIALIZED VIEW {} AS\n{};",
                    q, mv.definition
                ));
                if let Some(c) = &mv.comment {
                    stmts.push(format!(
                        "COMMENT ON MATERIALIZED VIEW {} IS '{}';",
                        q,
                        esc(c)
                    ));
                }
            }
        }

        stmts.join("\n")
    }
}

/// Tests / Example usage (not exhaustive)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_schema_roundtrip() {
        let mut schema = Schema {
            name: "public".to_string(),
            tables: Default::default(),
            views: Default::default(),
            materialized_views: Default::default(),
            enums: Default::default(),
            domains: Default::default(),
            composite_types: Default::default(),
            sequences: Default::default(),
            collations: Default::default(),
            functions: Default::default(),
            comment: None,
        };

        let table = Table {
            name: "users".to_string(),
            columns: vec![
                Column {
                    name: "id".to_string(),
                    data_type: DataType::BigSerial,
                    default: None,
                    nullable: false,
                    collation: None,
                    is_generated: false,
                    generation_expression: None,
                    is_identity: false,
                    identity_generation: None,
                    comment: None,
                    privileges: None,
                },
                Column {
                    name: "email".to_string(),
                    data_type: DataType::Varchar(Some(255)),
                    default: None,
                    nullable: false,
                    collation: None,
                    is_generated: false,
                    generation_expression: None,
                    is_identity: false,
                    identity_generation: None,
                    comment: None,
                    privileges: None,
                },
            ],
            primary_key: Some(PrimaryKey {
                name: Some("users_pkey".to_string()),
                columns: vec!["id".to_string()],
                deferrable: None,
                using: None,
            }),
            uniques: vec![UniqueConstraint {
                name: Some("users_email_key".to_string()),
                columns: vec!["email".to_string()],
                deferrable: None,
            }],
            foreign_keys: vec![],
            checks: vec![],
            indexes: vec![],
            options: TableOptions {
                inherits: vec![],
                temporary: false,
                unlogged: false,
                partitioned: None,
                tablespace: None,
                with_storage_params: Default::default(),
            },
            comment: Some("Application users".to_string()),
            owned_sequences: vec![],
        };

        schema.tables.insert(table.name.clone(), table);

        // Render DDL and assert it contains the expected CREATE TABLE line.
        let ddl = schema.to_ddl();
        assert!(
            ddl.contains("CREATE TABLE public.users"),
            "DDL did not contain expected table definition:\n{}",
            ddl
        );
    }
}
