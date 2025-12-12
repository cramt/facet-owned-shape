# AGENTS.md - Context & Knowledge Base

This document captures high-level context, architectural decisions, and "gotchas" for AI agents working on `facet-psql-schema`.

## Core Architecture
- **Goal**: One-way conversion from `facet::Shape` (reflection data) -> `sea_query::Table` (SQL schema definition).
- **Entry Point**: `impl TryFrom<&facet::Shape> for sea_query::Table`.
- **Location**: `src/conversion.rs`.

## Type System & Logic
The conversion logic relies heavily on `facet::Shape` properties:
- **Identifier Check**: Strings, Vecs, and HashMaps are often identified by recursive or substring checks on `type_identifier` (e.g., checking for "Vec" or "alloc::string::String").
- **Layout Check**: Integers and Floats use `shape.layout.size()` to determine the correct SQL precision (`SmallInt` vs `BigInt`, `Real` vs `Double`).
- **Attribute Parsing**:
    - Attributes are accessed via `field.attributes`.
    - Namespace is strictly checked: `attr.ns == Some("psql")`.
    - Key is strictly checked: `attr.key == "primary_key"`.
    - *Note*: Ensure safe access to attributes; they are iterated directly.
    - **Constraints**: Currently enforces strictly *one* primary key per table. Defining multiple PK attributes results in `ConversionError::MultiplePrimaryKeys`.

## "Gotchas" & Decisions
1.  **Reference Types**:
    - `&str` and `&'static str` are extremely common in Rust.
    - **Decision**: Logic explicitly checks `shape.ty` for `Pointer/Reference`. If inner type looks like "str", it forces `DataType::Text`.
    - **Why**: Prevents "Unsupported Type" errors for common borrowed string patterns.

2.  **Generics**:
    - `facet` generates unique shapes for monomorphized types (e.g., `Container<u64>`).
    - **Decision**: The conversion works "by accident" of design for these. `Container::<u64>` results in a table named `container` (lowercase of identifier).
    - **Warning**: Code generating schemas for *generic* definitions without concrete types is impossible in this model.

3.  **Complex Types -> Jsonb**:
    - `Vec<T>`, `HashMap`, and Nested Structs are all mapped to `DataType::Jsonb`.
    - **Reasoning**: Easiest way to support complex Rust data without generating auxiliary tables or sophisticated composite integer types just yet.

4.  **Enum Handling**:
    - **Top-Level**: `UserType::Enum` passed to `Table::try_from` returns `ConversionError::NotAStruct`. Tables must map to Structs.
    - **Fields**: Enum fields map to `DataType::Integer`. This assumes enums are stored as discriminants.

## Testing Strategy
- **File**: `tests/comprehensive_conversion_tests.rs`.
- **Pattern**:
    1.  Define a Struct/Shape with `#[derive(Facet)]`.
    2.  `Table::try_from(MyStruct::SHAPE)`.
    3.  Assert `table.name`, `column.len()`.
    4.  Iterate columns and assert `data_type`, `nullable`.
    5.  Check `table.primary_key` existence and column matches.
    6.  *Primary Key Update*: Tests were recently refactored to explicitly use `#[facet(psql::primary_key)]` on `id` fields.

## Future Areas
- **Better Array Support**: Map `Vec<T>` to `Array(Box<DataType>)` instead of `Jsonb`.
- **Postgres Enums**: Create actual `CREATE TYPE AS ENUM` statements for Rust enums.
- **Composite Types**: Map nested structs to Postgres composite types.
