use std::borrow::Cow;
use serde::{
    Serialize,
    Serializer,
    ser::SerializeMap,
};
use crate::{
    boost::Boost,
    field::Field,
    types::EqualsToDefault,
};

pub struct Term<'a> {
    field: Field<'a>,
    opts: TermOptions<'a>,
}

impl<'a> Term<'a> {
    pub fn new(field: Field<'a>, value: &'a str) -> Self {
        Self {
            field,
            opts: TermOptions {
                value: value.into(),
                boost: Default::default(),
                case_insensitive: Default::default(),
            }
        }
    }

    pub fn boost(&mut self, v: f32) -> &mut Self {
        self.opts.boost = Boost(v);

        self
    }

    pub fn case_insensitive(&mut self, v: bool) -> &mut Self {
        self.opts.case_insensitive = CaseInsensitive(v);

        self
    }
}

impl<'a> Serialize for Term<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut m = serializer.serialize_map(Some(1))?;
        m.serialize_entry(&self.field, &self.opts)?;
        m.end()
    }
}

#[derive(Serialize)]
pub struct TermOptions<'a> {
    value: Cow<'a, str>,
    #[serde(skip_serializing_if = "Boost::equals_to_default")]
    boost: Boost,
    #[serde(skip_serializing_if = "CaseInsensitive::equals_to_default")]
    case_insensitive: CaseInsensitive,
}

#[derive(PartialEq, Serialize)]
struct CaseInsensitive(bool);

impl Default for CaseInsensitive {
    fn default() -> Self {
        Self(false)
    }
}

impl EqualsToDefault for CaseInsensitive {}
