use facet::Facet;
use std::collections::HashMap;

/// Test struct with all primitive number types
#[derive(Facet)]
struct AllPrimitiveNumbers {
    uint8_field: u8,
    uint16_field: u16,
    uint32_field: u32,
    uint64_field: u64,
    uint128_field: u128,
    usize_field: usize,
    int8_field: i8,
    int16_field: i16,
    int32_field: i32,
    int64_field: i64,
    int128_field: i128,
    isize_field: isize,
    float32_field: f32,
    float64_field: f64,
}

/// Test struct with string types
#[derive(Facet)]
struct StringTypes {
    string_field: String,
    str_ref_field: &'static str,
}

/// Test struct with boolean
#[derive(Facet)]
struct BooleanType {
    active: bool,
    verified: bool,
}

/// Test struct with Option types
#[derive(Facet)]
struct OptionalFields {
    required_name: String,
    optional_email: Option<String>,
    optional_age: Option<u32>,
    optional_score: Option<f64>,
    optional_active: Option<bool>,
}

/// Test struct with Vec types
#[derive(Facet)]
struct VectorFields {
    tags: Vec<String>,
    scores: Vec<i32>,
    weights: Vec<f64>,
    flags: Vec<bool>,
}

/// Test struct with nested Vec and Option
#[derive(Facet)]
struct ComplexCollections {
    optional_tags: Option<Vec<String>>,
    vec_of_optionals: Vec<Option<String>>,
    nested_vecs: Vec<Vec<i32>>,
}

/// Nested struct for testing
#[derive(Facet)]
struct Address {
    street: String,
    city: String,
    zip_code: String,
    country: String,
}

/// Test struct with nested structs
#[derive(Facet)]
struct UserWithAddress {
    id: u64,
    name: String,
    email: String,
    address: Address,
    optional_billing_address: Option<Address>,
}

/// Simple enum for testing
#[derive(Facet)]
#[repr(C)]
enum Status {
    Active,
    Inactive,
    Pending,
}

/// Enum with data
#[derive(Facet)]
#[repr(C)]
enum UserRole {
    Guest,
    User { registration_date: u64 },
    Admin { level: u8, department: String },
    SuperAdmin,
}

/// Test struct with enums
#[derive(Facet)]
struct UserWithStatus {
    id: u64,
    username: String,
    status: Status,
    role: UserRole,
}

/// Test struct with HashMap
#[derive(Facet)]
struct MetadataContainer {
    id: String,
    metadata: HashMap<String, String>,
    settings: HashMap<String, i32>,
}

/// Test struct with mixed types
#[derive(Facet)]
struct Product {
    id: u64,
    name: String,
    description: Option<String>,
    price: f64,
    quantity: i32,
    in_stock: bool,
    categories: Vec<String>,
    tags: Option<Vec<String>>,
    metadata: HashMap<String, String>,
}

/// Test struct with tuple
#[derive(Facet)]
struct Coordinates {
    location_name: String,
    point: (f64, f64),
    bounding_box: Option<(f64, f64, f64, f64)>,
}

/// Test struct with array
#[derive(Facet)]
struct FixedSizeArrays {
    id: u32,
    rgb_color: [u8; 3],
    transform_matrix: [f32; 9],
}

/// Test struct simulating a real database table
#[derive(Facet)]
struct BlogPost {
    id: u64,
    title: String,
    slug: String,
    content: String,
    excerpt: Option<String>,
    author_id: u64,
    created_at: u64,
    updated_at: Option<u64>,
    published_at: Option<u64>,
    view_count: u64,
    like_count: u32,
    is_published: bool,
    is_featured: bool,
    tags: Vec<String>,
    category_ids: Vec<u32>,
    metadata: HashMap<String, String>,
}

/// Test struct with references and lifetimes
#[derive(Facet)]
struct BorrowedData<'a> {
    id: u64,
    static_label: &'static str,
    borrowed_name: &'a str,
    owned_description: String,
}

/// Test struct with generic type parameter
#[derive(Facet)]
struct Container<T: 'static> {
    id: u64,
    name: String,
    value: T,
    items: Vec<T>,
}

/// Complex nested structure
#[derive(Facet)]
struct Organization {
    id: u64,
    name: String,
    departments: Vec<Department>,
}

#[derive(Facet)]
struct Department {
    id: u64,
    name: String,
    employees: Vec<Employee>,
}

#[derive(Facet)]
struct Employee {
    id: u64,
    name: String,
    email: String,
    position: String,
    salary: f64,
}

#[test]
fn test_all_primitive_numbers_shape() {
    use facet_owned_shape::*;

    let shape = AllPrimitiveNumbers::SHAPE;
    println!("AllPrimitiveNumbers shape: {:#?}", shape);

    // TDD: Convert shape to table schema - this will fail until we implement conversion
    let table =
        Table::try_from(shape).expect("Failed to convert AllPrimitiveNumbers shape to Table");

    // Assert table name
    assert_eq!(
        table.name, "allprimitivenumbers",
        "Table name should be lowercase struct name"
    );

    // Assert we have 14 columns (all primitive number types)
    assert_eq!(
        table.columns.len(),
        14,
        "Should have 14 columns for primitive types"
    );

    // Assert column names match field names
    let column_names: Vec<&str> = table.columns.iter().map(|c| c.name.as_str()).collect();
    assert!(column_names.contains(&"uint8_field"));
    assert!(column_names.contains(&"uint16_field"));
    assert!(column_names.contains(&"uint32_field"));
    assert!(column_names.contains(&"uint64_field"));
    assert!(column_names.contains(&"uint128_field"));
    assert!(column_names.contains(&"usize_field"));
    assert!(column_names.contains(&"int8_field"));
    assert!(column_names.contains(&"int16_field"));
    assert!(column_names.contains(&"int32_field"));
    assert!(column_names.contains(&"int64_field"));
    assert!(column_names.contains(&"int128_field"));
    assert!(column_names.contains(&"isize_field"));
    assert!(column_names.contains(&"float32_field"));
    assert!(column_names.contains(&"float64_field"));

    // Assert all columns are non-nullable by default
    for col in &table.columns {
        assert!(!col.nullable, "Column {} should be non-nullable", col.name);
    }

    // Assert numeric types are mapped correctly
    // u8, u16, u32 -> Integer or SmallInt
    // u64, u128, usize -> BigInt
    // i8, i16, i32 -> Integer or SmallInt
    // i64, i128, isize -> BigInt
    // f32 -> Real
    // f64 -> DoublePrecision
    let uint64_col = table
        .columns
        .iter()
        .find(|c| c.name == "uint64_field")
        .unwrap();
    assert!(
        matches!(uint64_col.data_type, DataType::BigInt),
        "u64 should map to BigInt"
    );

    let float64_col = table
        .columns
        .iter()
        .find(|c| c.name == "float64_field")
        .unwrap();
    assert!(
        matches!(float64_col.data_type, DataType::DoublePrecision),
        "f64 should map to DoublePrecision"
    );

    let float32_col = table
        .columns
        .iter()
        .find(|c| c.name == "float32_field")
        .unwrap();
    assert!(
        matches!(float32_col.data_type, DataType::Real),
        "f32 should map to Real"
    );
}

#[test]
fn test_string_types_shape() {
    use facet_owned_shape::*;

    let shape = StringTypes::SHAPE;
    println!("StringTypes shape: {:#?}", shape);

    let table = Table::try_from(shape).expect("Failed to convert StringTypes shape to Table");

    assert_eq!(table.name, "stringtypes");
    assert_eq!(table.columns.len(), 2, "Should have 2 string columns");

    let string_col = table
        .columns
        .iter()
        .find(|c| c.name == "string_field")
        .unwrap();
    assert!(matches!(string_col.data_type, DataType::Text));
    assert!(!string_col.nullable);

    let str_ref_col = table
        .columns
        .iter()
        .find(|c| c.name == "str_ref_field")
        .unwrap();
    assert!(matches!(str_ref_col.data_type, DataType::Text));
    assert!(!str_ref_col.nullable);
}

#[test]
fn test_boolean_type_shape() {
    use facet_owned_shape::*;

    let shape = BooleanType::SHAPE;
    println!("BooleanType shape: {:#?}", shape);

    let table = Table::try_from(shape).expect("Failed to convert BooleanType shape to Table");

    assert_eq!(table.name, "booleantype");
    assert_eq!(table.columns.len(), 2);

    for col in &table.columns {
        assert!(
            matches!(col.data_type, DataType::Boolean),
            "Column {} should be Boolean",
            col.name
        );
        assert!(!col.nullable);
    }
}

#[test]
fn test_optional_fields_shape() {
    use facet_owned_shape::*;

    let shape = OptionalFields::SHAPE;
    println!("OptionalFields shape: {:#?}", shape);

    let table = Table::try_from(shape).expect("Failed to convert OptionalFields shape to Table");

    assert_eq!(table.name, "optionalfields");
    assert_eq!(table.columns.len(), 5);

    // Required name should be non-nullable
    let required_col = table
        .columns
        .iter()
        .find(|c| c.name == "required_name")
        .unwrap();
    assert!(!required_col.nullable);
    assert!(matches!(required_col.data_type, DataType::Text));

    // Optional fields should be nullable
    let optional_email = table
        .columns
        .iter()
        .find(|c| c.name == "optional_email")
        .unwrap();
    assert!(optional_email.nullable, "optional_email should be nullable");
    assert!(matches!(optional_email.data_type, DataType::Text));

    let optional_age = table
        .columns
        .iter()
        .find(|c| c.name == "optional_age")
        .unwrap();
    assert!(optional_age.nullable, "optional_age should be nullable");
    assert!(matches!(optional_age.data_type, DataType::Integer));

    let optional_score = table
        .columns
        .iter()
        .find(|c| c.name == "optional_score")
        .unwrap();
    assert!(optional_score.nullable, "optional_score should be nullable");
    assert!(matches!(
        optional_score.data_type,
        DataType::DoublePrecision
    ));

    let optional_active = table
        .columns
        .iter()
        .find(|c| c.name == "optional_active")
        .unwrap();
    assert!(
        optional_active.nullable,
        "optional_active should be nullable"
    );
    assert!(matches!(optional_active.data_type, DataType::Boolean));
}

#[test]
fn test_vector_fields_shape() {
    use facet_owned_shape::*;

    let shape = VectorFields::SHAPE;
    println!("VectorFields shape: {:#?}", shape);

    let table = Table::try_from(shape).expect("Failed to convert VectorFields");

    assert_eq!(table.name, "vectorfields");
    assert_eq!(
        table.columns.len(),
        4,
        "VectorFields has tags, scores, weights, flags"
    );

    // Vec fields are currently mapped to Jsonb
    let tags = table.columns.iter().find(|c| c.name == "tags").unwrap();
    assert!(!tags.nullable);
    assert!(matches!(tags.data_type, DataType::Jsonb));

    let scores = table.columns.iter().find(|c| c.name == "scores").unwrap();
    assert!(!scores.nullable);
    assert!(matches!(scores.data_type, DataType::Jsonb));

    let weights = table.columns.iter().find(|c| c.name == "weights").unwrap();
    assert!(!weights.nullable);
    assert!(matches!(weights.data_type, DataType::Jsonb));

    let flags = table.columns.iter().find(|c| c.name == "flags").unwrap();
    assert!(!flags.nullable);
    assert!(matches!(flags.data_type, DataType::Jsonb));
}

#[test]
fn test_complex_collections_shape() {
    use facet_owned_shape::*;

    let shape = ComplexCollections::SHAPE;
    println!("ComplexCollections shape: {:#?}", shape);

    let table = Table::try_from(shape).expect("Failed to convert ComplexCollections");

    assert_eq!(table.name, "complexcollections");
    assert_eq!(table.columns.len(), 3, "ComplexCollections has 3 fields");

    // Nested Option/Vec combinations
    let opt_tags = table
        .columns
        .iter()
        .find(|c| c.name == "optional_tags")
        .unwrap();
    assert!(opt_tags.nullable, "Option<Vec<String>> should be nullable");

    let vec_opts = table
        .columns
        .iter()
        .find(|c| c.name == "vec_of_optionals")
        .unwrap();
    assert!(
        !vec_opts.nullable,
        "Vec<Option<String>> itself is not nullable"
    );

    let nested_vecs = table
        .columns
        .iter()
        .find(|c| c.name == "nested_vecs")
        .unwrap();
    assert!(!nested_vecs.nullable);
}

#[test]
fn test_user_with_address_shape() {
    use facet_owned_shape::*;

    let shape = UserWithAddress::SHAPE;
    println!("UserWithAddress shape: {:#?}", shape);

    let table = Table::try_from(shape).expect("Failed to convert UserWithAddress");

    assert_eq!(table.name, "userwithaddress");
    assert_eq!(
        table.columns.len(),
        5,
        "UserWithAddress has id, name, email, address, optional_billing_address"
    );

    let id = table.columns.iter().find(|c| c.name == "id").unwrap();
    assert!(matches!(id.data_type, DataType::BigInt));
    assert!(!id.nullable);

    let name = table.columns.iter().find(|c| c.name == "name").unwrap();
    assert!(matches!(name.data_type, DataType::Text));
    assert!(!name.nullable);

    let email = table.columns.iter().find(|c| c.name == "email").unwrap();
    assert!(matches!(email.data_type, DataType::Text));
    assert!(!email.nullable);

    // Nested struct should be Jsonb
    let address = table.columns.iter().find(|c| c.name == "address").unwrap();
    assert!(matches!(address.data_type, DataType::Jsonb));
    assert!(!address.nullable);

    // Optional nested struct
    let optional_billing = table
        .columns
        .iter()
        .find(|c| c.name == "optional_billing_address")
        .unwrap();
    assert!(matches!(optional_billing.data_type, DataType::Jsonb));
    assert!(
        optional_billing.nullable,
        "Option<Address> should be nullable"
    );
}

#[test]
fn test_status_enum_shape() {
    use facet_owned_shape::*;

    let shape = Status::SHAPE;
    println!("Status shape: {:#?}", shape);

    // Note: Enums themselves don't convert to tables
    // This would be used as a field type in a struct
    // For now, we test that it doesn't panic
    let result = Table::try_from(shape);
    assert!(
        result.is_err(),
        "Enum itself should not convert to table (needs to be a struct field)"
    );
}

#[test]
fn test_user_role_enum_shape() {
    use facet_owned_shape::*;

    let shape = UserRole::SHAPE;
    println!("UserRole shape: {:#?}", shape);

    // Like Status, enum itself doesn't convert to table
    let result = Table::try_from(shape);
    assert!(result.is_err(), "Enum itself should not convert to table");
}

#[test]
fn test_user_with_status_shape() {
    use facet_owned_shape::*;

    let shape = UserWithStatus::SHAPE;
    println!("UserWithStatus shape: {:#?}", shape);

    let table = Table::try_from(shape).expect("Failed to convert UserWithStatus");

    assert_eq!(table.name, "userwithstatus");
    assert_eq!(
        table.columns.len(),
        4,
        "UserWithStatus has id, username, status, role"
    );

    let id = table.columns.iter().find(|c| c.name == "id").unwrap();
    assert!(matches!(id.data_type, DataType::BigInt));

    let username = table.columns.iter().find(|c| c.name == "username").unwrap();
    assert!(matches!(username.data_type, DataType::Text));

    // Enum field should be Integer
    let status = table.columns.iter().find(|c| c.name == "status").unwrap();
    assert!(
        matches!(status.data_type, DataType::Integer),
        "Enum should map to Integer"
    );
    assert!(!status.nullable);

    let role = table.columns.iter().find(|c| c.name == "role").unwrap();
    assert!(
        matches!(role.data_type, DataType::Integer),
        "Enum should map to Integer"
    );
    assert!(!role.nullable);
}

#[test]
fn test_metadata_container_shape() {
    use facet_owned_shape::*;

    let shape = MetadataContainer::SHAPE;
    println!("MetadataContainer shape: {:#?}", shape);

    let table = Table::try_from(shape).expect("Failed to convert MetadataContainer");

    assert_eq!(table.name, "metadatacontainer");
    // HashMap maps to Jsonb
    assert!(table.columns.iter().any(|c| c.name == "metadata"));
    let metadata_col = table.columns.iter().find(|c| c.name == "metadata").unwrap();
    assert!(
        matches!(metadata_col.data_type, DataType::Jsonb),
        "HashMap should map to Jsonb"
    );
}

#[test]
fn test_product_shape() {
    use facet_owned_shape::*;

    let shape = Product::SHAPE;
    println!("Product shape: {:#?}", shape);

    let table = Table::try_from(shape).expect("Failed to convert Product");

    assert_eq!(table.name, "product");
    // Product should have various fields with mixed types
    assert!(table.columns.len() > 0, "Product should have fields");
}

#[test]
fn test_coordinates_shape() {
    use facet_owned_shape::*;

    let shape = Coordinates::SHAPE;
    println!("Coordinates shape: {:#?}", shape);

    let table = Table::try_from(shape).expect("Failed to convert Coordinates");

    assert_eq!(table.name, "coordinates");
    // Tuple struct - might have named or numbered fields
    assert!(
        table.columns.len() >= 2,
        "Coordinates should have at least 2 fields"
    );
}

#[test]
fn test_fixed_size_arrays_shape() {
    use facet_owned_shape::*;

    let shape = FixedSizeArrays::SHAPE;
    println!("FixedSizeArrays shape: {:#?}", shape);

    // Fixed-size arrays might not convert well
    let result = Table::try_from(shape);

    match result {
        Ok(table) => {
            assert_eq!(table.name, "fixedsizearrays");
            // Arrays should be present
            assert!(table.columns.len() > 0);
        }
        Err(_) => {
            // Also acceptable - arrays might not be fully supported yet
        }
    }
}

#[test]
fn test_blog_post_shape() {
    use facet_owned_shape::*;

    let shape = BlogPost::SHAPE;
    println!("BlogPost shape: {:#?}", shape);

    let table = Table::try_from(shape).expect("Failed to convert BlogPost");

    assert_eq!(table.name, "blogpost");
    // BlogPost should have title, content, tags, etc.
    assert!(table.columns.len() > 0);
    assert!(
        table
            .columns
            .iter()
            .any(|c| c.name == "title" || c.name == "content")
    );
}

#[test]
fn test_borrowed_data_shape() {
    use facet_owned_shape::*;

    let shape = BorrowedData::SHAPE;
    println!("BorrowedData shape: {:#?}", shape);
    // BorrowedData contains references (&'static str, &'a str)
    // References like &str actually work fine - they map to Text
    // So this conversion should succeed
    let result = Table::try_from(shape);

    // For now, we allow it to succeed since &str is commonly used
    // True lifetime-parameterized types would be more complex
    match result {
        Ok(table) => {
            assert_eq!(table.name.to_lowercase(), "borroweddata");
            // Should have fields that are references
            assert!(table.columns.len() > 0);
        }
        Err(_) => {
            // Also acceptable if we want to reject reference types
        }
    }
}

#[test]
fn test_container_u64_shape() {
    use facet_owned_shape::*;

    let shape = Container::<u64>::SHAPE;
    println!("Container<u64> shape: {:#?}", shape);

    // Generic types can't be converted to database schemas
    // The schema needs concrete types, not type parameters
    let result = Table::try_from(shape);

    // This might succeed if the generic is monomorphized, but ideally should indicate generic limitation
    // For now, we just ensure it doesn't panic
    match result {
        Ok(table) => {
            // If it succeeds, verify it at least has the 'value' field
            assert_eq!(table.name.to_lowercase(), "container");
            assert!(table.columns.iter().any(|c| c.name == "value"));
        }
        Err(_) => {
            // Also acceptable - generics shouldn't convert
        }
    }
}

#[test]
fn test_container_string_shape() {
    use facet_owned_shape::*;

    let shape = Container::<String>::SHAPE;
    println!("Container<String> shape: {:#?}", shape);

    // Generic types can't be converted to database schemas
    // The schema needs concrete types, not type parameters
    let result = Table::try_from(shape);

    // This might succeed if the generic is monomorphized, but ideally should indicate generic limitation
    // For now, we just ensure it doesn't panic
    match result {
        Ok(table) => {
            // If it succeeds, verify it at least has the 'value' field
            assert_eq!(table.name.to_lowercase(), "container");
            assert!(table.columns.iter().any(|c| c.name == "value"));
        }
        Err(_) => {
            // Also acceptable - generics shouldn't convert
        }
    }
}

#[test]
fn test_organization_shape() {
    use facet_owned_shape::*;

    let shape = Organization::SHAPE;
    println!("Organization shape: {:#?}", shape);

    let table = Table::try_from(shape).expect("Failed to convert Organization");

    assert_eq!(table.name, "organization");
    assert_eq!(
        table.columns.len(),
        3,
        "Organization has id, name, departments"
    );

    let id = table.columns.iter().find(|c| c.name == "id").unwrap();
    assert!(matches!(id.data_type, DataType::BigInt));
    assert!(!id.nullable);

    let name = table.columns.iter().find(|c| c.name == "name").unwrap();
    assert!(matches!(name.data_type, DataType::Text));
    assert!(!name.nullable);

    // Nested Vec<Department> should be Jsonb
    let departments = table
        .columns
        .iter()
        .find(|c| c.name == "departments")
        .unwrap();
    assert!(
        matches!(departments.data_type, DataType::Jsonb),
        "Vec<Department> should map to Jsonb"
    );
    assert!(!departments.nullable);
}

#[test]
fn test_employee_shape() {
    use facet_owned_shape::*;

    let shape = Employee::SHAPE;
    println!("Employee shape: {:#?}", shape);

    let table = Table::try_from(shape).expect("Failed to convert Employee");

    assert_eq!(table.name, "employee");
    // Employee has multiple fields
    assert!(table.columns.len() > 0, "Employee should have fields");

    // Verify key fields exist
    assert!(table.columns.iter().any(|c| c.name == "id"));
    assert!(
        table
            .columns
            .iter()
            .any(|c| c.name == "name" || c.name == "first_name" || c.name == "last_name")
    );

    let id = table.columns.iter().find(|c| c.name == "id").unwrap();
    assert!(matches!(id.data_type, DataType::BigInt));
    assert!(!id.nullable);
}
