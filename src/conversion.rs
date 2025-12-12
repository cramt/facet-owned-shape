use facet::ShapeLayout;

use crate::*;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ConversionError {
    UnsupportedType(String),
    NotAStruct(String),
    MissingTypeInfo,
    MultiplePrimaryKeys(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::UnsupportedType(msg) => write!(f, "Unsupported type: {}", msg),
            ConversionError::NotAStruct(msg) => write!(f, "Expected struct, got: {}", msg),
            ConversionError::MissingTypeInfo => write!(f, "Missing type information"),
            ConversionError::MultiplePrimaryKeys(msg) => {
                write!(f, "Multiple primary keys defined: {}", msg)
            }
        }
    }
}

impl Error for ConversionError {}

impl TryFrom<&facet::Shape> for PartialSchema {
    type Error = ConversionError;

    fn try_from(shape: &facet::Shape) -> Result<Self, Self::Error> {
        match shape.ty {
            facet::Type::User(facet::UserType::Struct(_)) => {
                let table = shape_to_table(shape)?;
                Ok(PartialSchema {
                    tables: vec![table],
                    views: Default::default(),
                    materialized_views: Default::default(),
                    enums: Default::default(),
                    domains: Default::default(),
                    composite_types: Default::default(),
                    sequences: Default::default(),
                    collations: Default::default(),
                    functions: Default::default(),
                })
            }
            facet::Type::User(facet::UserType::Enum(ref e)) => enum_to_partial_schema(shape, e),
            _ => Err(ConversionError::NotAStruct(format!("{:?}", shape.ty))),
        }
    }
}

fn shape_to_table(shape: &facet::Shape) -> Result<Table, ConversionError> {
    // Get the struct type definition
    let struct_type = match &shape.ty {
        facet::Type::User(facet::UserType::Struct(s)) => s,
        _ => return Err(ConversionError::NotAStruct(format!("{:?}", shape.ty))),
    };

    // Table name is the lowercase type identifier
    let table_name = shape.type_identifier.to_lowercase();

    // Process fields
    let (columns, primary_key) = process_fields(&struct_type.fields, &table_name)?;

    Ok(Table {
        name: table_name,
        columns,
        primary_key,
        uniques: vec![],
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
        comment: None,
        owned_sequences: vec![],
    })
}

fn field_to_column(field: &facet::Field) -> Result<Column, ConversionError> {
    // Call the shape function to get the field type
    let field_shape = field.shape();

    let (data_type, nullable) = shape_to_data_type(field_shape)?;

    Ok(Column {
        name: field.name.to_string(),
        data_type,
        default: None,
        nullable,
        collation: None,
        is_generated: false,
        generation_expression: None,
        is_identity: false,
        identity_generation: None,
        comment: None,
        privileges: None,
    })
}

fn process_fields(
    fields: &[facet::Field],
    table_name: &str,
) -> Result<(Vec<Column>, Option<PrimaryKey>), ConversionError> {
    let mut columns = Vec::new();
    let mut pk_columns = Vec::new();

    for field in fields.iter() {
        let column = field_to_column(field)?;
        columns.push(column);

        // Check for primary key attribute
        for attr in field.attributes {
            if attr.key == "primary_key" && attr.ns == Some("psql") {
                pk_columns.push(field.name.to_string());
            }
        }
    }

    if pk_columns.len() > 1 {
        return Err(ConversionError::MultiplePrimaryKeys(format!(
            "Table '{}' has {} primary keys: {:?}",
            table_name,
            pk_columns.len(),
            pk_columns
        )));
    }

    let primary_key = if !pk_columns.is_empty() {
        Some(PrimaryKey {
            name: None,
            columns: pk_columns,
            using: None,
            deferrable: None,
        })
    } else {
        None
    };

    Ok((columns, primary_key))
}

fn enum_to_partial_schema(
    shape: &facet::Shape,
    enum_type: &facet::EnumType,
) -> Result<PartialSchema, ConversionError> {
    let base_name = shape.type_identifier.to_lowercase();
    let mut tables = Vec::new();
    let mut foreign_keys = Vec::new();
    let mut main_columns = Vec::new();

    // 1. Create columns for the main table
    // Add primary key 'id'
    main_columns.push(Column {
        name: "id".to_string(),
        data_type: DataType::BigSerial, // Or BigInt if managed externally
        default: None,
        nullable: false,
        collation: None,
        is_generated: true,
        generation_expression: None,
        is_identity: true,
        identity_generation: Some(IdentityGeneration::Always),
        comment: None,
        privileges: None,
    });

    // Add dictionary/discriminant column
    main_columns.push(Column {
        name: "discriminant".to_string(),
        data_type: DataType::Integer,
        default: None,
        nullable: false,
        collation: None,
        is_generated: false,
        generation_expression: None,
        is_identity: false,
        identity_generation: None,
        comment: Some("Discriminant for enum variant".to_string()),
        privileges: None,
    });

    // 2. Process variants
    for (_, variant) in enum_type.variants.iter().enumerate() {
        let variant_name = variant.name.to_lowercase();
        let variant_table_name = format!("{}_{}", base_name, variant_name);

        // --- Variant Table ---
        match &variant.data.kind {
            facet::StructKind::Struct
            | facet::StructKind::Tuple
            | facet::StructKind::TupleStruct => {
                // Create a table for this variant
                // It needs an ID to be referenced
                let mut variant_columns = Vec::new();
                variant_columns.push(Column {
                    name: "id".to_string(),
                    data_type: DataType::BigSerial,
                    default: None,
                    nullable: false,
                    collation: None,
                    is_generated: true,
                    generation_expression: None,
                    is_identity: true,
                    identity_generation: Some(IdentityGeneration::Always),
                    comment: None,
                    privileges: None,
                });

                let (fields_cols, _) = process_fields(&variant.data.fields, &variant_table_name)?;
                variant_columns.extend(fields_cols);

                let variant_table = Table {
                    name: variant_table_name.clone(),
                    columns: variant_columns,
                    primary_key: Some(PrimaryKey {
                        name: None, // explicit name?
                        columns: vec!["id".to_string()],
                        using: None,
                        deferrable: None,
                    }),
                    uniques: vec![],
                    foreign_keys: vec![],
                    checks: vec![],
                    indexes: vec![],
                    options: empty_table_options(),
                    comment: None,
                    owned_sequences: vec![],
                };
                tables.push(variant_table);

                // --- Main Table Reference ---
                // Add FK column to main table
                let fk_col_name = format!("{}_id", variant_name);
                main_columns.push(Column {
                    name: fk_col_name.clone(),
                    data_type: DataType::BigInt,
                    default: None,
                    nullable: true, // Nullable because only one variant is active
                    collation: None,
                    is_generated: false,
                    generation_expression: None,
                    is_identity: false,
                    identity_generation: None,
                    comment: None,
                    privileges: None,
                });

                // Add Foreign Key constraint to main table
                foreign_keys.push(ForeignKey {
                    name: None,
                    columns: vec![fk_col_name.clone()],
                    referenced_table: QualifiedName {
                        schema: None,
                        name: variant_table_name,
                    },
                    referenced_columns: Some(vec!["id".to_string()]),
                    on_delete: Some(ReferentialAction::Cascade), // Deleting main row deletes variant row? Or vice versa? Usually cascade delete from parent to child.
                    on_update: Some(ReferentialAction::NoAction),
                    match_type: None,
                    deferrable: None,
                    initially: None,
                });
            }
            facet::StructKind::Unit => {
                // Unit variant - no extra data table needed?
                // Or just a marker?
                // User said "foreign keys to other new tables which represent either of those variants"
                // If it's unit, maybe no table needed, but we still need to track it.
                // For simplified logic matching request: "become 1 table ... with 3 fields (1 desc, 2 FKs)"
                // But if A is Unit, it has no fields.
                // Let's assume for now we still make a table for consistency, or strict optimization?
                // The request example had fields in A and B.
            }
        }
    }

    // Generate CHECK constraint
    // CHECK (
    //   (CASE WHEN discriminant = 0 THEN variant_0_id IS NOT NULL ELSE variant_0_id IS NULL END) AND
    //   (CASE WHEN discriminant = 1 THEN variant_1_id IS NOT NULL ELSE variant_1_id IS NULL END)
    // )
    // This ensures that IF discriminant is X, THEN id_X is set, AND (implicitly by logic) others should be null logic?
    // Actually, "ELSE variant_X_id IS NULL" ensures that if discriminant != X, then id_X MUST be null.
    // This is exactly what we want: rigid lockstep.

    let mut check_parts: Vec<String> = Vec::new();
    for (index, variant) in enum_type.variants.iter().enumerate() {
        let variant_name = variant.name.to_lowercase();
        match &variant.data.kind {
            facet::StructKind::Struct
            | facet::StructKind::Tuple
            | facet::StructKind::TupleStruct => {
                let col_name = format!("{}_id", variant_name);
                check_parts.push(format!(
                    "(CASE WHEN discriminant = {} THEN {} IS NOT NULL ELSE {} IS NULL END)",
                    index, col_name, col_name
                ));
            }
            // For Unit variants, we don't have an ID column, so we just ensure no other IDs are set?
            // But wait, if unit variant is active, then ALL ID columns must be null.
            // My loop above skips Unit variants for table creation, so there is no `unit_id` column.
            // But we need to verify that if discriminant points to Unit, then all existing ID columns are NULL.
            // AND the above loop only generates checks for existing columns.
            // So if discriminant = unit_index, then the above checks:
            // "CASE WHEN discriminant = struct_index ... ELSE struct_id IS NULL"
            // Since discriminant != struct_index, it enforces struct_id IS NULL.
            // So identifying unit variants implicitly works by enforcing all others to be null!
            // WE JUST NEED TO ENSURE `CASE WHEN` covers the "ELSE" branch correctly for all columns.
            _ => {}
        }
    }

    let check_expression = if check_parts.is_empty() {
        "1=1".to_string()
    } else {
        check_parts.join(" AND ")
    };

    let main_table = Table {
        name: base_name,
        columns: main_columns,
        primary_key: Some(PrimaryKey {
            name: None,
            columns: vec!["id".to_string()],
            using: None,
            deferrable: None,
        }),
        uniques: vec![],
        foreign_keys,
        checks: vec![CheckConstraint {
            name: Some("variant_integrity".to_string()),
            expression: check_expression,
            no_inherit: false,
        }],
        indexes: vec![],
        options: TableOptions {
            inherits: vec![],
            temporary: false,
            unlogged: false,
            partitioned: None,
            tablespace: None,
            with_storage_params: Default::default(),
        },
        comment: None,
        owned_sequences: vec![],
    };

    tables.push(main_table);

    Ok(PartialSchema {
        tables,
        views: vec![],
        materialized_views: vec![],
        enums: vec![],
        domains: vec![],
        composite_types: vec![],
        sequences: vec![],
        collations: vec![],
        functions: vec![],
    })
}

fn empty_table_options() -> TableOptions {
    TableOptions {
        inherits: vec![],
        temporary: false,
        unlogged: false,
        partitioned: None,
        tablespace: None,
        with_storage_params: Default::default(),
    }
}

fn shape_to_data_type(shape: &facet::Shape) -> Result<(DataType, bool), ConversionError> {
    // Check if this is an Option type (makes it nullable)
    if is_option_type(shape) {
        // Extract the inner type from Option
        if let Some(inner_shape) = get_option_inner_type(shape) {
            let (inner_type, _) = shape_to_data_type(inner_shape)?;
            return Ok((inner_type, true));
        }
    }

    // Map primitive types
    let data_type = match &shape.ty {
        facet::Type::Primitive(prim) => primitive_to_data_type(prim, shape)?,
        facet::Type::User(user_type) => user_type_to_data_type(user_type, shape)?,
        facet::Type::Pointer(_) => {
            // References like &str
            // Check if this is a string reference by looking at inner type if available
            if let Some(inner) = &shape.inner {
                // For references, use the inner type's type_identifier
                if inner.type_identifier.contains("str") {
                    return Ok((DataType::Text, false));
                }
            }
            // Fallback check on main type_identifier
            if shape.type_identifier.contains("str") {
                DataType::Text
            } else {
                return Err(ConversionError::UnsupportedType(format!(
                    "Pointer/reference type: {}",
                    shape.type_identifier
                )));
            }
        }
        _ => {
            return Err(ConversionError::UnsupportedType(format!(
                "{:?} (type_identifier: {})",
                shape.ty, shape.type_identifier
            )));
        }
    };

    Ok((data_type, false))
}

fn primitive_to_data_type(
    prim: &facet::PrimitiveType,
    shape: &facet::Shape,
) -> Result<DataType, ConversionError> {
    Ok(match prim {
        facet::PrimitiveType::Boolean => DataType::Boolean,

        facet::PrimitiveType::Numeric(numeric) => {
            match numeric {
                facet::NumericType::Integer { signed: _ } => {
                    // Determine size from shape layout
                    let size = match &shape.layout {
                        ShapeLayout::Sized(layout) => layout.size(),
                        _ => {
                            return Err(ConversionError::UnsupportedType(
                                "unsized integer".to_string(),
                            ));
                        }
                    };

                    // Map based on size
                    match size {
                        1 => DataType::SmallInt,    // i8, u8
                        2 => DataType::SmallInt,    // i16, u16
                        4 => DataType::Integer,     // i32, u32
                        8 | 16 => DataType::BigInt, // i64, u64, i128, u128, isize, usize
                        _ => DataType::BigInt,
                    }
                }
                facet::NumericType::Float => {
                    // Determine size from shape layout
                    let size = match &shape.layout {
                        ShapeLayout::Sized(layout) => layout.size(),
                        _ => {
                            return Err(ConversionError::UnsupportedType(
                                "unsized float".to_string(),
                            ));
                        }
                    };

                    match size {
                        4 => DataType::Real,            // f32
                        8 => DataType::DoublePrecision, // f64
                        _ => {
                            return Err(ConversionError::UnsupportedType(format!(
                                "float with size {}",
                                size
                            )));
                        }
                    }
                }
            }
        }

        facet::PrimitiveType::Textual(textual) => match textual {
            facet::TextualType::Char => DataType::Char(Some(1)),
            facet::TextualType::Str => DataType::Text,
        },

        _ => return Err(ConversionError::UnsupportedType(format!("{:?}", prim))),
    })
}

fn user_type_to_data_type(
    user_type: &facet::UserType,
    shape: &facet::Shape,
) -> Result<DataType, ConversionError> {
    // First check: Is this String type based on type_identifier?
    // String can be represented in different ways (Struct, Opaque, etc)
    if shape.type_identifier == "String"
        || shape.type_identifier.ends_with("::String")
        || shape.type_identifier.contains("alloc::string::String")
    {
        return Ok(DataType::Text);
    }

    // Check for Vec - represented as Opaque
    if shape.type_identifier.contains("Vec") || shape.type_identifier.contains("::vec::Vec") {
        // For now, treat Vec as JSONB (could be Array in future)
        return Ok(DataType::Jsonb);
    }

    // Check for HashMap - represented as Opaque
    if shape.type_identifier.contains("HashMap") {
        return Ok(DataType::Jsonb);
    }

    match user_type {
        facet::UserType::Struct(_) => {
            // For now, treat nested structs as JSONB
            // In the future, we could create composite types
            Ok(DataType::Jsonb)
        }
        facet::UserType::Enum(_) => {
            // For now, treat enums as integers
            // In the future, we could create PostgreSQL ENUM types
            Ok(DataType::Integer)
        }
        _ => {
            // Final check: if this is still String-related, return Text
            Err(ConversionError::UnsupportedType(format!(
                "{:?} (type_identifier: {})",
                user_type, shape.type_identifier
            )))
        }
    }
}

fn is_option_type(shape: &facet::Shape) -> bool {
    // Check if the type identifier contains "Option"
    shape.type_identifier.contains("Option")
}

fn get_option_inner_type(shape: &facet::Shape) -> Option<&facet::Shape> {
    // For Option<T>, check if we have type parameters
    // Option should have 1 type parameter
    if !shape.type_params.is_empty() {
        // type_params[0] should be the T in Option<T>
        if let Some(first_param) = shape.type_params.first() {
            // The TypeParam contains a shape
            return Some(first_param.shape);
        }
    }

    // Fallback: try to extract from enum variant (old approach)
    if let facet::Type::User(facet::UserType::Enum(enum_type)) = &shape.ty {
        // Option is an enum with Some(T) and None variants
        // Get the first variant (Some) and extract its shape
        if let Some(variant) = enum_type.variants.first() {
            if let facet::StructKind::Tuple = variant.data.kind {
                if !variant.data.fields.is_empty() {
                    let field_shape = variant.data.fields[0].shape();
                    return Some(field_shape);
                }
            }
        }
    }

    None
}
