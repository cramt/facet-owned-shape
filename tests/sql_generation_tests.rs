use facet::Facet;
use facet_psql_schema as psql;
use facet_psql_schema::*;

#[allow(dead_code)]
#[derive(Facet)]
struct TableB {
    #[facet(psql::primary_key)]
    id: i64,
}

#[allow(dead_code)]
#[derive(Facet)]
struct TableA {
    #[facet(psql::primary_key)]
    id: i64,
    b_id: Option<i64>,
}

#[test]
fn test_sql_generation_foreign_key_ordering() {
    // Generate schemas from Facet derived shapes
    let schema_a = PartialSchema::try_from(TableA::SHAPE).expect("Failed to convert TableA");
    let schema_b = PartialSchema::try_from(TableB::SHAPE).expect("Failed to convert TableB");

    let mut table_a = schema_a.tables.into_iter().next().unwrap();
    let table_b = schema_b.tables.into_iter().next().unwrap();

    // Manually add Foreign Key to Table A (since not yet derivable)
    table_a.foreign_keys.push(ForeignKey {
        name: None,
        columns: vec!["b_id".into()],
        referenced_table: QualifiedName {
            schema: None,
            name: "tableb".into(), // snake_case of TableB
        },
        referenced_columns: Some(vec!["id".into()]),
        on_delete: Some(ReferentialAction::Cascade),
        on_update: None,
        match_type: None,
        deferrable: None,
        initially: None,
    });

    // Combine into one schema
    // Put A before B to test ordering logic
    let schema = PartialSchema {
        tables: vec![table_a, table_b],
        views: vec![],
        materialized_views: vec![],
        enums: vec![],
        domains: vec![],
        composite_types: vec![],
        sequences: vec![],
        collations: vec![],
        functions: vec![],
    };

    let sql = schema.to_ddl("public");
    println!("{}", sql);

    let create_a_idx = sql
        .find("CREATE TABLE public.tablea")
        .expect("Missing CREATE TABLE tablea");
    let create_b_idx = sql
        .find("CREATE TABLE public.tableb")
        .expect("Missing CREATE TABLE tableb");
    let alter_fk_idx = sql.find("ALTER TABLE public.tablea ADD CONSTRAINT tablea_b_id_fkey FOREIGN KEY (b_id) REFERENCES tableb (id)").expect("Missing FK constraint");

    assert!(
        alter_fk_idx > create_a_idx,
        "FK creation should come after table A creation"
    );
    assert!(
        alter_fk_idx > create_b_idx,
        "FK creation should come after table B creation"
    );
}

#[allow(dead_code)]
#[derive(Facet)]
struct Users {
    email: String,
}

#[test]
fn test_sql_generation_indexes() {
    let schema = PartialSchema::try_from(Users::SHAPE).expect("Failed to convert Users");
    let mut table = schema.tables.into_iter().next().unwrap();

    // Manually add Index (since not yet derivable)
    table.indexes.push(Index {
        name: "idx_users_email".into(),
        columns: vec![IndexColumn {
            expr: IndexExpr::Column("email".into()),
            collate: None,
            opclass: None,
            order: Some(SortOrder::Desc),
            nulls_order: Some(NullsOrder::Last),
        }],
        unique: true,
        method: Some("btree".into()),
        predicate: Some("email IS NOT NULL".into()),
        include: vec![],
        tablespace: None,
        concurrently: false,
        is_primary: false,
        is_valid: true,
    });

    let schema = PartialSchema {
        tables: vec![table],
        views: vec![],
        materialized_views: vec![],
        enums: vec![],
        domains: vec![],
        composite_types: vec![],
        sequences: vec![],
        collations: vec![],
        functions: vec![],
    };

    let sql = schema.to_ddl("public");
    println!("{}", sql);

    // Expected: CREATE UNIQUE INDEX idx_users_email ON public.users USING btree (email DESC NULLS LAST) WHERE email IS NOT NULL;
    assert!(sql.contains("CREATE UNIQUE INDEX idx_users_email ON public.users USING btree (email DESC NULLS LAST) WHERE email IS NOT NULL;"));
}
