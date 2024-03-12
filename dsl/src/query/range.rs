use std::borrow::Cow;
use serde::{Serialize, Serializer, ser::SerializeMap};
use serde_with::{skip_serializing_none, SerializeDisplay};
use macros::DisplayCase;
use crate::{
    boost::Boost,
    field::Field,
    types::{
        number::Number,
        EqualsToDefault,
    },
};

pub struct Range<'a> {
    field: Field<'a>,
    opts: RangeOptions<'a>,
}

impl<'a> Serialize for Range<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.field, &self.opts)?;
        map.end()
    }
}

impl<'a> Range<'a> {
    pub fn new(field: Field<'a>) -> Self {
        Self {
            field,
            opts: Default::default(),
        }
    }

    pub fn gt(&mut self, v: RangeValue) -> &mut Self {
        self.opts.gt = Some(v);

        self
    }

    pub fn gte(&mut self, v: RangeValue) -> &mut Self {
        self.opts.gte = Some(v);

        self
    }

    pub fn lt(&mut self, v: RangeValue) -> &mut Self {
        self.opts.lt = Some(v);

        self
    }

    pub fn lte(&mut self, v: RangeValue) -> &mut Self {
        self.opts.lte = Some(v);

        self
    }

    pub fn format(&mut self, v: &'a str) -> &mut Self {
        self.opts.format = Some(v.into());

        self
    }

    pub fn relation(&mut self, v: Relation) -> &mut Self {
        self.opts.relation = v;

        self
    }

    pub fn time_zone(&mut self, v: &'a str) -> &mut Self {
        self.opts.time_zone = Some(v.into());

        self
    }

    pub fn boost(&mut self, v: f32) -> &mut Self {
        self.opts.boost = Boost(v);

        self
    }
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RangeOptions<'a> {
    gt: Option<RangeValue>,
    gte: Option<RangeValue>,
    lt: Option<RangeValue>,
    lte: Option<RangeValue>,
    format: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Relation::equals_to_default")]
    relation: Relation,
    time_zone: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Boost::equals_to_default")]
    boost: Boost,
}

impl<'a> Default for RangeOptions<'a> {
    fn default() -> Self {
        Self {
            gt: None,
            gte: None,
            lt: None,
            lte: None,
            format: None,
            relation: Default::default(),
            time_zone: None,
            boost: Default::default(),
        }
    }
}

#[allow(dead_code)]
pub enum RangeValue {
    Number(Number),
    Date(String),
}

impl Serialize for RangeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            Self::Date(d) => serializer.serialize_str(d),
            Self::Number(n) => n.serialize(serializer),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, DisplayCase, SerializeDisplay, PartialEq)]
#[display_case(case = "uppercase")]
pub enum Relation {
    Intersects,
    Contains,
    Within,
}

impl Default for Relation {
    fn default() -> Self {
        Self::Intersects
    }
}

impl EqualsToDefault for Relation {}
