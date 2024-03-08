use std::borrow::Cow;
use serde::Serialize;

#[derive(Clone, PartialEq ,Eq, Hash, Serialize)]
pub struct Field<'a>(Cow<'a, str>);

impl<'a> From<&'a str> for Field<'a> {
    fn from(s: &'a str) -> Self {
        Self(s.into())
    }
}
