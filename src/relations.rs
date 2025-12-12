use facet::Facet;

#[derive(Facet)]
#[repr(C)]
enum Identifier {
    Stringish(String),
    Numberish(usize),
}

#[derive(Facet)]
#[repr(C)]
enum Many<T: 'static> {
    Lazy(Identifier),
    Eager(Vec<T>),
}
