#[test]
fn test_serialize() {
    use crate::query::wildcard::Wildcard;

    let w = Wildcard::new("user.id", "ki*y", "wildcard");

    let w0 = serde_json::to_string(&w).unwrap();

    assert_eq!(w0, r#"{"user.id":{"value":"ki*y","wildcard":"wildcard"}}"#);

    let w1 = serde_json::to_string(
        w.clone()
            .rewrite("rewrite")
    )
    .unwrap();

    assert_eq!(w1, r#"{"user.id":{"rewrite":"rewrite","value":"ki*y","wildcard":"wildcard"}}"#);

    let w2 = serde_json::to_string(
        w.clone()
            .boost(2.0)
    )
    .unwrap();

    assert_eq!(w2, r#"{"user.id":{"boost":2.0,"value":"ki*y","wildcard":"wildcard"}}"#);

    let w3 = serde_json::to_string(
        w.clone()
            .case_insensitive(true)
    )
    .unwrap();

    assert_eq!(w3, r#"{"user.id":{"case_insensitive":true,"value":"ki*y","wildcard":"wildcard"}}"#);
}
