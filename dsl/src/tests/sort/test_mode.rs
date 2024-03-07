#[test]
fn test_derive_display_case() {
    use crate::sort::Mode;

    assert_eq!(&Mode::Min.to_string(), "min");
    assert_eq!(&Mode::Max.to_string(), "max");
    assert_eq!(&Mode::Sum.to_string(), "sum");
    assert_eq!(&Mode::Avg.to_string(), "avg");
    assert_eq!(&Mode::Median.to_string(), "median");
}
