#[test]
fn test_derive_display_case() {
    use crate::query::r#match::ZeroTermsQuery;

    assert_eq!(&ZeroTermsQuery::All.to_string(), "all");
    assert_eq!(&ZeroTermsQuery::None.to_string(), "none");
}
