use facet::Facet;
use facet_psql_schema::{DataType, PartialSchema};

#[repr(u8)]
#[derive(Facet)]
enum Thingy {
    A { a: usize },
    B { b: usize },
}

#[test]
fn test_enum_to_schema_thingy() {
    let shape = Thingy::SHAPE;
    let schema = PartialSchema::try_from(shape).expect("Failed to convert Thingy enum");

    // Expect 3 tables: thingy, thingy_a, thingy_b
    assert_eq!(schema.tables.len(), 3);

    // 1. Verify Main Table
    let main_table = schema
        .tables
        .iter()
        .find(|t| t.name == "thingy")
        .expect("Main table 'thingy' not found");

    // Columns: id, discriminant, a_id, b_id
    assert_eq!(main_table.columns.len(), 4);
    assert!(main_table.columns.iter().any(|c| c.name == "id"));
    assert!(main_table.columns.iter().any(|c| c.name == "discriminant"));
    assert!(
        main_table
            .columns
            .iter()
            .any(|c| c.name == "a_id" && matches!(c.data_type, DataType::BigInt) && c.nullable)
    );
    assert!(
        main_table
            .columns
            .iter()
            .any(|c| c.name == "b_id" && matches!(c.data_type, DataType::BigInt) && c.nullable)
    );

    // Foreign Keys
    assert_eq!(main_table.foreign_keys.len(), 2);
    let fk_a = main_table
        .foreign_keys
        .iter()
        .find(|fk| fk.columns == vec!["a_id"])
        .expect("FK for a_id missing");
    assert_eq!(fk_a.referenced_table.name, "thingy_a");

    let fk_b = main_table
        .foreign_keys
        .iter()
        .find(|fk| fk.columns == vec!["b_id"])
        .expect("FK for b_id missing");
    assert_eq!(fk_b.referenced_table.name, "thingy_b");

    // Check Constraint
    assert_eq!(main_table.checks.len(), 1);
    let check = &main_table.checks[0];
    // We don't strictly assert the expression string as it formats differently, but we can print it
    println!("Check constraint: {}", check.expression);
    // (CASE WHEN discriminant = 0 THEN a_id IS NOT NULL ELSE a_id IS NULL END) AND (CASE WHEN discriminant = 1 THEN b_id IS NOT NULL ELSE b_id IS NULL END)
    assert!(check.expression.contains("discriminant = 0"));
    assert!(check.expression.contains("a_id IS NOT NULL"));

    // 2. Verify Variant Tables
    let table_a = schema
        .tables
        .iter()
        .find(|t| t.name == "thingy_a")
        .expect("Variant table 'thingy_a' not found");
    // Columns: id, a
    assert!(table_a.columns.iter().any(|c| c.name == "id"));
    assert!(table_a.columns.iter().any(|c| c.name == "a")); // usize -> BigInt

    let table_b = schema
        .tables
        .iter()
        .find(|t| t.name == "thingy_b")
        .expect("Variant table 'thingy_b' not found");
    // Columns: id, b
    assert!(table_b.columns.iter().any(|c| c.name == "id"));
    assert!(table_b.columns.iter().any(|c| c.name == "b"));
}
