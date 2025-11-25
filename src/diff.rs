use std::collections::{HashMap, HashSet};

use crate::cow_shape::{
    CowDef, CowEnumType, CowShape, CowStructType, CowType, CowUnionType, CowUserType,
};

/// The difference between two shape definitions.
///
/// This compares the structure and metadata of shapes, not runtime values.
#[derive(Debug, Clone)]
pub enum Diff<'a> {
    /// The two shapes are structurally equal
    Equal,

    /// The shapes are different
    Different {
        /// The `from` shape
        from: CowShape<'a>,
        /// The `to` shape
        to: CowShape<'a>,
    },

    /// The two shapes are both structures or both enums with similar structure
    User {
        /// The `from` shape
        from: CowShape<'a>,
        /// The `to` shape
        to: CowShape<'a>,
        /// Field-level differences for structs
        value: Value<'a>,
    },

    /// A diff between two sequence-like shapes
    Sequence {
        /// The `from` shape
        from: CowShape<'a>,
        /// The `to` shape
        to: CowShape<'a>,
    },
}

/// Field-level differences for structs
#[derive(Debug, Clone)]
pub enum Value<'a> {
    Struct {
        /// Fields that exist in both but have different shapes
        updates: HashMap<String, Diff<'a>>,
        /// Fields that are in `from` but not in `to`
        deletions: HashSet<String>,
        /// Fields that are in `to` but not in `from`
        insertions: HashSet<String>,
        /// Fields that are unchanged
        unchanged: HashSet<String>,
    },
}

impl<'a> Diff<'a> {
    /// Returns true if the two shapes are equal
    pub fn is_equal(&self) -> bool {
        matches!(self, Self::Equal)
    }

    /// Computes the difference between two owned shapes
    pub fn new(from: &CowShape<'a>, to: &CowShape<'a>) -> Self {
        // Quick equality check
        if shapes_equal(from, to) {
            return Diff::Equal;
        }

        // Compare based on type
        match (from.ty.as_ref(), to.ty.as_ref()) {
            (
                CowType::User(CowUserType::Struct(from_struct)),
                CowType::User(CowUserType::Struct(to_struct)),
            ) => {
                let mut updates = HashMap::new();
                let mut deletions = HashSet::new();
                let mut insertions = HashSet::new();
                let mut unchanged = HashSet::new();

                // Build a map of field names to fields for quick lookup
                let to_fields: HashMap<_, _> = to_struct
                    .fields
                    .iter()
                    .map(|f| (f.name.as_ref(), f))
                    .collect();

                // Compare fields from 'from' struct
                for from_field in &from_struct.fields {
                    if let Some(to_field) = to_fields.get(from_field.name.as_ref()) {
                        let field_diff = Diff::new(&from_field.shape, &to_field.shape);
                        if field_diff.is_equal() {
                            unchanged.insert(from_field.name.to_string());
                        } else {
                            updates.insert(from_field.name.to_string(), field_diff);
                        }
                    } else {
                        deletions.insert(from_field.name.to_string());
                    }
                }

                // Find insertions (fields in 'to' but not in 'from')
                let from_field_names: HashSet<_> =
                    from_struct.fields.iter().map(|f| f.name.as_ref()).collect();

                for to_field in &to_struct.fields {
                    if !from_field_names.contains(to_field.name.as_ref()) {
                        insertions.insert(to_field.name.to_string());
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
            (CowType::User(CowUserType::Enum(_)), CowType::User(CowUserType::Enum(_))) => {
                // For enums, we could compare variants but for now just mark as different or equal
                Diff::Different {
                    from: from.clone(),
                    to: to.clone(),
                }
            }
            (CowType::Sequence(_), CowType::Sequence(_)) => Diff::Sequence {
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
fn shapes_equal(a: &CowShape, b: &CowShape) -> bool {
    // Compare type identifiers
    if a.type_identifier != b.type_identifier {
        return false;
    }

    // Compare definitions
    if !defs_equal(a.def.as_ref(), b.def.as_ref()) {
        return false;
    }

    // Compare types
    types_equal(a.ty.as_ref(), b.ty.as_ref())
}

fn defs_equal(a: &CowDef, b: &CowDef) -> bool {
    match (a, b) {
        (CowDef::Undefined, CowDef::Undefined) => true,
        (CowDef::Scalar, CowDef::Scalar) => true,
        (CowDef::Map(a), CowDef::Map(b)) => shapes_equal(&a.k, &b.k) && shapes_equal(&a.v, &b.v),
        (CowDef::Set(a), CowDef::Set(b)) => shapes_equal(&a.t, &b.t),
        (CowDef::List(a), CowDef::List(b)) => shapes_equal(&a.t, &b.t),
        (CowDef::Array(a), CowDef::Array(b)) => a.n == b.n && shapes_equal(&a.t, &b.t),
        (CowDef::Option(a), CowDef::Option(b)) => shapes_equal(&a.t, &b.t),
        _ => false,
    }
}

fn types_equal(a: &CowType, b: &CowType) -> bool {
    match (a, b) {
        (CowType::Primitive(a), CowType::Primitive(b)) => {
            // Using Debug format for simple comparison
            format!("{:?}", a) == format!("{:?}", b)
        }
        (CowType::Sequence(a), CowType::Sequence(b)) => shapes_equal(&a.t, &b.t),
        (CowType::User(a), CowType::User(b)) => match (a, b) {
            (CowUserType::Struct(a), CowUserType::Struct(b)) => structs_equal(a, b),
            (CowUserType::Enum(a), CowUserType::Enum(b)) => enums_equal(a, b),
            (CowUserType::Union(a), CowUserType::Union(b)) => unions_equal(a, b),
            (CowUserType::Opaque, CowUserType::Opaque) => true,
            _ => false,
        },
        _ => false,
    }
}

fn structs_equal(a: &CowStructType, b: &CowStructType) -> bool {
    if a.fields.len() != b.fields.len() {
        return false;
    }
    a.fields
        .iter()
        .zip(b.fields.iter())
        .all(|(af, bf)| af.name == bf.name && shapes_equal(&af.shape, &bf.shape))
}

fn enums_equal(a: &CowEnumType, b: &CowEnumType) -> bool {
    if a.variants.len() != b.variants.len() {
        return false;
    }
    a.variants
        .iter()
        .zip(b.variants.iter())
        .all(|(av, bv)| av.name == bv.name && structs_equal(&av.data, &bv.data))
}

fn unions_equal(a: &CowUnionType, b: &CowUnionType) -> bool {
    if a.fields.len() != b.fields.len() {
        return false;
    }
    a.fields
        .iter()
        .zip(b.fields.iter())
        .all(|(af, bf)| af.name == bf.name && shapes_equal(&af.shape, &bf.shape))
}
