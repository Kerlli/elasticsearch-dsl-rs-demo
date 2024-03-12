use serde::Serialize;
use crate::query::{
    exists::Exists,
    r#match::Match,
    range::Range,
};

#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LeafClause<'a> {
    Exists(Exists<'a>),
    Match(Match<'a>),
    Term,
    Range(&'a Range<'a>),
}
