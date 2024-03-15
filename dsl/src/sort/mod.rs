pub mod nested;

use std::borrow::Cow;
use serde::{
    Serialize,
    Serializer,
    ser::{
        SerializeMap,
        SerializeSeq,
    },
};
use serde_with::{SerializeDisplay, skip_serializing_none};
use macros::DisplayCase;
use nested::SortNested;
use crate::field::Field;

#[derive(Clone)]
pub struct Sort<'a> {
    clauses: Vec<&'a SortClause<'a>>,
}

impl<'a> Sort<'a> {
    pub fn new() -> Self {
        Self {
            clauses: vec![],
        }
    }

    pub fn sort(&mut self, sort_clause: &'a SortClause<'a>) -> &mut Self {
        self.clauses.push(sort_clause);

        self
    }
}

impl<'a> Serialize for Sort<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut seq = serializer.serialize_seq(Some(self.clauses.len()))?;
        for clause in &self.clauses {
            seq.serialize_element(&clause)?;
        }
        seq.end()
    }
}

#[derive(Clone)]
pub struct SortClause<'a> {
    field: Field<'a>,
    opts: SortOptions<'a>,
}

impl<'a> SortClause<'a> {
    pub fn new(field: &'a str) -> Self {
        Self {
            field: field.into(),
            opts: Default::default(),
        }
    }

    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.opts.format = Some(format.into());

        self
    }

    pub fn order(&mut self, order: Order) -> &mut Self {
        self.opts.order = Some(order);

        self
    }

    pub fn mode(&mut self, mode: Mode) -> &mut Self {
        self.opts.mode = Some(mode);

        self
    }

    pub fn numberic_type(&mut self, numberic_type: NumbericType) -> &mut Self {
        self.opts.numberic_type = Some(numberic_type);

        self
    }

    pub fn nested(&mut self, nested: SortNested<'a>) -> &mut Self {
        self.opts.nested = Some(nested.clone());

        self
    }
}

impl<'a> Serialize for SortClause<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.field, &self.opts)?;
        map.end()
    }
}

#[skip_serializing_none]
#[derive(Clone, Serialize)]
struct SortOptions<'a> {
    format: Option<Cow<'a, str>>,
    order: Option<Order>,
    mode: Option<Mode>,
    numberic_type: Option<NumbericType>,
    nested: Option<SortNested<'a>>,
}

impl<'a> Default for SortOptions<'a> {
    fn default() -> Self {
        Self {
            format: None,
            order: None,
            mode: None,
            numberic_type: None,
            nested: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, DisplayCase, SerializeDisplay)]
#[display_case(case = "lowercase")]
pub enum Order {
    Asc,
    Desc,
}

#[allow(dead_code)]
#[derive(Clone, DisplayCase, SerializeDisplay)]
#[display_case(case = "lowercase")]
pub enum Mode {
    Min,
    Max,
    Sum,
    Avg,
    Median,
}

#[allow(dead_code)]
#[derive(Clone, DisplayCase, SerializeDisplay)]
#[display_case(case = "snakecase")]
pub enum NumbericType {
    Double,
    Long,
    Date,
    DateNanos,
}

