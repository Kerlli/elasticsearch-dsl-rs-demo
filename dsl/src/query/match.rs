use std::borrow::Cow;
use macros::DisplayCase;
use serde::{Serialize, Serializer, ser::SerializeMap};
use serde_with::{skip_serializing_none, SerializeDisplay};
use crate::field::Field;

use super::QueryValue;

#[derive(Clone)]
pub struct Match<'a> {
    field: Field<'a>,
    inner: Inner<'a>,
}

impl<'a> Match<'a> {
    pub fn new(field: Field<'a>, value: QueryValue) -> Self {
        Self {
            field,
            inner: Inner {
                query: value,
                opts: Default::default(),
            },
        }
    }

    pub fn analyzer(&mut self, v: &'a str) -> &mut Self {
        self.inner.opts.analyzer = Some(v.into());

        self
    }

    pub fn auto_generate_synonyms_phrase_query(&mut self, v: bool) -> &mut Self {
        self.inner.opts.auto_generate_synonyms_phrase_query = AutoGenerateSynonymsPhraseQuery(v);

        self
    }

    pub fn fuzziness(&mut self, v: &'a str) -> &mut Self {
        self.inner.opts.fuzziness = Some(v.into());

        self
    }

    pub fn max_expansions(&mut self, v: i32) -> &mut Self {
        self.inner.opts.max_expansions = MaxExpansions(v);

        self
    }

    pub fn prefix_length(&mut self, v: i32) -> &mut Self {
        self.inner.opts.prefix_length = PrefixLength(v);

        self
    }

    pub fn fuzzy_transpositions(&mut self, v: bool) -> &mut Self {
        self.inner.opts.fuzzy_transpositions = FuzzyTranspositions(v);

        self
    }

    pub fn fuzzy_rewrite(&mut self, v: &'a str) -> &mut Self {
        self.inner.opts.fuzzy_rewrite = Some(v.into());

        self
    }

    pub fn lenient(&mut self, v: bool) -> &mut Self {
        self.inner.opts.lenient = Lenient(v);

        self
    }

    pub fn operator(&mut self, v: Operator) -> &mut Self {
        self.inner.opts.operator = Some(v);

        self
    }

    pub fn minimum_should_match(&mut self, v: &'a str) -> &mut Self {
        self.inner.opts.minimum_should_match = Some(v.into());

        self
    }

    pub fn zero_terms_query(&mut self, v: ZeroTermsQuery) -> &mut Self {
        self.inner.opts.zero_terms_query = Some(v);

        self
    }
}

impl<'a> Serialize for Match<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut m = serializer.serialize_map(Some(1))?;
        m.serialize_entry(&self.field, &self.inner)?;
        m.end()
    }
}

#[derive(Clone, Serialize)]
struct Inner<'a> {
    query: QueryValue,
    #[serde(flatten, skip_serializing_if = "MatchOptions::is_default")]
    opts: MatchOptions<'a>,
}

#[skip_serializing_none]
#[derive(Clone, PartialEq, Serialize)]
struct MatchOptions<'a> {
    analyzer: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "AutoGenerateSynonymsPhraseQuery::is_default")]
    auto_generate_synonyms_phrase_query: AutoGenerateSynonymsPhraseQuery,
    fuzziness: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "MaxExpansions::is_default")]
    max_expansions: MaxExpansions,
    #[serde(skip_serializing_if = "PrefixLength::is_default")]
    prefix_length: PrefixLength,
    #[serde(skip_serializing_if = "FuzzyTranspositions::is_default")]
    fuzzy_transpositions: FuzzyTranspositions,
    fuzzy_rewrite: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Lenient::is_default")]
    lenient: Lenient,
    operator: Option<Operator>,
    minimum_should_match: Option<Cow<'a, str>>,
    zero_terms_query: Option<ZeroTermsQuery>,
}

impl<'a> MatchOptions<'a> {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl<'a> Default for MatchOptions<'a> {
    fn default() -> Self {
        Self {
            analyzer: None,
            auto_generate_synonyms_phrase_query: Default::default(),
            fuzziness: None,
            max_expansions: Default::default(),
            prefix_length: Default::default(),
            fuzzy_transpositions: Default::default(),
            fuzzy_rewrite: None,
            lenient: Default::default(),
            operator: None,
            minimum_should_match: None,
            zero_terms_query: None,
        }
    }
}

#[derive(Clone, PartialEq, Serialize)]
struct AutoGenerateSynonymsPhraseQuery(bool);

impl AutoGenerateSynonymsPhraseQuery {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl Default for AutoGenerateSynonymsPhraseQuery {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Clone, PartialEq, Serialize)]
struct MaxExpansions(i32);

impl MaxExpansions {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl Default for MaxExpansions {
    fn default() -> Self {
        Self(50)
    }
}

#[derive(Clone, PartialEq, Serialize)]
struct PrefixLength(i32);

impl PrefixLength {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl Default for PrefixLength {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Clone, PartialEq, Serialize)]
struct FuzzyTranspositions(bool);

impl FuzzyTranspositions {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl Default for FuzzyTranspositions {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Clone, PartialEq, Serialize)]
struct Lenient(bool);

impl Lenient {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl Default for Lenient {
    fn default() -> Self {
        Self(false)
    }
}

#[allow(dead_code)]
#[derive(Clone, DisplayCase, PartialEq, SerializeDisplay)]
#[display_case(case = "uppercase")]
pub enum Operator {
    And,
    Or,
}

impl Default for Operator {
    fn default() -> Self {
        Self::Or
    }
}

#[allow(dead_code)]
#[derive(Clone, DisplayCase, PartialEq, SerializeDisplay)]
#[display_case(case = "lowercase")]
pub enum ZeroTermsQuery {
    All,
    None,
}

impl Default for ZeroTermsQuery {
    fn default() -> Self {
        Self::None
    }
}

