use serde::Serialize;
use serde_with::skip_serializing_none;
use crate::sort::Sort;

use super::query::Query;

#[skip_serializing_none]
#[derive(Serialize)]
pub struct Search<'a> {
    query: Option<&'a Query<'a>>,
    sort: Option<&'a Sort<'a>>,
}

impl<'a> Search<'a> {
    pub fn new() -> Self {
        Self {
            query: None,
            sort: None,
        }
    }

    pub fn query(&mut self, query: &'a Query) -> &mut Self {
        self.query = Some(query);

        self
    }

    pub fn sort(&mut self, sort: &'a Sort<'a>) -> &mut Self {
        self.sort = Some(sort);

        self
    }

    pub fn build(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
