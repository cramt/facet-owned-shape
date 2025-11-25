use facet::Facet;
use facet_owned_shape::cow_shape::CowShape;
use facet_owned_shape::owned_shape::OwnedShape;

// Test 1: Simple primitive types
#[derive(Facet, Clone)]
struct SimpleStruct {
    id: u32,
    name: String,
    active: bool,
}

#[test]
fn test_round_trip_simple_struct() {
    let cow = CowShape::try_from(SimpleStruct::SHAPE).unwrap();
    let owned: OwnedShape = cow.clone().into();
    let cow2: CowShape = owned.into();

    // Verify structure matches
    assert_eq!(cow.type_identifier, cow2.type_identifier);
    // Deep comparison is hard because of BoxCow/Box structure, but we can check properties
    // or rely on Debug impl if we want to be lazy, but let's trust the types for now.
    // Ideally we should implement PartialEq for CowShape and OwnedShape, but that's out of scope.
    // We can format them and compare strings.
    assert_eq!(format!("{:?}", cow), format!("{:?}", cow2));
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
fn test_round_trip_nested_struct() {
    let cow = CowShape::try_from(Person::SHAPE).unwrap();
    let owned: OwnedShape = cow.clone().into();
    let cow2: CowShape = owned.into();
    assert_eq!(format!("{:?}", cow), format!("{:?}", cow2));
}

// Test 3: Complex enum
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
fn test_round_trip_complex_enum() {
    let cow = CowShape::try_from(HttpStatus::SHAPE).unwrap();
    let owned: OwnedShape = cow.clone().into();
    let cow2: CowShape = owned.into();
    assert_eq!(format!("{:?}", cow), format!("{:?}", cow2));
}

// Test 4: Array of options
#[derive(Facet, Clone)]
struct ArraysOfOptions {
    optional_values: [Option<i32>; 5],
    optional_strings: [Option<String>; 3],
}

#[test]
fn test_round_trip_arrays_of_options() {
    let cow = CowShape::try_from(ArraysOfOptions::SHAPE).unwrap();
    let owned: OwnedShape = cow.clone().into();
    let cow2: CowShape = owned.into();
    assert_eq!(format!("{:?}", cow), format!("{:?}", cow2));
}

// Test 5: Owned to Cow
#[test]
fn test_owned_to_cow() {
    let owned = OwnedShape::try_from(SimpleStruct::SHAPE).unwrap();
    let cow: CowShape = owned.clone().into();
    let owned2: OwnedShape = cow.into();
    assert_eq!(format!("{:?}", owned), format!("{:?}", owned2));
}
