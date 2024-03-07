#[test]
fn test_derive_display_case() {
    use crate::query::range::Relation;

    assert_eq!(&Relation::Intersects.to_string(), "INTERSECTS");
    assert_eq!(&Relation::Contains.to_string(), "CONTAINS");
    assert_eq!(&Relation::Within.to_string(), "WITHIN");
}

#[test]
fn test_default() {
    use crate::query::range::Relation;

    assert_eq!(Relation::default(), Relation::Intersects);
}
