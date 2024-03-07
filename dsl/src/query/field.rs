use serde::Serialize;

#[derive(PartialEq ,Eq, Hash, Serialize)]
pub struct Field(String);

impl From<&str> for Field {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}
