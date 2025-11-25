use facet::Facet;
use facet_owned_shape::{diff::Diff, owned_shape::OwnedShape};
use sea_query::{PostgresQueryBuilder, TableAlterStatement, TableCreateStatement};

#[derive(Facet, Clone)]
struct User {
    id: i32,
    username: String,
    active: bool,
    age: Option<i32>,
}

#[test]
fn test_user_table_creation() {
    let shape = OwnedShape::try_from(User::SHAPE).unwrap();
    let stmt: TableCreateStatement = shape.try_into().unwrap();
    let sql = stmt.to_string(PostgresQueryBuilder);

    assert!(sql.contains("CREATE TABLE \"User\""));
    assert!(sql.contains("\"id\" integer NOT NULL"));
    assert!(sql.contains("\"username\" varchar"));
    assert!(sql.contains("\"active\" bool NOT NULL"));
    assert!(sql.contains("\"age\" integer"));
}

#[derive(Facet, Clone)]
struct AllTypes {
    f1: u8,
    f2: u16,
    f3: u32,
    f4: u64,
    f5: i8,
    f6: i16,
    f7: i32,
    f8: i64,
    f9: f32,
    f10: f64,
    f11: bool,
    f12: String,
    f13: char,
}

#[test]
fn test_all_types() {
    let shape = OwnedShape::try_from(AllTypes::SHAPE).unwrap();
    let stmt: TableCreateStatement = shape.try_into().unwrap();
    let sql = stmt.to_string(PostgresQueryBuilder);

    assert!(sql.contains("\"f1\" smallint NOT NULL"));
    assert!(sql.contains("\"f3\" integer NOT NULL"));
    assert!(sql.contains("\"f4\" bigint NOT NULL"));
    assert!(sql.contains("\"f9\" real NOT NULL"));
    assert!(sql.contains("\"f10\" double precision NOT NULL"));
}

#[test]
fn test_non_struct_failure() {
    let shape = OwnedShape::try_from(u32::SHAPE).unwrap();
    let res: Result<TableCreateStatement, String> = shape.try_into();
    assert!(res.is_err());
}

#[derive(Facet, Clone)]
struct UserV1 {
    id: i32,
    username: String,
}

#[derive(Facet, Clone)]
struct UserV2 {
    id: i32,
    username: String,
    email: String,
    created_at: Option<i64>,
}

#[test]
fn test_diff_add_columns() {
    let v1 = OwnedShape::try_from(UserV1::SHAPE).unwrap();
    let v2 = OwnedShape::try_from(UserV2::SHAPE).unwrap();

    let diff = Diff::new(&v1.into(), &v2.into());
    let stmt: TableAlterStatement = diff.try_into().unwrap();
    let sql = stmt.to_string(PostgresQueryBuilder);

    assert!(sql.contains("ALTER TABLE \"UserV2\""));
    assert!(sql.contains("ADD COLUMN \"email\" varchar NOT NULL"));
    assert!(sql.contains("ADD COLUMN \"created_at\" bigint"));
}

#[derive(Facet, Clone)]
struct UserV3 {
    id: i32,
}

#[test]
fn test_diff_drop_columns() {
    let v1 = OwnedShape::try_from(UserV1::SHAPE).unwrap();
    let v3 = OwnedShape::try_from(UserV3::SHAPE).unwrap();

    let diff = Diff::new(&v1.into(), &v3.into());
    let stmt: TableAlterStatement = diff.try_into().unwrap();
    let sql = stmt.to_string(PostgresQueryBuilder);

    assert!(sql.contains("ALTER TABLE \"UserV3\""));
    assert!(sql.contains("DROP COLUMN \"username\""));
}

#[derive(Facet, Clone)]
struct UserV4 {
    id: i32,
    username: String,
    status: bool,
}

#[test]
fn test_diff_add_and_drop() {
    let v1 = OwnedShape::try_from(UserV1::SHAPE).unwrap();
    let v4 = OwnedShape::try_from(UserV4::SHAPE).unwrap();

    let diff = Diff::new(&v1.into(), &v4.into());
    let stmt: TableAlterStatement = diff.try_into().unwrap();
    let sql = stmt.to_string(PostgresQueryBuilder);

    assert!(sql.contains("ALTER TABLE \"UserV4\""));
    assert!(sql.contains("ADD COLUMN \"status\" bool NOT NULL"));
}

#[derive(Facet, Clone)]
struct UserV5 {
    id: String,
    username: String,
}

#[test]
fn test_diff_type_change_i32_to_string() {
    let v1 = OwnedShape::try_from(UserV1::SHAPE).unwrap();
    let v5 = OwnedShape::try_from(UserV5::SHAPE).unwrap();

    let diff = Diff::new(&v1.into(), &v5.into());
    let stmt: TableAlterStatement = diff.try_into().unwrap();
    let sql = stmt.to_string(PostgresQueryBuilder);

    assert!(sql.contains("ALTER TABLE \"UserV5\""));
    assert!(sql.contains("ALTER COLUMN \"id\"") || sql.contains("MODIFY COLUMN \"id\""));
}

#[derive(Facet, Clone)]
struct UserV6 {
    id: i64,
    username: String,
}

#[test]
fn test_diff_type_change_i32_to_i64() {
    let v1 = OwnedShape::try_from(UserV1::SHAPE).unwrap();
    let v6 = OwnedShape::try_from(UserV6::SHAPE).unwrap();

    let diff = Diff::new(&v1.into(), &v6.into());
    let stmt: TableAlterStatement = diff.try_into().unwrap();
    let sql = stmt.to_string(PostgresQueryBuilder);

    assert!(sql.contains("ALTER TABLE \"UserV6\""));
    assert!(sql.contains("bigint"));
}

#[derive(Facet, Clone)]
struct UserV7 {
    id: i32,
    username: bool,
}

#[test]
fn test_diff_incompatible_type_change() {
    let v1 = OwnedShape::try_from(UserV1::SHAPE).unwrap();
    let v7 = OwnedShape::try_from(UserV7::SHAPE).unwrap();

    let diff = Diff::new(&v1.into(), &v7.into());
    let res: Result<TableAlterStatement, String> = diff.try_into();

    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Incompatible type change"));
}

#[test]
fn test_diff_equal_error() {
    let v1 = OwnedShape::try_from(UserV1::SHAPE).unwrap();
    let v1_copy = OwnedShape::try_from(UserV1::SHAPE).unwrap();

    let diff = Diff::new(&v1.into(), &v1_copy.into());
    let res: Result<TableAlterStatement, String> = diff.try_into();

    assert!(res.is_err());
    assert!(res.unwrap_err().contains("no changes needed"));
}
