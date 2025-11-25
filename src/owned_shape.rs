use facet::Facet;

#[derive(Facet, Clone, Debug)]
pub struct OwnedMapDef {
    pub k: OwnedShape,
    pub v: OwnedShape,
}

#[derive(Facet, Clone, Debug)]
pub struct OwnedSetDef {
    pub t: OwnedShape,
}

#[derive(Facet, Clone, Debug)]
pub struct OwnedListDef {
    pub t: OwnedShape,
}

#[derive(Facet, Clone, Debug)]
pub struct OwnedArrayDef {
    pub t: OwnedShape,
    pub n: usize,
}

#[derive(Facet, Clone, Debug)]
pub struct OwnedOptionDef {
    pub t: OwnedShape,
}

#[derive(Facet, Clone, Debug)]
#[repr(C)]
pub enum OwnedDef {
    Undefined,
    Scalar,
    Map(OwnedMapDef),
    Set(OwnedSetDef),
    List(OwnedListDef),
    Array(OwnedArrayDef),
    Option(OwnedOptionDef),
}

#[derive(Facet, Clone, Debug)]
#[repr(C)]
pub enum OwnedNumericType {
    Integer { signed: bool },
    Float,
}

#[derive(Facet, Clone, Debug)]
#[repr(C)]
pub enum OwnedTextualType {
    Char = 0,
    Str = 1,
}

#[derive(Facet, Clone, Debug)]
#[repr(C)]
pub enum OwnedPrimitiveType {
    Boolean,
    Numeric(OwnedNumericType),
    Textual(OwnedTextualType),
    Never,
}

#[derive(Facet, Clone, Debug)]
pub struct OwnedSequenceType {
    pub t: OwnedShape,
}

#[derive(Facet, Clone, Debug)]
pub struct OwnedField {
    pub name: String,
    pub shape: OwnedShape,
    pub doc: Vec<String>,
}

#[derive(Facet, Clone, Debug)]
pub struct OwnedStructType {
    pub fields: Vec<OwnedField>,
}

#[derive(Facet, Clone, Debug)]
pub struct OwnedUnionType {
    pub fields: Vec<OwnedField>,
}

#[derive(Facet, Clone, Debug)]
pub struct OwnedVariant {
    pub name: String,
    pub data: OwnedStructType,
    pub doc: Vec<String>,
}

#[derive(Facet, Clone, Debug)]
pub struct OwnedEnumType {
    pub variants: Vec<OwnedVariant>,
}

#[derive(Facet, Clone, Debug)]
#[repr(C)]
pub enum OwnedUserType {
    Struct(OwnedStructType),
    Enum(OwnedEnumType),
    Union(OwnedUnionType),
    Opaque,
}

#[derive(Facet, Clone, Debug)]
#[repr(C)]
pub enum OwnedType {
    Primitive(OwnedPrimitiveType),
    Sequence(OwnedSequenceType),
    User(OwnedUserType),
}

#[derive(Facet, Clone, Debug)]
pub struct OwnedShape {
    pub type_identifier: String,
    pub def: Box<OwnedDef>,
    pub ty: Box<OwnedType>,
}

use crate::box_cow::BoxCow;
use crate::cow_shape::*;

impl<'a> From<CowShape<'a>> for OwnedShape {
    fn from(shape: CowShape<'a>) -> Self {
        OwnedShape {
            type_identifier: shape.type_identifier.into_owned(),
            def: Box::new(match shape.def {
                BoxCow::Borrowed(b) => b.clone().into(),
                BoxCow::Owned(o) => (*o).into(),
            }),
            ty: Box::new(match shape.ty {
                BoxCow::Borrowed(b) => b.clone().into(),
                BoxCow::Owned(o) => (*o).into(),
            }),
        }
    }
}

impl<'a> From<CowDef<'a>> for OwnedDef {
    fn from(def: CowDef<'a>) -> Self {
        match def {
            CowDef::Undefined => OwnedDef::Undefined,
            CowDef::Scalar => OwnedDef::Scalar,
            CowDef::Map(d) => OwnedDef::Map(OwnedMapDef {
                k: d.k.into(),
                v: d.v.into(),
            }),
            CowDef::Set(d) => OwnedDef::Set(OwnedSetDef { t: d.t.into() }),
            CowDef::List(d) => OwnedDef::List(OwnedListDef { t: d.t.into() }),
            CowDef::Array(d) => OwnedDef::Array(OwnedArrayDef {
                t: d.t.into(),
                n: d.n,
            }),
            CowDef::Option(d) => OwnedDef::Option(OwnedOptionDef { t: d.t.into() }),
        }
    }
}

impl<'a> From<CowType<'a>> for OwnedType {
    fn from(ty: CowType<'a>) -> Self {
        match ty {
            CowType::Primitive(p) => OwnedType::Primitive(p.into()),
            CowType::Sequence(s) => OwnedType::Sequence(OwnedSequenceType { t: s.t.into() }),
            CowType::User(u) => OwnedType::User(u.into()),
        }
    }
}

impl From<CowPrimitiveType> for OwnedPrimitiveType {
    fn from(p: CowPrimitiveType) -> Self {
        match p {
            CowPrimitiveType::Boolean => OwnedPrimitiveType::Boolean,
            CowPrimitiveType::Numeric(n) => OwnedPrimitiveType::Numeric(n.into()),
            CowPrimitiveType::Textual(t) => OwnedPrimitiveType::Textual(t.into()),
            CowPrimitiveType::Never => OwnedPrimitiveType::Never,
        }
    }
}

impl From<CowNumericType> for OwnedNumericType {
    fn from(n: CowNumericType) -> Self {
        match n {
            CowNumericType::Integer { signed } => OwnedNumericType::Integer { signed },
            CowNumericType::Float => OwnedNumericType::Float,
        }
    }
}

impl From<CowTextualType> for OwnedTextualType {
    fn from(t: CowTextualType) -> Self {
        match t {
            CowTextualType::Char => OwnedTextualType::Char,
            CowTextualType::Str => OwnedTextualType::Str,
        }
    }
}

impl<'a> From<CowUserType<'a>> for OwnedUserType {
    fn from(u: CowUserType<'a>) -> Self {
        match u {
            CowUserType::Struct(s) => OwnedUserType::Struct(s.into()),
            CowUserType::Enum(e) => OwnedUserType::Enum(e.into()),
            CowUserType::Union(u) => OwnedUserType::Union(u.into()),
            CowUserType::Opaque => OwnedUserType::Opaque,
        }
    }
}

impl<'a> From<CowStructType<'a>> for OwnedStructType {
    fn from(s: CowStructType<'a>) -> Self {
        OwnedStructType {
            fields: s.fields.into_iter().map(Into::into).collect(),
        }
    }
}

impl<'a> From<CowField<'a>> for OwnedField {
    fn from(f: CowField<'a>) -> Self {
        OwnedField {
            name: f.name.into_owned(),
            shape: f.shape.into(),
            doc: f.doc.into_iter().map(|s| s.into_owned()).collect(),
        }
    }
}

impl<'a> From<CowEnumType<'a>> for OwnedEnumType {
    fn from(e: CowEnumType<'a>) -> Self {
        OwnedEnumType {
            variants: e.variants.into_iter().map(Into::into).collect(),
        }
    }
}

impl<'a> From<CowVariant<'a>> for OwnedVariant {
    fn from(v: CowVariant<'a>) -> Self {
        OwnedVariant {
            name: v.name.into_owned(),
            data: v.data.into(),
            doc: v.doc.into_iter().map(|s| s.into_owned()).collect(),
        }
    }
}

impl<'a> From<CowUnionType<'a>> for OwnedUnionType {
    fn from(u: CowUnionType<'a>) -> Self {
        OwnedUnionType {
            fields: u.fields.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<&facet::Shape> for OwnedShape {
    type Error = String;

    fn try_from(shape: &facet::Shape) -> Result<Self, Self::Error> {
        Ok(OwnedShape {
            type_identifier: shape.type_identifier.to_string(),
            def: Box::new((&shape.def).try_into()?),
            ty: Box::new((&shape.ty).try_into()?),
        })
    }
}

impl TryFrom<&facet::Def> for OwnedDef {
    type Error = String;

    fn try_from(def: &facet::Def) -> Result<Self, Self::Error> {
        match def {
            facet::Def::Undefined => Ok(OwnedDef::Undefined),
            facet::Def::Scalar => Ok(OwnedDef::Scalar),
            facet::Def::Map(map_def) => Ok(OwnedDef::Map(OwnedMapDef {
                k: map_def.k().try_into()?,
                v: map_def.v().try_into()?,
            })),
            facet::Def::Set(set_def) => Ok(OwnedDef::Set(OwnedSetDef {
                t: set_def.t().try_into()?,
            })),
            facet::Def::List(list_def) => Ok(OwnedDef::List(OwnedListDef {
                t: list_def.t().try_into()?,
            })),
            facet::Def::Slice(slice_def) => Ok(OwnedDef::List(OwnedListDef {
                t: slice_def.t().try_into()?,
            })),
            facet::Def::Array(array_def) => Ok(OwnedDef::Array(OwnedArrayDef {
                t: array_def.t().try_into()?,
                n: array_def.n,
            })),
            facet::Def::Option(option_def) => Ok(OwnedDef::Option(OwnedOptionDef {
                t: option_def.t().try_into()?,
            })),
            _ => Err("Unsupported Def variant".to_string()),
        }
    }
}

impl TryFrom<&facet::Type> for OwnedType {
    type Error = String;

    fn try_from(ty: &facet::Type) -> Result<Self, Self::Error> {
        match ty {
            facet::Type::Primitive(p) => Ok(OwnedType::Primitive(p.try_into()?)),
            facet::Type::Sequence(s) => Ok(OwnedType::Sequence(OwnedSequenceType {
                t: match s {
                    facet::SequenceType::Array(array_type) => array_type.t.try_into()?,
                    facet::SequenceType::Slice(slice_type) => slice_type.t.try_into()?,
                },
            })),
            facet::Type::User(u) => Ok(OwnedType::User(u.try_into()?)),
            facet::Type::Pointer(_) => Err("Pointer types not supported".to_string()),
        }
    }
}

impl TryFrom<&facet::PrimitiveType> for OwnedPrimitiveType {
    type Error = String;

    fn try_from(p: &facet::PrimitiveType) -> Result<Self, Self::Error> {
        match p {
            facet::PrimitiveType::Boolean => Ok(OwnedPrimitiveType::Boolean),
            facet::PrimitiveType::Numeric(n) => Ok(OwnedPrimitiveType::Numeric(n.try_into()?)),
            facet::PrimitiveType::Textual(t) => Ok(OwnedPrimitiveType::Textual(t.try_into()?)),
            facet::PrimitiveType::Never => Ok(OwnedPrimitiveType::Never),
        }
    }
}

impl TryFrom<&facet::NumericType> for OwnedNumericType {
    type Error = String;

    fn try_from(n: &facet::NumericType) -> Result<Self, Self::Error> {
        match n {
            facet::NumericType::Integer { signed } => {
                Ok(OwnedNumericType::Integer { signed: *signed })
            }
            facet::NumericType::Float { .. } => Ok(OwnedNumericType::Float),
        }
    }
}

impl TryFrom<&facet::TextualType> for OwnedTextualType {
    type Error = String;

    fn try_from(t: &facet::TextualType) -> Result<Self, Self::Error> {
        match t {
            facet::TextualType::Char => Ok(OwnedTextualType::Char),
            facet::TextualType::Str => Ok(OwnedTextualType::Str),
        }
    }
}

impl TryFrom<&facet::UserType> for OwnedUserType {
    type Error = String;

    fn try_from(u: &facet::UserType) -> Result<Self, Self::Error> {
        match u {
            facet::UserType::Struct(s) => Ok(OwnedUserType::Struct(s.try_into()?)),
            facet::UserType::Enum(e) => Ok(OwnedUserType::Enum(e.try_into()?)),
            facet::UserType::Union(u) => Ok(OwnedUserType::Union(u.try_into()?)),
            facet::UserType::Opaque => Ok(OwnedUserType::Opaque),
        }
    }
}

impl TryFrom<&facet::StructType> for OwnedStructType {
    type Error = String;

    fn try_from(s: &facet::StructType) -> Result<Self, Self::Error> {
        let fields: Result<Vec<_>, _> = s.fields.iter().map(|f| f.try_into()).collect();
        Ok(OwnedStructType { fields: fields? })
    }
}

impl TryFrom<&facet::Field> for OwnedField {
    type Error = String;

    fn try_from(f: &facet::Field) -> Result<Self, Self::Error> {
        Ok(OwnedField {
            name: f.name.to_string(),
            shape: (f.shape)().try_into()?,
            doc: f.doc.iter().map(|s| s.to_string()).collect(),
        })
    }
}

impl TryFrom<&facet::EnumType> for OwnedEnumType {
    type Error = String;

    fn try_from(e: &facet::EnumType) -> Result<Self, Self::Error> {
        let variants: Result<Vec<_>, _> = e.variants.iter().map(|v| v.try_into()).collect();
        Ok(OwnedEnumType {
            variants: variants?,
        })
    }
}

impl TryFrom<&facet::Variant> for OwnedVariant {
    type Error = String;

    fn try_from(v: &facet::Variant) -> Result<Self, Self::Error> {
        Ok(OwnedVariant {
            name: v.name.to_string(),
            data: (&v.data).try_into()?,
            doc: v.doc.iter().map(|s| s.to_string()).collect(),
        })
    }
}

impl TryFrom<&facet::UnionType> for OwnedUnionType {
    type Error = String;

    fn try_from(u: &facet::UnionType) -> Result<Self, Self::Error> {
        let fields: Result<Vec<_>, _> = u.fields.iter().map(|f| f.try_into()).collect();
        Ok(OwnedUnionType { fields: fields? })
    }
}
