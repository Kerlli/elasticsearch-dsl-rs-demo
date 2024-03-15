use std::borrow::Cow;
use std::rc::Rc;
use serde::Serialize;

use crate::query::LeafClause;

#[derive(Clone, Serialize)]
pub struct SortNested<'a> {
    path: Cow<'a, str>,
    filter: Option<LeafClause<'a>>,
    max_children: Option<i32>,
    nested: Option<Rc<SortNested<'a>>>,
}

impl<'a> SortNested<'a> {
    pub fn new(path: &'a str) -> Self {
        Self {
            path: path.into(),
            filter: None,
            max_children: None,
            nested: None,
        }
    }

    pub fn filter(&mut self, filter: LeafClause<'a>) -> &mut Self {
        self.filter = Some(filter);

        self
    }

    pub fn max_children(&mut self, v: i32) -> &mut Self {
        self.max_children = Some(v);

        self
    }

    pub fn nested(&mut self, nested: SortNested<'a>) -> &mut Self {
        self.nested = Some(Rc::new(nested));

        self
    }
}
