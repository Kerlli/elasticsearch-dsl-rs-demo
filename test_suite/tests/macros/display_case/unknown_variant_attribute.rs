use macros::DisplayCase;

#[derive(DisplayCase)]
#[display_case(case = "lowercase")]
enum Test {
    Foo,
    #[display_case(bar = "bar")]
    Bar,
}

fn main() {}
