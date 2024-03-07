use macros::DisplayCase;

#[derive(DisplayCase)]
#[display_case(case = "lowercase")]
struct Test {
    foo: String,
    bar: i32,
}

fn main() {}
