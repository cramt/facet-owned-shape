use std::collections::{HashMap, HashSet};

use crate::owned_shape::{OwnedDef, OwnedShape, OwnedType, OwnedUserType};

/// The difference between two shape definitions.
///
/// This compares the structure and metadata of shapes, not runtime values.
#[derive(Debug, Clone)]
pub enum Diff {
    /// The two shapes are structurally equal
    Equal,

    /// The shapes are different
    Different {
        /// The `from` shape
        from: OwnedShape,
        /// The `to` shape
        to: OwnedShape,
    },

    /// The two shapes are both structures or both enums with similar structure
    User {
        /// The `from` shape
        from: OwnedShape,
        /// The `to` shape
        to: OwnedShape,
        /// Field-level differences for structs
        value: Value,
    },

    /// A diff between two sequence-like shapes
    Sequence {
        /// The `from` shape
        from: OwnedShape,
        /// The `to` shape
        to: OwnedShape,
    },
}

/// Field-level differences for structs
#[derive(Debug, Clone)]
pub enum Value {
    Struct {
        /// Fields that exist in both but have different shapes
        updates: HashMap<String, Diff>,
        /// Fields that are in `from` but not in `to`
        deletions: HashSet<String>,
        /// Fields that are in `to` but not in `from`
        insertions: HashSet<String>,
        /// Fields that are unchanged
        unchanged: HashSet<String>,
    },
}

impl Diff {
    /// Returns true if the two shapes are equal
    pub fn is_equal(&self) -> bool {
        matches!(self, Self::Equal)
    }

    /// Computes the difference between two owned shapes
    pub fn new(from: &OwnedShape, to: &OwnedShape) -> Self {
        // Quick equality check
        if shapes_equal(from, to) {
            return Diff::Equal;
        }

        // Compare based on type
        match (&*from.ty, &*to.ty) {
            (
                OwnedType::User(OwnedUserType::Struct(from_struct)),
                OwnedType::User(OwnedUserType::Struct(to_struct)),
            ) => {
                let mut updates = HashMap::new();
                let mut deletions = HashSet::new();
                let mut insertions = HashSet::new();
                let mut unchanged = HashSet::new();

                // Build a map of field names to fields for quick lookup
                let to_fields: HashMap<_, _> = to_struct
                    .fields
                    .iter()
                    .map(|f| (f.name.as_str(), f))
                    .collect();

                // Compare fields from 'from' struct
                for from_field in &from_struct.fields {
                    if let Some(to_field) = to_fields.get(from_field.name.as_str()) {
                        let field_diff = Diff::new(&from_field.shape, &to_field.shape);
                        if field_diff.is_equal() {
                            unchanged.insert(from_field.name.clone());
                        } else {
                            updates.insert(from_field.name.clone(), field_diff);
                        }
                    } else {
                        deletions.insert(from_field.name.clone());
                    }
                }

                // Find insertions (fields in 'to' but not in 'from')
                let from_field_names: HashSet<_> =
                    from_struct.fields.iter().map(|f| f.name.as_str()).collect();

                for to_field in &to_struct.fields {
                    if !from_field_names.contains(to_field.name.as_str()) {
                        insertions.insert(to_field.name.clone());
                    }
                }

                Diff::User {
                    from: from.clone(),
                    to: to.clone(),
                    value: Value::Struct {
                        updates,
                        deletions,
                        insertions,
                        unchanged,
                    },
                }
            }
            (OwnedType::User(OwnedUserType::Enum(_)), OwnedType::User(OwnedUserType::Enum(_))) => {
                // For enums, we could compare variants but for now just mark as different or equal
                Diff::Different {
                    from: from.clone(),
                    to: to.clone(),
                }
            }
            (OwnedType::Sequence(_), OwnedType::Sequence(_)) => Diff::Sequence {
                from: from.clone(),
                to: to.clone(),
            },
            _ => Diff::Different {
                from: from.clone(),
                to: to.clone(),
            },
        }
    }
}

/// Helper function to check if two shapes are structurally equal
fn shapes_equal(a: &OwnedShape, b: &OwnedShape) -> bool {
    // Compare type identifiers
    if a.type_identifier != b.type_identifier {
        return false;
    }

    // Compare definitions
    if !defs_equal(&a.def, &b.def) {
        return false;
    }

    // Compare types
    types_equal(&a.ty, &b.ty)
}

fn defs_equal(a: &OwnedDef, b: &OwnedDef) -> bool {
    match (a, b) {
        (OwnedDef::Undefined, OwnedDef::Undefined) => true,
        (OwnedDef::Scalar, OwnedDef::Scalar) => true,
        (OwnedDef::Map(a), OwnedDef::Map(b)) => {
            shapes_equal(&a.k, &b.k) && shapes_equal(&a.v, &b.v)
        }
        (OwnedDef::Set(a), OwnedDef::Set(b)) => shapes_equal(&a.t, &b.t),
        (OwnedDef::List(a), OwnedDef::List(b)) => shapes_equal(&a.t, &b.t),
        (OwnedDef::Array(a), OwnedDef::Array(b)) => a.n == b.n && shapes_equal(&a.t, &b.t),
        (OwnedDef::Option(a), OwnedDef::Option(b)) => shapes_equal(&a.t, &b.t),
        _ => false,
    }
}

fn types_equal(a: &OwnedType, b: &OwnedType) -> bool {
    match (a, b) {
        (OwnedType::Primitive(a), OwnedType::Primitive(b)) => {
            // Using Debug format for simple comparison
            format!("{:?}", a) == format!("{:?}", b)
        }
        (OwnedType::Sequence(a), OwnedType::Sequence(b)) => shapes_equal(&a.t, &b.t),
        (OwnedType::User(a), OwnedType::User(b)) => match (a, b) {
            (OwnedUserType::Struct(a), OwnedUserType::Struct(b)) => {
                if a.fields.len() != b.fields.len() {
                    return false;
                }
                a.fields
                    .iter()
                    .zip(b.fields.iter())
                    .all(|(af, bf)| af.name == bf.name && shapes_equal(&af.shape, &bf.shape))
            }
            (OwnedUserType::Enum(a), OwnedUserType::Enum(b)) => {
                if a.variants.len() != b.variants.len() {
                    return false;
                }
                a.variants.iter().zip(b.variants.iter()).all(|(av, bv)| {
                    av.name == bv.name
                        && av.data.fields.len() == bv.data.fields.len()
                        && av
                            .data
                            .fields
                            .iter()
                            .zip(bv.data.fields.iter())
                            .all(|(af, bf)| {
                                af.name == bf.name && shapes_equal(&af.shape, &bf.shape)
                            })
                })
            }
            (OwnedUserType::Union(a), OwnedUserType::Union(b)) => {
                if a.fields.len() != b.fields.len() {
                    return false;
                }
                a.fields
                    .iter()
                    .zip(b.fields.iter())
                    .all(|(af, bf)| af.name == bf.name && shapes_equal(&af.shape, &bf.shape))
            }
            (OwnedUserType::Opaque, OwnedUserType::Opaque) => true,
            _ => false,
        },
        _ => false,
    }
}
