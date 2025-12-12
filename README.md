# facet-psql-schema

Bridge library for generating [SeaQuery](https://github.com/SeaQL/sea-query) table definitions from [Facet](https://github.com/facet-rs/facet) shapes.

This crate allows you to automatically generate PostgreSQL database schemas (via `sea-query::Table`) directly from your Rust struct definitions, leveraging `facet`'s compile-time reflection capabilities.

## Usage

### Basic Conversion
Derive `Facet` on your structs, then convert the shape to a `sea_query::Table`.

```rust
use facet::Facet;
use facet_psql_schema::convert; // wrapper or directly TryFrom
use sea_query::Table;

#[derive(Facet)]
struct User {
    id: u64,
    username: String,
    is_active: bool,
}

fn main() {
    let shape = User::SHAPE;
    let table = Table::try_from(shape).expect("Schema conversion failed");
    
    // table.name == "user"
    // table.columns include "id" (BigInt), "username" (Text), "is_active" (Boolean)
}
```

### Primary Keys
Use the `#[facet(...)]` attribute to mark fields as primary keys.

```rust
#[derive(Facet)]
struct BlogPost {
    #[facet(psql::primary_key)]
    id: String, // Maps to Text Primary Key
    title: String,
}
```

### Supported Types

| Rust Type | PostgreSQL Type (SeaQuery) | Notes |
|-----------|------------------|-------|
| `bool` | `Boolean` | |
| `i8`, `u8`, `i16`, `u16` | `SmallInt` | |
| `i32`, `u32` | `Integer` | |
| `i64`, `u64`, `usize` | `BigInt` | |
| `f32` | `Real` | |
| `f64` | `DoublePrecision` | |
| `String`, `&str` | `Text` | References are supported |
| `char` | `Char(1)` | |
| `Vec<T>` | `Jsonb` | Currently maps to JSONB, not Array |
| `HashMap<K,V>` | `Jsonb` | |
| Nested Structs | `Jsonb` | |
| Enums | `Integer` | Field-level enums maps to Integer |
| `Option<T>` | `Nullable` | Wraps the inner type |

## Limitations
- **Generics**: Generic structs (`struct Foo<T>`) work only when monomorphized (e.g., `Foo::<u64>::SHAPE`).
- **Fixed-Size Arrays**: `[T; N]` are currently not supported/mapped.
- **Enums**: Top-level enums cannot be converted to Tables. Field-level enums map to `Integer`.
