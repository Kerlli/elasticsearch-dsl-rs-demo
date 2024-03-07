#[test]
fn test_derive_display_case() {
    use crate::sort::NumbericType;

    assert_eq!(&NumbericType::Double.to_string(), "double");
    assert_eq!(&NumbericType::Long.to_string(), "long");
    assert_eq!(&NumbericType::Date.to_string(), "date");
    assert_eq!(&NumbericType::DateNanos.to_string(), "date_nanos");
}
