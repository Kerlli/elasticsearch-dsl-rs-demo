#[test]
fn test_serialize() {
    use crate::clause;
    
    let s = serde_json::to_string(
        &clause!(MatchNone)
    )
    .unwrap();

    assert_eq!(s, r#"{"match_none":{}}"#);
}
