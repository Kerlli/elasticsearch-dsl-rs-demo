use serde::{Serialize, Serializer, ser::SerializeMap};
use serde_with::{skip_serializing_none, SerializeDisplay};
use macros::DisplayCase;
use crate::types::number::Number;

pub struct Range {
    field: String,
    opts: Options,
}

impl Serialize for Range {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.field, &self.opts)?;
        map.end()
    }
}

impl Range {
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
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

    pub fn format(&mut self, v: &str) -> &mut Self {
        self.opts.format = Some(v.to_string());

        self
    }

    pub fn relation(&mut self, v: Relation) -> &mut Self {
        self.opts.relation = v;

        self
    }

    pub fn time_zone(&mut self, v: &str) -> &mut Self {
        self.opts.time_zone = Some(v.to_string());

        self
    }

    pub fn boost(&mut self, v: f32) -> &mut Self {
        self.opts.boost = Boost(v);

        self
    }
}

#[skip_serializing_none]
#[derive(Serialize)]
struct Options {
    gt: Option<RangeValue>,
    gte: Option<RangeValue>,
    lt: Option<RangeValue>,
    lte: Option<RangeValue>,
    format: Option<String>,
    #[serde(skip_serializing_if = "Relation::is_default")]
    relation: Relation,
    time_zone: Option<String>,
    #[serde(skip_serializing_if = "Boost::is_default")]
    boost: Boost,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            gt: None,
            gte: None,
            lt: None,
            lte: None,
            format: None,
            relation: Default::default(),
            time_zone: None,
            boost: Boost(1.0),
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
#[derive(Debug, DisplayCase, SerializeDisplay, PartialEq, Eq)]
#[display_case(case = "uppercase")]
pub enum Relation {
    Intersects,
    Contains,
    Within,
}

impl Relation {
    fn is_default(&self) -> bool {
        self == &Default::default()
    }
}

impl Default for Relation {
    fn default() -> Self {
        Self::Intersects
    }
}

#[derive(PartialEq)]
struct Boost(f32);

impl Boost {
    fn is_default(&self) -> bool {
        self.0 == 1.0
    }
}

impl Serialize for Boost {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_f32(self.0)
    }
}

