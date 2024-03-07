use macros::DisplayCase;

#[derive(DisplayCase)]
#[display_case(case = "foocase")]
enum Test {
    Foo,
    Bar,
}

fn main() {}
