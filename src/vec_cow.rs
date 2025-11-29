use std::fmt::Debug;

pub enum VecCow<'a, T> {
    Borrowed(&'a [T]),
    Owned(Vec<T>),
}

impl<'a, T> From<&'a [T]> for VecCow<'a, T> {
    fn from(value: &'a [T]) -> Self {
        VecCow::Borrowed(value)
    }
}

impl<'a, T> From<Vec<T>> for VecCow<'a, T> {
    fn from(value: Vec<T>) -> Self {
        VecCow::Owned(value)
    }
}

impl<'a, T> AsRef<[T]> for VecCow<'a, T> {
    fn as_ref(&self) -> &[T] {
        match self {
            VecCow::Borrowed(x) => x,
            VecCow::Owned(x) => x.as_ref(),
        }
    }
}

impl<'a, T: Clone> Clone for VecCow<'a, T> {
    fn clone(&self) -> Self {
        match self {
            Self::Borrowed(x) => Self::Borrowed(x),
            Self::Owned(x) => Self::Owned(x.clone()),
        }
    }
}

impl<'a, T: Debug> Debug for VecCow<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Borrowed(x) => f.debug_tuple("Borrowed").field(x).finish(),
            Self::Owned(x) => f.debug_tuple("Owned").field(x).finish(),
        }
    }
}
