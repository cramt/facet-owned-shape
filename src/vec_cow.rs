use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::Deref;

pub enum VecCow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
{
    Borrowed(&'a B),
    Owned(B::Owned),
}

impl<'a, B> From<&'a B> for VecCow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
{
    fn from(value: &'a B) -> Self {
        VecCow::Borrowed(value)
    }
}

impl<'a, T> From<Vec<T>> for VecCow<'a, [T]>
where
    T: Clone + 'a,
{
    fn from(value: Vec<T>) -> Self {
        VecCow::Owned(value)
    }
}

impl<'a, B> Deref for VecCow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
    B::Owned: Borrow<B>,
{
    type Target = B;

    fn deref(&self) -> &Self::Target {
        match self {
            VecCow::Borrowed(x) => x,
            VecCow::Owned(x) => x.borrow(),
        }
    }
}

impl<'a, B> AsRef<B> for VecCow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
    B::Owned: Borrow<B>,
{
    fn as_ref(&self) -> &B {
        self.deref()
    }
}

impl<'a, B> Clone for VecCow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
    B::Owned: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Borrowed(x) => Self::Borrowed(x),
            Self::Owned(x) => Self::Owned(x.clone()),
        }
    }
}

impl<'a, B> Debug for VecCow<'a, B>
where
    B: 'a + ToOwned + ?Sized + Debug,
    B::Owned: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Borrowed(x) => f.debug_tuple("Borrowed").field(x).finish(),
            Self::Owned(x) => f.debug_tuple("Owned").field(x).finish(),
        }
    }
}

impl<'a, T> IntoIterator for VecCow<'a, [T]>
where
    T: Clone + 'a,
    [T]: ToOwned<Owned = Vec<T>>,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            VecCow::Borrowed(x) => x.to_vec().into_iter(),
            VecCow::Owned(x) => x.into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a VecCow<'_, [T]>
where
    T: 'a,
    [T]: ToOwned<Owned = Vec<T>>,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.deref().iter()
    }
}
