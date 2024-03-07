use macros::DisplayCase;

#[derive(DisplayCase)]
#[display_case(foo = "lowercase")]
enum Test {
    Foo,
    Bar,
}

fn main() {}
