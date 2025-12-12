use facet::Facet;
use facet_psql_schema as psql;
use facet_psql_schema::{ConversionError, Table};

#[derive(Facet)]
struct DoublePk {
    #[facet(psql::primary_key)]
    id1: u64,
    #[facet(psql::primary_key)]
    id2: u64,
}

#[test]
fn test_double_pk_fails() {
    let shape = DoublePk::SHAPE;
    let result = Table::try_from(shape);

    assert!(result.is_err(), "Expected error for multiple primary keys");
    match result {
        Err(ConversionError::MultiplePrimaryKeys(_)) => (), // Expected
        Err(e) => panic!("Expected MultiplePrimaryKeys error, got: {:?}", e),
        Ok(_) => panic!("Expected error, got Ok"),
    }
}
