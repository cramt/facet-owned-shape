use facet::Facet;
use facet_owned_shape::cow_shape::CowShape;

// Test 1: Simple primitive types
#[derive(Facet, Clone)]
struct SimpleStruct {
    id: u32,
    name: String,
    active: bool,
}

#[test]
fn test_simple_struct() {
    let cow = CowShape::try_from(SimpleStruct::SHAPE);
    assert!(
        cow.is_ok(),
        "Failed to convert SimpleStruct: {:?}",
        cow.err()
    );
    let cow = cow.unwrap();
    assert_eq!(cow.type_identifier, "SimpleStruct");
}

// Test 2: Nested struct
#[derive(Facet, Clone)]
struct Address {
    street: String,
    city: String,
    zip_code: u32,
}

#[derive(Facet, Clone)]
struct Person {
    name: String,
    age: u8,
    address: Address,
}

#[test]
fn test_nested_struct() {
    let cow = CowShape::try_from(Person::SHAPE);
    assert!(cow.is_ok(), "Failed to convert Person: {:?}", cow.err());
}

// Test 3: Struct with Option
#[derive(Facet, Clone)]
struct OptionalFields {
    required: String,
    optional_number: Option<i32>,
    optional_text: Option<String>,
}

#[test]
fn test_optional_fields() {
    let cow = CowShape::try_from(OptionalFields::SHAPE);
    assert!(
        cow.is_ok(),
        "Failed to convert OptionalFields: {:?}",
        cow.err()
    );
}

// Test 4: Array types
#[derive(Facet, Clone)]
struct ArrayStruct {
    coords: [f64; 3],
    matrix: [i32; 9],
}

#[test]
fn test_arrays() {
    let cow = CowShape::try_from(ArrayStruct::SHAPE);
    assert!(
        cow.is_ok(),
        "Failed to convert ArrayStruct: {:?}",
        cow.err()
    );
}

// Test 5: All primitive types
#[derive(Facet, Clone)]
struct AllPrimitives {
    bool_val: bool,
    u8_val: u8,
    u16_val: u16,
    u32_val: u32,
    u64_val: u64,
    i8_val: i8,
    i16_val: i16,
    i32_val: i32,
    i64_val: i64,
    f32_val: f32,
    f64_val: f64,
    char_val: char,
    string_val: String,
}

#[test]
fn test_all_primitives() {
    let cow = CowShape::try_from(AllPrimitives::SHAPE);
    assert!(
        cow.is_ok(),
        "Failed to convert AllPrimitives: {:?}",
        cow.err()
    );
}

// Test 6: Empty struct
#[derive(Facet, Clone)]
struct EmptyStruct {}

#[test]
fn test_empty_struct() {
    let cow = CowShape::try_from(EmptyStruct::SHAPE);
    assert!(
        cow.is_ok(),
        "Failed to convert EmptyStruct: {:?}",
        cow.err()
    );
}

// Test 7: Simple enum
#[derive(Facet, Clone)]
#[repr(C)]
enum SimpleEnum {
    VariantA,
    VariantB { value: u32 },
    VariantC { x: f64, y: f64 },
}

#[test]
fn test_simple_enum() {
    let cow = CowShape::try_from(SimpleEnum::SHAPE);
    assert!(cow.is_ok(), "Failed to convert SimpleEnum: {:?}", cow.err());
}

// Test 8: Nested optionals
#[derive(Facet, Clone)]
struct NestedOptions {
    maybe_int: Option<Option<i32>>,
    maybe_string: Option<Option<String>>,
}

#[test]
fn test_nested_options() {
    let cow = CowShape::try_from(NestedOptions::SHAPE);
    assert!(
        cow.is_ok(),
        "Failed to convert NestedOptions: {:?}",
        cow.err()
    );
}

// Test 9: Multiple nestings
#[derive(Facet, Clone)]
struct Coordinate {
    x: f64,
    y: f64,
}

#[derive(Facet, Clone)]
struct Point3D {
    coord: Coordinate,
    z: f64,
}

#[derive(Facet, Clone)]
struct Location {
    point: Point3D,
    name: String,
}

#[test]
fn test_deep_nesting() {
    let cow = CowShape::try_from(Location::SHAPE);
    assert!(cow.is_ok(), "Failed to convert Location: {:?}", cow.err());
}

// Test 10: Array of arrays
#[derive(Facet, Clone)]
struct NestedArrays {
    matrix_2x3: [[f64; 3]; 2],
    cube_3x3x3: [[[u8; 3]; 3]; 3],
}

#[test]
fn test_nested_arrays() {
    let cow = CowShape::try_from(NestedArrays::SHAPE);
    assert!(
        cow.is_ok(),
        "Failed to convert NestedArrays: {:?}",
        cow.err()
    );
}

// Test 11: Optional arrays
#[derive(Facet, Clone)]
struct OptionalArrays {
    maybe_coords: Option<[f64; 3]>,
    maybe_matrix: Option<[i32; 4]>,
}

#[test]
fn test_optional_arrays() {
    let cow = CowShape::try_from(OptionalArrays::SHAPE);
    assert!(
        cow.is_ok(),
        "Failed to convert OptionalArrays: {:?}",
        cow.err()
    );
}

// Test 12: Arrays of options
#[derive(Facet, Clone)]
struct ArraysOfOptions {
    optional_values: [Option<i32>; 5],
    optional_strings: [Option<String>; 3],
}

#[test]
fn test_arrays_of_options() {
    let cow = CowShape::try_from(ArraysOfOptions::SHAPE);
    assert!(
        cow.is_ok(),
        "Failed to convert ArraysOfOptions: {:?}",
        cow.err()
    );
}

// Test 13: Struct with all signed integers
#[derive(Facet, Clone)]
struct SignedIntegers {
    i8_val: i8,
    i16_val: i16,
    i32_val: i32,
    i64_val: i64,
    i128_val: i128,
    isize_val: isize,
}

#[test]
fn test_signed_integers() {
    let cow = CowShape::try_from(SignedIntegers::SHAPE);
    assert!(
        cow.is_ok(),
        "Failed to convert SignedIntegers: {:?}",
        cow.err()
    );
}

// Test 14: Struct with all unsigned integers
#[derive(Facet, Clone)]
struct UnsignedIntegers {
    u8_val: u8,
    u16_val: u16,
    u32_val: u32,
    u64_val: u64,
    u128_val: u128,
    usize_val: usize,
}

#[test]
fn test_unsigned_integers() {
    let cow = CowShape::try_from(UnsignedIntegers::SHAPE);
    assert!(
        cow.is_ok(),
        "Failed to convert UnsignedIntegers: {:?}",
        cow.err()
    );
}

// Test 15: Floats
#[derive(Facet, Clone)]
struct FloatTypes {
    f32_val: f32,
    f64_val: f64,
}

#[test]
fn test_float_types() {
    let cow = CowShape::try_from(FloatTypes::SHAPE);
    assert!(cow.is_ok(), "Failed to convert FloatTypes: {:?}", cow.err());
}

// Test 16: Textual types
#[derive(Facet, Clone)]
struct TextTypes {
    char_val: char,
    string_val: String,
}

#[test]
fn test_text_types() {
    let cow = CowShape::try_from(TextTypes::SHAPE);
    assert!(cow.is_ok(), "Failed to convert TextTypes: {:?}", cow.err());
}

// Test 17: Complex enum with multiple field variants
#[derive(Facet, Clone)]
#[repr(C)]
enum HttpStatus {
    Ok,
    Created { location: String },
    BadRequest { message: String },
    NotFound { path: String },
    InternalError { error: String, code: u32 },
}

#[test]
fn test_complex_enum() {
    let cow = CowShape::try_from(HttpStatus::SHAPE);
    assert!(cow.is_ok(), "Failed to convert HttpStatus: {:?}", cow.err());
}

// Test 18: Nested struct in optional
#[derive(Facet, Clone)]
struct Config {
    timeout: u32,
    retries: u8,
}

#[derive(Facet, Clone)]
struct Service {
    name: String,
    config: Option<Config>,
}

#[test]
fn test_optional_nested_struct() {
    let cow = CowShape::try_from(Service::SHAPE);
    assert!(cow.is_ok(), "Failed to convert Service: {:?}", cow.err());
}

// Test 19: Enum with nested struct variant
#[derive(Facet, Clone)]
struct ErrorDetails {
    message: String,
    code: u32,
}

#[derive(Facet, Clone)]
#[repr(C)]
enum ResultEnum {
    Success { value: String },
    Failure { error: ErrorDetails },
}

#[test]
fn test_enum_with_nested_struct() {
    let cow = CowShape::try_from(ResultEnum::SHAPE);
    assert!(cow.is_ok(), "Failed to convert ResultEnum: {:?}", cow.err());
}

// Test 20: Database entities
#[derive(Facet, Clone)]
struct User {
    id: u64,
    username: String,
    email: String,
    created_at: u64,
    is_active: bool,
}

#[test]
fn test_user_struct() {
    let cow = CowShape::try_from(User::SHAPE);
    assert!(cow.is_ok(), "Failed to convert User: {:?}", cow.err());
}

// Test 21: Complex nested structure
#[derive(Facet, Clone)]
struct Metadata {
    version: u32,
    timestamp: u64,
}

#[derive(Facet, Clone)]
struct DataRecord {
    id: String,
    data: [u8; 32],
    metadata: Metadata,
}

#[test]
fn test_data_record() {
    let cow = CowShape::try_from(DataRecord::SHAPE);
    assert!(cow.is_ok(), "Failed to convert DataRecord: {:?}", cow.err());
}

// Test 22: Multiple optional fields
#[derive(Facet, Clone)]
struct FormData {
    required_field: String,
    optional_name: Option<String>,
    optional_age: Option<u8>,
    optional_email: Option<String>,
    optional_phone: Option<String>,
}

#[test]
fn test_form_data() {
    let cow = CowShape::try_from(FormData::SHAPE);
    assert!(cow.is_ok(), "Failed to convert FormData: {:?}", cow.err());
}

// Test 23: Arrays with different sizes
#[derive(Facet, Clone)]
struct VariousSizedArrays {
    single: [u8; 1],
    pair: [u16; 2],
    quad: [u32; 4],
    octet: [u64; 8],
    big: [i32; 16],
}

#[test]
fn test_various_sized_arrays() {
    let cow = CowShape::try_from(VariousSizedArrays::SHAPE);
    assert!(
        cow.is_ok(),
        "Failed to convert VariousSizedArrays: {:?}",
        cow.err()
    );
}

// Test 24: Mixed primitive types in struct
#[derive(Facet, Clone)]
struct MixedPrimitives {
    small_int: i8,
    big_int: i64,
    small_float: f32,
    big_float: f64,
    text: String,
    letter: char,
    flag: bool,
}

#[test]
fn test_mixed_primitives() {
    let cow = CowShape::try_from(MixedPrimitives::SHAPE);
    assert!(
        cow.is_ok(),
        "Failed to convert MixedPrimitives: {:?}",
        cow.err()
    );
}

// Test 25: Large array
#[derive(Facet, Clone)]
struct LargeArrayStruct {
    buffer: [u8; 256],
    small: [i32; 10],
}

#[test]
fn test_large_array() {
    let cow = CowShape::try_from(LargeArrayStruct::SHAPE);
    assert!(
        cow.is_ok(),
        "Failed to convert LargeArrayStruct: {:?}",
        cow.err()
    );
}
