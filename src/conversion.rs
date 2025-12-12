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

impl TryFrom<&facet::Shape> for Table {
    type Error = ConversionError;

    fn try_from(shape: &facet::Shape) -> Result<Self, Self::Error> {
        // Get the struct type definition
        let struct_type = match &shape.ty {
            facet::Type::User(facet::UserType::Struct(s)) => s,
            _ => return Err(ConversionError::NotAStruct(format!("{:?}", shape.ty))),
        };

        // Table name is the lowercase type identifier
        let table_name = shape.type_identifier.to_lowercase();

        // Convert each field to a column
        let mut columns = Vec::new();
        let mut pk_columns = Vec::new();

        for field in struct_type.fields.iter() {
            let column = field_to_column(field)?;
            columns.push(column);

            // Check for primary key attribute
            for attr in field.attributes {
                // Check if attribute is "psql::primary_key"
                // attributes usually have ns and key
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
