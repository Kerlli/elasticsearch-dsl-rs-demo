use std::collections::HashMap;
use macros::DisplayCase;
use serde::Serialize;
use serde_with::SerializeDisplay;

use super::{
    field::Field,
    QueryValue,
};

#[derive(Serialize)]
pub struct Match<'a> {
    #[serde(flatten)]
    inner: HashMap<Field<'a>, Inner>,
}

impl<'a> Match<'a> {
    pub fn new(field: &'a str, value: QueryValue) -> Self {
        Self {
            inner: HashMap::from([
                (field.into(), Inner {
                    query: value,
                    opts: None,
                })
            ])
        }
    }
}

#[derive(Serialize)]
struct Inner {
    query: QueryValue,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    opts: Option<MatchOptions>,
}

#[derive(Serialize)]
struct MatchOptions {
    analyzer: Option<String>,
    auto_generate_synonyms_phrase_query: Option<bool>,
    fuzziness: Option<String>,
    max_expansions: Option<i32>,
    prefix_length: Option<i32>,
    fuzzy_transpositions: Option<bool>,
    fuzzy_rewrite: Option<String>,
    lenient: Option<bool>,
    operator: Option<Operator>,
    minimum_should_match: Option<String>,
    zero_terms_query: Option<ZeroTermsQuery>,
}

impl Default for MatchOptions {
    fn default() -> Self {
        Self {
            analyzer: None,
            auto_generate_synonyms_phrase_query: Some(true),
            fuzziness: None,
            max_expansions: Some(50),
            prefix_length: Some(0),
            fuzzy_transpositions: Some(true),
            fuzzy_rewrite: None,
            lenient: Some(false),
            operator: Some(Operator::Or),
            minimum_should_match: None,
            zero_terms_query: Some(ZeroTermsQuery::None),
        }
    }
}

#[allow(dead_code)]
#[derive(DisplayCase, SerializeDisplay)]
#[display_case(case = "uppercase")]
pub enum Operator {
    And,
    Or,
}

#[allow(dead_code)]
#[derive(DisplayCase, SerializeDisplay)]
#[display_case(case = "lowercase")]
pub enum ZeroTermsQuery {
    All,
    None,
}

