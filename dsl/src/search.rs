use serde::Serialize;
use serde_with::skip_serializing_none;
use super::{
    query::Query,
    sort::Sort,
};

#[skip_serializing_none]
#[derive(Serialize)]
pub struct Search<'a> {
    query: Option<&'a Query<'a>>,
    sort: Option<Sort>,
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

    pub fn sort(&mut self, sort: Sort) -> &mut Self {
        self.sort = Some(sort);

        self
    }

    pub fn build(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
