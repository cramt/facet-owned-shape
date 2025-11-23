use facet::Facet;
use facet_owned_shape::{diff::Diff, OwnedShape};

// Test 1: Equal shapes should be detected
#[derive(Facet, Clone, Debug)]
struct SimpleStruct {
    id: u32,
    name: String,
}

#[test]
fn test_equal_shapes() {
    let shape1 = OwnedShape::try_from(SimpleStruct::SHAPE).unwrap();
    let shape2 = OwnedShape::try_from(SimpleStruct::SHAPE).unwrap();
    
    let diff = Diff::new(&shape1, &shape2);
    assert!(diff.is_equal(), "Same shapes should be equal");
}

// Test 2: Different primitive types
#[derive(Facet, Clone, Debug)]
struct IntStruct {
    value: i32,
}

#[derive(Facet, Clone, Debug)]
struct StringStruct {
    value: String,
}

#[test]
fn test_different_primitive_types() {
    let shape1 = OwnedShape::try_from(IntStruct::SHAPE).unwrap();
    let shape2 = OwnedShape::try_from(StringStruct::SHAPE).unwrap();
    
    let diff = Diff::new(&shape1, &shape2);
    assert!(!diff.is_equal(), "Different primitive types should not be equal");
}

// Test 3: Struct field additions
#[derive(Facet, Clone, Debug)]
struct PersonV1 {
    name: String,
    age: u32,
}

#[derive(Facet, Clone, Debug)]
struct PersonV2 {
    name: String,
    age: u32,
    email: String,
}

#[test]
fn test_field_addition() {
    let shape1 = OwnedShape::try_from(PersonV1::SHAPE).unwrap();
    let shape2 = OwnedShape::try_from(PersonV2::SHAPE).unwrap();
    
    let diff = Diff::new(&shape1, &shape2);
    assert!(!diff.is_equal(), "Shapes with different fields should not be equal");
    
    // Check that it's a User diff with struct value
    match diff {
        Diff::User { value: facet_owned_shape::diff::Value::Struct { insertions, unchanged, .. }, .. } => {
            assert_eq!(insertions.len(), 1, "Should have one insertion");
            assert!(insertions.contains("email"), "Should have email as insertion");
            assert_eq!(unchanged.len(), 2, "Should have two unchanged fields");
        }
        _ => panic!("Expected User diff with Struct value"),
    }
}

// Test 4: Struct field deletions
#[test]
fn test_field_deletion() {
    let shape1 = OwnedShape::try_from(PersonV2::SHAPE).unwrap();
    let shape2 = OwnedShape::try_from(PersonV1::SHAPE).unwrap();
    
    let diff = Diff::new(&shape1, &shape2);
    assert!(!diff.is_equal(), "Shapes with different fields should not be equal");
    
    match diff {
        Diff::User { value: facet_owned_shape::diff::Value::Struct { deletions, unchanged, .. }, .. } => {
            assert_eq!(deletions.len(), 1, "Should have one deletion");
            assert!(deletions.contains("email"), "Should have email as deletion");
            assert_eq!(unchanged.len(), 2, "Should have two unchanged fields");
        }
        _ => panic!("Expected User diff with Struct value"),
    }
}

// Test 5: Field type changes
#[derive(Facet, Clone, Debug)]
struct ConfigV1 {
    port: u16,
    host: String,
}

#[derive(Facet, Clone, Debug)]
struct ConfigV2 {
    port: String,  // Changed from u16 to String
    host: String,
}

#[test]
fn test_field_type_change() {
    let shape1 = OwnedShape::try_from(ConfigV1::SHAPE).unwrap();
    let shape2 = OwnedShape::try_from(ConfigV2::SHAPE).unwrap();
    
    let diff = Diff::new(&shape1, &shape2);
    assert!(!diff.is_equal(), "Shapes with different field types should not be equal");
    
    match diff {
        Diff::User { value: facet_owned_shape::diff::Value::Struct { updates, .. }, .. } => {
            assert_eq!(updates.len(), 1, "Should have one update");
            assert!(updates.contains_key("port"), "Should have port as updated");
        }
        _ => panic!("Expected User diff with Struct value"),
    }
}

// Test 6: Nested struct changes
#[derive(Facet, Clone, Debug)]
struct Address {
    street: String,
    city: String,
}

#[derive(Facet, Clone, Debug)]
struct PersonWithAddress {
    name: String,
    address: Address,
}

#[derive(Facet, Clone, Debug)]
struct SimpleAddress {
    city: String,
}

#[derive(Facet, Clone, Debug)]
struct PersonWithSimpleAddress {
    name: String,
    address: SimpleAddress,
}

#[test]
fn test_nested_struct_change() {
    let shape1 = OwnedShape::try_from(PersonWithAddress::SHAPE).unwrap();
    let shape2 = OwnedShape::try_from(PersonWithSimpleAddress::SHAPE).unwrap();
    
    let diff = Diff::new(&shape1, &shape2);
    assert!(!diff.is_equal(), "Nested struct changes should be detected");
    
    match diff {
        Diff::User { value: facet_owned_shape::diff::Value::Struct { updates, .. }, .. } => {
            assert!(updates.contains_key("address"), "Should have address as updated");
        }
        _ => panic!("Expected User diff with Struct value"),
    }
}

// Test 7: Array size changes
#[derive(Facet, Clone, Debug)]
struct SmallArray {
    data: [u8; 10],
}

#[derive(Facet, Clone, Debug)]
struct LargeArray {
    data: [u8; 20],
}

#[test]
fn test_array_size_change() {
    let shape1 = OwnedShape::try_from(SmallArray::SHAPE).unwrap();
    let shape2 = OwnedShape::try_from(LargeArray::SHAPE).unwrap();
    
    let diff = Diff::new(&shape1, &shape2);
    assert!(!diff.is_equal(), "Different array sizes should not be equal");
}

// Test 8: Option type changes
#[derive(Facet, Clone, Debug)]
struct RequiredField {
    value: String,
}

#[derive(Facet, Clone, Debug)]
struct OptionalField {
    value: Option<String>,
}

#[test]
fn test_option_type_change() {
    let shape1 = OwnedShape::try_from(RequiredField::SHAPE).unwrap();
    let shape2 = OwnedShape::try_from(OptionalField::SHAPE).unwrap();
    
    let diff = Diff::new(&shape1, &shape2);
    assert!(!diff.is_equal(), "Required vs optional should not be equal");
}

// Test 9: Enum shapes
#[derive(Facet, Clone, Debug)]
#[repr(C)]
enum StatusV1 {
    Active,
    Inactive,
}

#[derive(Facet, Clone, Debug)]
#[repr(C)]
enum StatusV2 {
    Active,
    Inactive,
    Pending,
}

#[test]
fn test_enum_shapes() {
    let shape1 = OwnedShape::try_from(StatusV1::SHAPE).unwrap();
    let shape2 = OwnedShape::try_from(StatusV2::SHAPE).unwrap();
    
    let diff = Diff::new(&shape1, &shape2);
    assert!(!diff.is_equal(), "Enums with different variants should not be equal");
}

// Test 10: Complex nested structures
#[derive(Facet, Clone, Debug)]
struct Metadata {
    version: u32,
    timestamp: u64,
}

#[derive(Facet, Clone, Debug)]
struct DataRecordV1 {
    id: String,
    data: [u8; 32],
    metadata: Metadata,
}

#[derive(Facet, Clone, Debug)]
struct DataRecordV2 {
    id: String,
    data: [u8; 64],  // Changed size
    metadata: Metadata,
}

#[test]
fn test_complex_nested_change() {
    let shape1 = OwnedShape::try_from(DataRecordV1::SHAPE).unwrap();
    let shape2 = OwnedShape::try_from(DataRecordV2::SHAPE).unwrap();
    
    let diff = Diff::new(&shape1, &shape2);
    assert!(!diff.is_equal(), "Complex nested changes should be detected");
}

// Test 11: Identical enum shapes
#[derive(Facet, Clone, Debug)]
#[repr(C)]
enum Color {
    Red,
    Green,
    Blue,
}

#[test]
fn test_equal_enum_shapes() {
    let shape1 = OwnedShape::try_from(Color::SHAPE).unwrap();
    let shape2 = OwnedShape::try_from(Color::SHAPE).unwrap();
    
    let diff = Diff::new(&shape1, &shape2);
    assert!(diff.is_equal(), "Same enum shapes should be equal");
}

// Test 12: All fields unchanged
#[derive(Facet, Clone, Debug)]
struct Unchanged {
    a: u32,
    b: String,
    c: bool,
}

#[test]
fn test_all_fields_unchanged() {
    let shape1 = OwnedShape::try_from(Unchanged::SHAPE).unwrap();
    let shape2 = OwnedShape::try_from(Unchanged::SHAPE).unwrap();
    
    let diff = Diff::new(&shape1, &shape2);
    assert!(diff.is_equal(), "Identical structs should be equal");
}

// Test 13: Multiple field updates
#[derive(Facet, Clone, Debug)]
struct MultiFieldV1 {
    field1: u32,
    field2: String,
    field3: bool,
}

#[derive(Facet, Clone, Debug)]
struct MultiFieldV2 {
    field1: u64,     // Changed type
    field2: String,  // Unchanged
    field3: u8,      // Changed type
}

#[test]
fn test_multiple_field_updates() {
    let shape1 = OwnedShape::try_from(MultiFieldV1::SHAPE).unwrap();
    let shape2 = OwnedShape::try_from(MultiFieldV2::SHAPE).unwrap();
    
    let diff = Diff::new(&shape1, &shape2);
    assert!(!diff.is_equal(), "Multiple field changes should be detected");
    
    match diff {
        Diff::User { value: facet_owned_shape::diff::Value::Struct { updates, unchanged, .. }, .. } => {
            assert_eq!(updates.len(), 2, "Should have two updates");
            assert!(updates.contains_key("field1"), "field1 should be updated");
            assert!(updates.contains_key("field3"), "field3 should be updated");
            assert_eq!(unchanged.len(), 1, "Should have one unchanged field");
            assert!(unchanged.contains("field2"), "field2 should be unchanged");
        }
        _ => panic!("Expected User diff with Struct value"),
    }
}

// Test 14: Empty structs
#[derive(Facet, Clone, Debug)]
struct EmptyStruct1 {}

#[derive(Facet, Clone, Debug)]
struct EmptyStruct2 {}

#[test]
fn test_empty_structs() {
    let shape1 = OwnedShape::try_from(EmptyStruct1::SHAPE).unwrap();
    let shape2 = OwnedShape::try_from(EmptyStruct2::SHAPE).unwrap();
    
    let diff = Diff::new(&shape1, &shape2);
    // Empty structs with different names are different
    assert!(!diff.is_equal(), "Empty structs with different type identifiers should not be equal");
}
