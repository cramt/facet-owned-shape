use std::borrow::Cow;

use crate::box_cow::BoxCow;
use crate::vec_cow::VecCow;

pub trait ShapeFrom<F: ?Sized> {
    fn shape_from(f: &F) -> Result<Self, String>
    where
        Self: Sized;
}

impl<'a> ShapeFrom<&'static str> for Cow<'a, str> {
    fn shape_from(f: &&'static str) -> Result<Self, String> {
        Ok(Cow::Borrowed(*f))
    }
}

impl<'a> ShapeFrom<facet::Field> for CowField<'a> {
    fn shape_from(f: &facet::Field) -> Result<Self, String> {
        f.try_into()
    }
}

impl<'a> ShapeFrom<facet::Variant> for CowVariant<'a> {
    fn shape_from(v: &facet::Variant) -> Result<Self, String> {
        v.try_into()
    }
}

#[derive(Clone)]
pub enum ShapeList<'a, T, F>
where
    T: Clone + 'a,
    [T]: ToOwned<Owned = Vec<T>>,
{
    Cow(VecCow<'a, [T]>),
    Facet(&'a [F]),
}

impl<'a, T, F> std::fmt::Debug for ShapeList<'a, T, F>
where
    T: Clone + 'a + std::fmt::Debug + ShapeFrom<F>,
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeList::Cow(cow) => f.debug_tuple("Cow").field(cow).finish(),
            ShapeList::Facet(facet) => {
                // Convert on-the-fly for debug output
                let converted: Vec<T> = facet
                    .iter()
                    .map(|x| T::shape_from(x).expect("Debug conversion failed"))
                    .collect();
                f.debug_tuple("Cow")
                    .field(&VecCow::<[T]>::from(converted))
                    .finish()
            }
        }
    }
}

impl<'a, T, F> From<VecCow<'a, [T]>> for ShapeList<'a, T, F>
where
    T: Clone + 'a,
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn from(cow: VecCow<'a, [T]>) -> Self {
        ShapeList::Cow(cow)
    }
}

impl<'a, T, F> From<Vec<T>> for ShapeList<'a, T, F>
where
    T: Clone + 'a,
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn from(vec: Vec<T>) -> Self {
        ShapeList::Cow(vec.into())
    }
}

pub struct ShapeListIter<'a, T, F> {
    inner: ShapeListIterInner<'a, T, F>,
}

enum ShapeListIterInner<'a, T, F> {
    Cow(std::slice::Iter<'a, T>),
    Facet(std::slice::Iter<'a, F>),
}

impl<'a, T, F> Iterator for ShapeListIter<'a, T, F>
where
    T: 'a + Clone + ShapeFrom<F>,
    F: 'a,
    [T]: ToOwned<Owned = Vec<T>>,
{
    type Item = Cow<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            ShapeListIterInner::Cow(iter) => iter.next().map(|x| Cow::Borrowed(x)),
            ShapeListIterInner::Facet(iter) => iter.next().map(|x| {
                let t: T = T::shape_from(x).expect("Lazy conversion failed");
                Cow::Owned(t)
            }),
        }
    }
}

impl<'a, T, F> ShapeList<'a, T, F>
where
    T: Clone + 'a,
    [T]: ToOwned<Owned = Vec<T>>,
{
    pub fn iter(&self) -> ShapeListIter<'_, T, F>
    where
        T: ShapeFrom<F>,
    {
        match self {
            ShapeList::Cow(cow) => ShapeListIter {
                inner: ShapeListIterInner::Cow(cow.iter()),
            },
            ShapeList::Facet(facet) => ShapeListIter {
                inner: ShapeListIterInner::Facet(facet.iter()),
            },
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ShapeList::Cow(cow) => cow.len(),
            ShapeList::Facet(facet) => facet.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T, F> IntoIterator for &'a ShapeList<'_, T, F>
where
    T: 'a + Clone + ShapeFrom<F>,
    F: 'a,
    [T]: ToOwned<Owned = Vec<T>>,
{
    type Item = Cow<'a, T>;
    type IntoIter = ShapeListIter<'a, T, F>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct ShapeListIntoIter<'a, T, F> {
    inner: ShapeListIntoIterInner<'a, T, F>,
}

enum ShapeListIntoIterInner<'a, T, F> {
    CowOwned(std::vec::IntoIter<T>),
    CowBorrowed(std::slice::Iter<'a, T>),
    Facet(std::slice::Iter<'a, F>),
}

impl<'a, T, F> Iterator for ShapeListIntoIter<'a, T, F>
where
    T: 'a + Clone + ShapeFrom<F>,
    F: 'a,
    [T]: ToOwned<Owned = Vec<T>>,
{
    type Item = Cow<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            ShapeListIntoIterInner::CowOwned(iter) => iter.next().map(Cow::Owned),
            ShapeListIntoIterInner::CowBorrowed(iter) => iter.next().map(Cow::Borrowed),
            ShapeListIntoIterInner::Facet(iter) => iter.next().map(|x| {
                let t: T = T::shape_from(x).expect("Lazy conversion failed");
                Cow::Owned(t)
            }),
        }
    }
}

impl<'a, T, F> IntoIterator for ShapeList<'a, T, F>
where
    T: 'a + Clone + ShapeFrom<F>,
    F: 'a,
    [T]: ToOwned<Owned = Vec<T>>,
{
    type Item = Cow<'a, T>;
    type IntoIter = ShapeListIntoIter<'a, T, F>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ShapeList::Cow(VecCow::Owned(vec)) => ShapeListIntoIter {
                inner: ShapeListIntoIterInner::CowOwned(vec.into_iter()),
            },
            ShapeList::Cow(VecCow::Borrowed(slice)) => ShapeListIntoIter {
                inner: ShapeListIntoIterInner::CowBorrowed(slice.iter()),
            },
            ShapeList::Facet(facet) => ShapeListIntoIter {
                inner: ShapeListIntoIterInner::Facet(facet.iter()),
            },
        }
    }
}

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
    pub doc: ShapeList<'a, Cow<'a, str>, &'static str>,
}

#[derive(Clone, Debug)]
pub struct CowStructType<'a> {
    pub fields: ShapeList<'a, CowField<'a>, facet::Field>,
}

#[derive(Clone, Debug)]
pub struct CowUnionType<'a> {
    pub fields: ShapeList<'a, CowField<'a>, facet::Field>,
}

#[derive(Clone, Debug)]
pub struct CowVariant<'a> {
    pub name: Cow<'a, str>,
    pub data: CowStructType<'a>,
    pub doc: ShapeList<'a, Cow<'a, str>, &'static str>,
}

#[derive(Clone, Debug)]
pub struct CowEnumType<'a> {
    pub variants: ShapeList<'a, CowVariant<'a>, facet::Variant>,
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
            fields: s
                .fields
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

impl<'a> From<OwnedField> for CowField<'a> {
    fn from(f: OwnedField) -> Self {
        CowField {
            name: Cow::Owned(f.name),
            shape: f.shape.into(),
            doc: f.doc.into_iter().map(Cow::Owned).collect::<Vec<_>>().into(),
        }
    }
}

impl<'a> From<OwnedEnumType> for CowEnumType<'a> {
    fn from(e: OwnedEnumType) -> Self {
        CowEnumType {
            variants: e
                .variants
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

impl<'a> From<OwnedVariant> for CowVariant<'a> {
    fn from(v: OwnedVariant) -> Self {
        CowVariant {
            name: Cow::Owned(v.name),
            data: v.data.into(),
            doc: v.doc.into_iter().map(Cow::Owned).collect::<Vec<_>>().into(),
        }
    }
}

impl<'a> From<OwnedUnionType> for CowUnionType<'a> {
    fn from(u: OwnedUnionType) -> Self {
        CowUnionType {
            fields: u
                .fields
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .into(),
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
        Ok(CowStructType {
            fields: ShapeList::Facet(&s.fields),
        })
    }
}

impl<'a> TryFrom<&facet::Field> for CowField<'a> {
    type Error = String;

    fn try_from(f: &facet::Field) -> Result<Self, Self::Error> {
        Ok(CowField {
            name: f.name.into(),
            shape: (f.shape)().try_into()?,
            doc: ShapeList::Facet(f.doc),
        })
    }
}

impl<'a> TryFrom<&facet::EnumType> for CowEnumType<'a> {
    type Error = String;

    fn try_from(e: &facet::EnumType) -> Result<Self, Self::Error> {
        Ok(CowEnumType {
            variants: ShapeList::Facet(&e.variants),
        })
    }
}

impl<'a> TryFrom<&facet::Variant> for CowVariant<'a> {
    type Error = String;

    fn try_from(v: &facet::Variant) -> Result<Self, Self::Error> {
        Ok(CowVariant {
            name: v.name.into(),
            data: (&v.data).try_into()?,
            doc: ShapeList::Facet(v.doc),
        })
    }
}

impl<'a> TryFrom<&facet::UnionType> for CowUnionType<'a> {
    type Error = String;

    fn try_from(u: &facet::UnionType) -> Result<Self, Self::Error> {
        Ok(CowUnionType {
            fields: ShapeList::Facet(&u.fields),
        })
    }
}
