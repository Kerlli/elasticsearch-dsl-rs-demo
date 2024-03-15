pub mod bool;
pub mod exists;
pub mod r#match;
pub mod match_all;
pub mod match_none;
pub mod nested;
pub mod range;
pub mod term;
pub mod prelude;
pub mod wildcard;

use std::rc::Rc;
use serde::{Serialize, Serializer};
use bool::Bool;
use exists::Exists;
use r#match::Match;
use match_all::MatchAll;
use match_none::MatchNone;
use nested::Nested;
use term::Term;
use range::Range;
use serde_with::skip_serializing_none;
use wildcard::Wildcard;
use crate::types::number::Number;

#[skip_serializing_none]
#[derive(Clone, Serialize)]
pub struct Query<'a> {
    r#bool: Option<&'a Bool<'a>>,
    nested: Option<Rc<Nested<'a>>>,
    wildcard: Option<&'a Wildcard<'a>>,
}

impl<'a> Query<'a> {
    pub fn new() -> Self {
        Self {
            r#bool: None,
            nested: None,
            wildcard: None,
        }
    }

    pub fn bool(&mut self, q: &'a Bool) -> &mut Self {
        self.r#bool = Some(q);

        self
    }

    pub fn nested(&mut self, q: Nested<'a>) -> &mut Self {
        self.nested = Some(Rc::new(q));

        self
    }

    pub fn wildcard(&mut self, q: &'a Wildcard<'a>) -> &mut Self {
        self.wildcard = Some(q);

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

macro_rules! declare_leaf_clause {
    ($($clause_name:ident),*) => {
        #[derive(Clone, Serialize)]
        #[serde(rename_all = "snake_case")]
        pub enum LeafClause<'a> {
            $($clause_name(&'a $clause_name<'a>),)*
        }
    };
}

declare_leaf_clause!(
    Exists,
    Match,
    MatchAll,
    MatchNone,
    Term,
    Range
);




