#[test]
fn test_derive_display_case() {
    use crate::sort::Order;

    assert_eq!(&Order::Asc.to_string(), "asc");
    assert_eq!(&Order::Desc.to_string(), "desc");
}
