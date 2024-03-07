pub(super) fn snakecase(s: &str) -> String {
    let mut snake = String::new();

    for (i, ch) in s.char_indices() {
        if i > 0 && ch.is_uppercase() {
            snake.push('_');
        }
        snake.push(ch.to_ascii_lowercase());
    }

    snake
}

#[test]
fn test_snakecase() {
    use self::snakecase;

    assert_eq!(&snakecase(""), "");
    assert_eq!(&snakecase("Foo"), "foo");
    assert_eq!(&snakecase("FooBar"), "foo_bar");
    assert_eq!(&snakecase("fooBar"), "foo_bar");
    assert_eq!(&snakecase("Foobar"), "foobar");
}
