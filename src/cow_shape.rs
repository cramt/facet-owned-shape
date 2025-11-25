use std::borrow::Cow;

use crate::box_cow::BoxCow;

#[derive(Clone, Debug)]
pub struct CowMapDef<'a> {
    pub k: CowShape<'a>,
    pub v: CowShape<'a>,
}

#[derive(Clone, Debug)]
pub struct CowSetDef<'a> {
    pub t: CowShape<'a>,
}

#[derive(Clone, Debug)]
pub struct CowListDef<'a> {
    pub t: CowShape<'a>,
}

#[derive(Clone, Debug)]
pub struct CowArrayDef<'a> {
    pub t: CowShape<'a>,
    pub n: usize,
}

#[derive(Clone, Debug)]
pub struct CowOptionDef<'a> {
    pub t: CowShape<'a>,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum CowDef<'a> {
    Undefined,
    Scalar,
    Map(CowMapDef<'a>),
    Set(CowSetDef<'a>),
    List(CowListDef<'a>),
    Array(CowArrayDef<'a>),
    Option(CowOptionDef<'a>),
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum CowNumericType {
    Integer { signed: bool },
    Float,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum CowTextualType {
    Char = 0,
    Str = 1,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum CowPrimitiveType {
    Boolean,
    Numeric(CowNumericType),
    Textual(CowTextualType),
    Never,
}

#[derive(Clone, Debug)]
pub struct CowSequenceType<'a> {
    pub t: CowShape<'a>,
}

#[derive(Clone, Debug)]
pub struct CowField<'a> {
    pub name: Cow<'a, str>,
    pub shape: CowShape<'a>,
    pub doc: Vec<Cow<'a, str>>,
}

#[derive(Clone, Debug)]
pub struct CowStructType<'a> {
    pub fields: Vec<CowField<'a>>,
}

#[derive(Clone, Debug)]
pub struct CowUnionType<'a> {
    pub fields: Vec<CowField<'a>>,
}

#[derive(Clone, Debug)]
pub struct CowVariant<'a> {
    pub name: Cow<'a, str>,
    pub data: CowStructType<'a>,
    pub doc: Vec<Cow<'a, str>>,
}

#[derive(Clone, Debug)]
pub struct CowEnumType<'a> {
    pub variants: Vec<CowVariant<'a>>,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum CowUserType<'a> {
    Struct(CowStructType<'a>),
    Enum(CowEnumType<'a>),
    Union(CowUnionType<'a>),
    Opaque,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum CowType<'a> {
    Primitive(CowPrimitiveType),
    Sequence(CowSequenceType<'a>),
    User(CowUserType<'a>),
}

#[derive(Clone, Debug)]
pub struct CowShape<'a> {
    pub type_identifier: Cow<'a, str>,
    pub def: BoxCow<'a, CowDef<'a>>,
    pub ty: BoxCow<'a, CowType<'a>>,
}

use crate::owned_shape::*;

impl<'a> From<OwnedShape> for CowShape<'a> {
    fn from(shape: OwnedShape) -> Self {
        CowShape {
            type_identifier: Cow::Owned(shape.type_identifier),
            def: BoxCow::Owned(Box::new((*shape.def).into())),
            ty: BoxCow::Owned(Box::new((*shape.ty).into())),
        }
    }
}

impl<'a> From<OwnedDef> for CowDef<'a> {
    fn from(def: OwnedDef) -> Self {
        match def {
            OwnedDef::Undefined => CowDef::Undefined,
            OwnedDef::Scalar => CowDef::Scalar,
            OwnedDef::Map(d) => CowDef::Map(CowMapDef {
                k: d.k.into(),
                v: d.v.into(),
            }),
            OwnedDef::Set(d) => CowDef::Set(CowSetDef { t: d.t.into() }),
            OwnedDef::List(d) => CowDef::List(CowListDef { t: d.t.into() }),
            OwnedDef::Array(d) => CowDef::Array(CowArrayDef {
                t: d.t.into(),
                n: d.n,
            }),
            OwnedDef::Option(d) => CowDef::Option(CowOptionDef { t: d.t.into() }),
        }
    }
}

impl<'a> From<OwnedType> for CowType<'a> {
    fn from(ty: OwnedType) -> Self {
        match ty {
            OwnedType::Primitive(p) => CowType::Primitive(p.into()),
            OwnedType::Sequence(s) => CowType::Sequence(CowSequenceType { t: s.t.into() }),
            OwnedType::User(u) => CowType::User(u.into()),
        }
    }
}

impl From<OwnedPrimitiveType> for CowPrimitiveType {
    fn from(p: OwnedPrimitiveType) -> Self {
        match p {
            OwnedPrimitiveType::Boolean => CowPrimitiveType::Boolean,
            OwnedPrimitiveType::Numeric(n) => CowPrimitiveType::Numeric(n.into()),
            OwnedPrimitiveType::Textual(t) => CowPrimitiveType::Textual(t.into()),
            OwnedPrimitiveType::Never => CowPrimitiveType::Never,
        }
    }
}

impl From<OwnedNumericType> for CowNumericType {
    fn from(n: OwnedNumericType) -> Self {
        match n {
            OwnedNumericType::Integer { signed } => CowNumericType::Integer { signed },
            OwnedNumericType::Float => CowNumericType::Float,
        }
    }
}

impl From<OwnedTextualType> for CowTextualType {
    fn from(t: OwnedTextualType) -> Self {
        match t {
            OwnedTextualType::Char => CowTextualType::Char,
            OwnedTextualType::Str => CowTextualType::Str,
        }
    }
}

impl<'a> From<OwnedUserType> for CowUserType<'a> {
    fn from(u: OwnedUserType) -> Self {
        match u {
            OwnedUserType::Struct(s) => CowUserType::Struct(s.into()),
            OwnedUserType::Enum(e) => CowUserType::Enum(e.into()),
            OwnedUserType::Union(u) => CowUserType::Union(u.into()),
            OwnedUserType::Opaque => CowUserType::Opaque,
        }
    }
}

impl<'a> From<OwnedStructType> for CowStructType<'a> {
    fn from(s: OwnedStructType) -> Self {
        CowStructType {
            fields: s.fields.into_iter().map(Into::into).collect(),
        }
    }
}

impl<'a> From<OwnedField> for CowField<'a> {
    fn from(f: OwnedField) -> Self {
        CowField {
            name: Cow::Owned(f.name),
            shape: f.shape.into(),
            doc: f.doc.into_iter().map(Cow::Owned).collect(),
        }
    }
}

impl<'a> From<OwnedEnumType> for CowEnumType<'a> {
    fn from(e: OwnedEnumType) -> Self {
        CowEnumType {
            variants: e.variants.into_iter().map(Into::into).collect(),
        }
    }
}

impl<'a> From<OwnedVariant> for CowVariant<'a> {
    fn from(v: OwnedVariant) -> Self {
        CowVariant {
            name: Cow::Owned(v.name),
            data: v.data.into(),
            doc: v.doc.into_iter().map(Cow::Owned).collect(),
        }
    }
}

impl<'a> From<OwnedUnionType> for CowUnionType<'a> {
    fn from(u: OwnedUnionType) -> Self {
        CowUnionType {
            fields: u.fields.into_iter().map(Into::into).collect(),
        }
    }
}

impl<'a> TryFrom<&facet::Shape> for CowShape<'a> {
    type Error = String;

    fn try_from(shape: &facet::Shape) -> Result<Self, Self::Error> {
        Ok(CowShape {
            type_identifier: shape.type_identifier.into(),
            def: CowDef::try_from(&shape.def)?.into(),
            ty: CowType::try_from(&shape.ty)?.into(),
        })
    }
}

impl<'a> TryFrom<&facet::Def> for CowDef<'a> {
    type Error = String;

    fn try_from(def: &facet::Def) -> Result<Self, Self::Error> {
        match def {
            facet::Def::Undefined => Ok(CowDef::Undefined),
            facet::Def::Scalar => Ok(CowDef::Scalar),
            facet::Def::Map(map_def) => Ok(CowDef::Map(CowMapDef {
                k: map_def.k().try_into()?,
                v: map_def.v().try_into()?,
            })),
            facet::Def::Set(set_def) => Ok(CowDef::Set(CowSetDef {
                t: set_def.t().try_into()?,
            })),
            facet::Def::List(list_def) => Ok(CowDef::List(CowListDef {
                t: list_def.t().try_into()?,
            })),
            facet::Def::Slice(slice_def) => Ok(CowDef::List(CowListDef {
                t: slice_def.t().try_into()?,
            })),
            facet::Def::Array(array_def) => Ok(CowDef::Array(CowArrayDef {
                t: array_def.t().try_into()?,
                n: array_def.n,
            })),
            facet::Def::Option(option_def) => Ok(CowDef::Option(CowOptionDef {
                t: option_def.t().try_into()?,
            })),
            _ => Err("Unsupported Def variant".to_string()),
        }
    }
}

impl<'a> TryFrom<&facet::Type> for CowType<'a> {
    type Error = String;

    fn try_from(ty: &facet::Type) -> Result<Self, Self::Error> {
        match ty {
            facet::Type::Primitive(p) => Ok(CowType::Primitive(p.try_into()?)),
            facet::Type::Sequence(s) => Ok(CowType::Sequence(CowSequenceType {
                t: match s {
                    facet::SequenceType::Array(array_type) => array_type.t.try_into()?,
                    facet::SequenceType::Slice(slice_type) => slice_type.t.try_into()?,
                },
            })),
            facet::Type::User(u) => Ok(CowType::User(u.try_into()?)),
            facet::Type::Pointer(_) => Err("Pointer types not supported".to_string()),
        }
    }
}

impl TryFrom<&facet::PrimitiveType> for CowPrimitiveType {
    type Error = String;

    fn try_from(p: &facet::PrimitiveType) -> Result<Self, Self::Error> {
        match p {
            facet::PrimitiveType::Boolean => Ok(CowPrimitiveType::Boolean),
            facet::PrimitiveType::Numeric(n) => Ok(CowPrimitiveType::Numeric(n.try_into()?)),
            facet::PrimitiveType::Textual(t) => Ok(CowPrimitiveType::Textual(t.try_into()?)),
            facet::PrimitiveType::Never => Ok(CowPrimitiveType::Never),
        }
    }
}

impl TryFrom<&facet::NumericType> for CowNumericType {
    type Error = String;

    fn try_from(n: &facet::NumericType) -> Result<Self, Self::Error> {
        match n {
            facet::NumericType::Integer { signed } => {
                Ok(CowNumericType::Integer { signed: *signed })
            }
            facet::NumericType::Float { .. } => Ok(CowNumericType::Float),
        }
    }
}

impl TryFrom<&facet::TextualType> for CowTextualType {
    type Error = String;

    fn try_from(t: &facet::TextualType) -> Result<Self, Self::Error> {
        match t {
            facet::TextualType::Char => Ok(CowTextualType::Char),
            facet::TextualType::Str => Ok(CowTextualType::Str),
        }
    }
}

impl<'a> TryFrom<&facet::UserType> for CowUserType<'a> {
    type Error = String;

    fn try_from(u: &facet::UserType) -> Result<Self, Self::Error> {
        match u {
            facet::UserType::Struct(s) => Ok(CowUserType::Struct(s.try_into()?)),
            facet::UserType::Enum(e) => Ok(CowUserType::Enum(e.try_into()?)),
            facet::UserType::Union(u) => Ok(CowUserType::Union(u.try_into()?)),
            facet::UserType::Opaque => Ok(CowUserType::Opaque),
        }
    }
}

impl<'a> TryFrom<&facet::StructType> for CowStructType<'a> {
    type Error = String;

    fn try_from(s: &facet::StructType) -> Result<Self, Self::Error> {
        let fields: Result<Vec<_>, _> = s.fields.iter().map(|f| f.try_into()).collect();
        Ok(CowStructType { fields: fields? })
    }
}

impl<'a> TryFrom<&facet::Field> for CowField<'a> {
    type Error = String;

    fn try_from(f: &facet::Field) -> Result<Self, Self::Error> {
        Ok(CowField {
            name: f.name.into(),
            shape: (f.shape)().try_into()?,
            doc: f.doc.iter().map(|s| (*s).into()).collect(),
        })
    }
}

impl<'a> TryFrom<&facet::EnumType> for CowEnumType<'a> {
    type Error = String;

    fn try_from(e: &facet::EnumType) -> Result<Self, Self::Error> {
        let variants: Result<Vec<_>, _> = e.variants.iter().map(|v| v.try_into()).collect();
        Ok(CowEnumType {
            variants: variants?,
        })
    }
}

impl<'a> TryFrom<&facet::Variant> for CowVariant<'a> {
    type Error = String;

    fn try_from(v: &facet::Variant) -> Result<Self, Self::Error> {
        Ok(CowVariant {
            name: v.name.into(),
            data: (&v.data).try_into()?,
            doc: v.doc.iter().map(|s| (*s).into()).collect(),
        })
    }
}

impl<'a> TryFrom<&facet::UnionType> for CowUnionType<'a> {
    type Error = String;

    fn try_from(u: &facet::UnionType) -> Result<Self, Self::Error> {
        let fields: Result<Vec<_>, _> = u.fields.iter().map(|f| f.try_into()).collect();
        Ok(CowUnionType { fields: fields? })
    }
}
