use crate::{
    diff::{Diff, Value},
    owned_shape::{
        OwnedDef, OwnedNumericType, OwnedPrimitiveType, OwnedShape, OwnedTextualType, OwnedType,
        OwnedUserType,
    },
};
use sea_query::{ColumnDef, Table, TableAlterStatement, TableCreateStatement};

impl TryFrom<OwnedShape> for TableCreateStatement {
    type Error = String;

    fn try_from(shape: OwnedShape) -> Result<Self, Self::Error> {
        match *shape.ty {
            OwnedType::User(OwnedUserType::Struct(s)) => {
                let mut table = Table::create();
                table.table(sea_query::Alias::new(&shape.type_identifier));

                for field in s.fields {
                    let mut col = ColumnDef::new(sea_query::Alias::new(&field.name));

                    let is_nullable = matches!(*field.shape.def, OwnedDef::Option(_));
                    if is_nullable {
                        col.null();
                    } else {
                        col.not_null();
                    }

                    set_column_type_from_shape(&mut col, &field.shape)?;

                    table.col(&mut col);
                }

                Ok(table)
            }
            _ => Err(format!(
                "Only Struct shapes can be converted to TableCreateStatement. Found: {:?}",
                shape.ty
            )),
        }
    }
}

impl TryFrom<Diff> for TableAlterStatement {
    type Error = String;

    fn try_from(diff: Diff) -> Result<Self, Self::Error> {
        match diff {
            Diff::Equal => {
                Err("Cannot create ALTER TABLE from Equal diff - no changes needed".to_string())
            }
            Diff::Different { .. } => Err(
                "Cannot create ALTER TABLE from Different diff - shapes are incompatible"
                    .to_string(),
            ),
            Diff::Sequence { .. } => Err(
                "Cannot create ALTER TABLE from Sequence diff - only struct diffs are supported"
                    .to_string(),
            ),
            Diff::User { from: _, to, value } => match value {
                Value::Struct {
                    updates,
                    deletions,
                    insertions,
                    unchanged: _,
                } => {
                    let mut alter = Table::alter();
                    alter.table(sea_query::Alias::new(&to.type_identifier));

                    let to_struct = match *to.ty {
                        OwnedType::User(OwnedUserType::Struct(s)) => s,
                        _ => return Err("Expected 'to' shape to be a struct".to_string()),
                    };

                    for field_name in &insertions {
                        let field = to_struct
                            .fields
                            .iter()
                            .find(|f| &f.name == field_name)
                            .ok_or_else(|| {
                                format!("Field '{}' not found in 'to' struct", field_name)
                            })?;

                        let mut col = ColumnDef::new(sea_query::Alias::new(&field.name));

                        let is_nullable = matches!(*field.shape.def, OwnedDef::Option(_));
                        if is_nullable {
                            col.null();
                        } else {
                            col.not_null();
                        }

                        set_column_type_from_shape(&mut col, &field.shape)?;

                        alter.add_column(&mut col);
                    }

                    for (field_name, field_diff) in &updates {
                        let to_field = to_struct
                            .fields
                            .iter()
                            .find(|f| &f.name == field_name)
                            .ok_or_else(|| {
                                format!("Field '{}' not found in 'to' struct", field_name)
                            })?;

                        if !is_compatible_type_change(field_diff)? {
                            return Err(format!(
                                "Incompatible type change for field '{}'. Only conversions between numbers and strings are supported",
                                field_name
                            ));
                        }

                        let mut col = ColumnDef::new(sea_query::Alias::new(&to_field.name));

                        let is_nullable = matches!(*to_field.shape.def, OwnedDef::Option(_));
                        if is_nullable {
                            col.null();
                        } else {
                            col.not_null();
                        }

                        set_column_type_from_shape(&mut col, &to_field.shape)?;

                        alter.modify_column(&mut col);
                    }

                    for field_name in &deletions {
                        alter.drop_column(sea_query::Alias::new(field_name));
                    }

                    if insertions.is_empty() && deletions.is_empty() && updates.is_empty() {
                        return Err("No column changes found".to_string());
                    }

                    Ok(alter)
                }
            },
        }
    }
}

fn is_compatible_type_change(diff: &Diff) -> Result<bool, String> {
    match diff {
        Diff::Different { from, to } => {
            let from_inner = unwrap_option_type(from);
            let to_inner = unwrap_option_type(to);

            match (&*from_inner.ty, &*to_inner.ty) {
                (OwnedType::Primitive(from_p), OwnedType::Primitive(to_p)) => {
                    match (from_p, to_p) {
                        (OwnedPrimitiveType::Numeric(_), OwnedPrimitiveType::Numeric(_)) => {
                            Ok(true)
                        }

                        (OwnedPrimitiveType::Numeric(_), OwnedPrimitiveType::Textual(_)) => {
                            Ok(true)
                        }

                        (OwnedPrimitiveType::Textual(_), OwnedPrimitiveType::Numeric(_)) => {
                            Ok(true)
                        }

                        (OwnedPrimitiveType::Textual(_), OwnedPrimitiveType::Textual(_)) => {
                            Ok(true)
                        }

                        _ => Ok(false),
                    }
                }

                (
                    OwnedType::User(OwnedUserType::Opaque),
                    OwnedType::Primitive(OwnedPrimitiveType::Numeric(_)),
                )
                | (
                    OwnedType::Primitive(OwnedPrimitiveType::Numeric(_)),
                    OwnedType::User(OwnedUserType::Opaque),
                ) => {
                    let opaque_shape =
                        if matches!(*from_inner.ty, OwnedType::User(OwnedUserType::Opaque)) {
                            from_inner
                        } else {
                            to_inner
                        };

                    if opaque_shape.type_identifier == "String" {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
                _ => Ok(false),
            }
        }
        _ => Ok(true),
    }
}

/// Unwrap Option types to get to the inner type
fn unwrap_option_type(shape: &OwnedShape) -> &OwnedShape {
    if let OwnedDef::Option(opt) = &*shape.def {
        unwrap_option_type(&opt.t)
    } else {
        shape
    }
}

fn set_column_type_from_shape(col: &mut ColumnDef, shape: &OwnedShape) -> Result<(), String> {
    let inner_shape = if let OwnedDef::Option(opt) = &*shape.def {
        &opt.t
    } else {
        shape
    };

    match &*inner_shape.ty {
        OwnedType::Primitive(p) => match p {
            OwnedPrimitiveType::Boolean => {
                col.boolean();
            }
            OwnedPrimitiveType::Numeric(n) => match n {
                OwnedNumericType::Integer { .. } => match inner_shape.type_identifier.as_str() {
                    "u8" | "i8" => {
                        col.tiny_integer();
                    }
                    "u16" | "i16" => {
                        col.small_integer();
                    }
                    "u32" | "i32" => {
                        col.integer();
                    }
                    "u64" | "i64" | "usize" | "isize" => {
                        col.big_integer();
                    }
                    _ => {
                        col.integer();
                    }
                },
                OwnedNumericType::Float => match inner_shape.type_identifier.as_str() {
                    "f32" => {
                        col.float();
                    }
                    "f64" => {
                        col.double();
                    }
                    _ => {
                        col.double();
                    }
                },
            },
            OwnedPrimitiveType::Textual(t) => match t {
                OwnedTextualType::Char => {
                    col.char_len(1);
                }
                OwnedTextualType::Str => {
                    col.string();
                }
            },
            OwnedPrimitiveType::Never => {
                return Err("Never type not supported in SQL".to_string());
            }
        },
        OwnedType::User(OwnedUserType::Enum(_)) => {
            col.string();
        }
        OwnedType::User(OwnedUserType::Opaque) => match inner_shape.type_identifier.as_str() {
            "String" | "str" => {
                col.string();
            }
            _ => {
                return Err(format!(
                    "Unsupported Opaque type for SQL column: {}",
                    inner_shape.type_identifier
                ));
            }
        },
        _ => {
            return Err(format!(
                "Unsupported type for SQL column: {:?} (ID: {})",
                inner_shape.ty, inner_shape.type_identifier
            ));
        }
    }

    Ok(())
}
