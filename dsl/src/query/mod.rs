pub mod bool;
pub mod exists;
pub mod r#match;
pub mod range;
use serde::{Serialize, Serializer};
use bool::Bool;
use serde_with::skip_serializing_none;
use crate::types::number::Number;

#[skip_serializing_none]
#[derive(Serialize)]
pub struct Query<'a> {
    r#bool: Option<&'a Bool<'a>>,
}

impl<'a> Query<'a> {
    pub fn new() -> Self {
        Self {
            r#bool: None,
        }
    }

    pub fn bool(&mut self, q: &'a Bool) -> &mut Self {
        self.r#bool = Some(q);

        self
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum QueryValue {
    Text(String),
    Number(Number),
    Boolean(bool),
    // Date,
}

impl Serialize for QueryValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            Self::Boolean(v) => serializer.serialize_bool(*v),
            Self::Number(v) => v.serialize(serializer),
            Self::Text(v) => serializer.serialize_str(v),
        }
    }
}




