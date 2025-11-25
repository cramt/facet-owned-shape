use std::fmt::Debug;

pub enum BoxCow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
{
    Borrowed(&'a B),
    Owned(Box<<B as ToOwned>::Owned>),
}

impl<'a, B> From<&'a B> for BoxCow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
{
    fn from(value: &'a B) -> BoxCow<'a, B> {
        BoxCow::Borrowed(value)
    }
}

impl<'a, B> From<B> for BoxCow<'a, B>
where
    B: 'a + ToOwned<Owned = B>,
{
    fn from(value: B) -> BoxCow<'a, B> {
        BoxCow::Owned(Box::new(value))
    }
}

impl<'a, T: ?Sized + ToOwned<Owned = T>> AsRef<T> for BoxCow<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            BoxCow::Borrowed(x) => x,
            BoxCow::Owned(x) => x.as_ref(),
        }
    }
}

impl<B: ToOwned<Owned = B>> Clone for BoxCow<'_, B> {
    fn clone(&self) -> Self {
        match self {
            Self::Borrowed(arg0) => Self::Borrowed(*arg0),
            Self::Owned(arg0) => Self::Owned(Box::new(arg0.as_ref().to_owned())),
        }
    }
}

impl<B: ToOwned<Owned = B> + std::fmt::Debug> Debug for BoxCow<'_, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Borrowed(arg0) => f.debug_tuple("Borrowed").field(arg0).finish(),
            Self::Owned(arg0) => f.debug_tuple("Owned").field(arg0).finish(),
        }
    }
}
