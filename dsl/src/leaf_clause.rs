use serde::Serialize;
use crate::query::{
    exists::Exists,
    r#match::Match,
    range::Range,
    term::Term,
};

#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LeafClause<'a> {
    Exists(&'a Exists<'a>),
    Match(&'a Match<'a>),
    Term(&'a Term<'a>),
    Range(&'a Range<'a>),
}
