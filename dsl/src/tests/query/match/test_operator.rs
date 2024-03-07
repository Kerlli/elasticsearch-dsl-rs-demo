#[test]
fn test_derive_display_case() {
    use crate::query::r#match::Operator;

    assert_eq!(&Operator::And.to_string(), "AND");
    assert_eq!(&Operator::Or.to_string(), "OR");
}
