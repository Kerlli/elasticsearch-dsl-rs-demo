use std::borrow::Cow;
use macros::DisplayCase;
use serde::Serialize;
use serde_with::SerializeDisplay;

use crate::types::EqualsToDefault;
use super::Query;

/// Build a nested Query
/// 
/// Reference: [Query DSL Nested Query](https://www.elastic.co/guide/en/elasticsearch/reference/7.14/query-dsl-nested-query.html#multi-level-nested-query-ex)
/// 
/// # Example
/// ```
/// use dsl::{
///     clause,
///     query::{
///         bool::Bool,
///         nested::Nested,
///         Query,
///         QueryValue,
///     },
/// };
/// let q = Query::new()
///     .nested(
///         Nested::new(
///             "event",
///             Query::new()
///                 .bool(
///                     Bool::new()
///                         .must(
///                             clause!(
///                                 Match,
///                                 "event.action",
///                                 QueryValue::Text("logged-in".to_owned())
///                             )
///                         )
///                 )
///                 .clone()
///         )
///     );
/// ```
#[derive(Clone, Serialize)]
pub struct Nested<'a> {
    path: Cow<'a, str>,
    query: Query<'a>,
    #[serde(skip_serializing_if = "ScoreMode::equals_to_default")]
    score_mode: ScoreMode,
    #[serde(skip_serializing_if = "IgnoreUnmapped::equals_to_default")]
    ignore_unmapped: IgnoreUnmapped,
}

impl<'a> Nested<'a> {
    pub fn new(path: &'a str, query: Query<'a>) -> Self {
        Self {
            path: path.into(),
            query,
            score_mode: Default::default(),
            ignore_unmapped: Default::default(),
        }
    }

    pub fn score_mode(&mut self, v: ScoreMode) -> &mut Self {
        self.score_mode = v;
        
        self
    }

    pub fn ignore_unmapped(&mut self, v: bool) -> &mut Self {
        self.ignore_unmapped = IgnoreUnmapped(v);

        self
    }
}

#[derive(Clone, PartialEq, DisplayCase, SerializeDisplay)]
#[display_case(case = "lowercase")]
pub enum ScoreMode {
    Avg,
    Max,
    Min,
    None,
    Sum,
}

impl Default for ScoreMode {
    fn default() -> Self {
        Self::Avg
    }
}

impl EqualsToDefault for ScoreMode {}

#[derive(Clone, PartialEq, Serialize)]
struct IgnoreUnmapped(bool);

impl Default for IgnoreUnmapped {
    fn default() -> Self {
        Self(false)
    }
}

impl EqualsToDefault for IgnoreUnmapped {}
