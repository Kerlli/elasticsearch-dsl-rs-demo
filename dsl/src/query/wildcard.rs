use std::borrow::Cow;
use serde::{
    Serialize,
    Serializer,
    ser::SerializeMap,
};
use serde_with::skip_serializing_none;
use crate::{
    boost::Boost,
    case_insensitive::CaseInsensitive,
    field::Field,
    types::EqualsToDefault,
};

#[derive(Clone)]
pub struct Wildcard<'a> {
    field: Field<'a>,
    opts: WildcardOptions<'a>,
}

impl<'a> Wildcard<'a> {
    pub fn new(field: &'a str, value: &'a str, wildcard: &'a str) -> Self {
        Self {
            field: field.into(),
            opts: WildcardOptions {
                boost: Default::default(),
                case_insensitive: Default::default(),
                rewrite: None,
                value: value.into(),
                wildcard: wildcard.into(),
            }
        }
    }

    pub fn boost(&mut self, boost: f32) -> &mut Self {
        self.opts.boost = Boost(boost);

        self
    }

    pub fn case_insensitive(&mut self, v: bool) -> &mut Self {
        self.opts.case_insensitive = CaseInsensitive(v);

        self
    }

    pub fn rewrite(&mut self, v: &'a str) -> &mut Self {
        self.opts.rewrite = Some(v.into());

        self
    }
}

impl<'a> Serialize for Wildcard<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut m = serializer.serialize_map(Some(1))?;
        m.serialize_entry(&self.field, &self.opts)?;
        m.end()
    }
}

#[skip_serializing_none]
#[derive(Clone, Serialize)]
struct WildcardOptions<'a> {
    #[serde(skip_serializing_if = "Boost::equals_to_default")]
    boost: Boost,
    #[serde(skip_serializing_if = "CaseInsensitive::equals_to_default")]
    case_insensitive: CaseInsensitive,
    rewrite: Option<Cow<'a, str>>,
    value: Cow<'a, str>,
    wildcard: Cow<'a, str>,
}
