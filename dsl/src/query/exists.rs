use serde::Serialize;
use crate::field::Field;

#[derive(Serialize)]
pub struct Exists<'a> {
    field: Field<'a>,
}

impl<'a> Exists<'a> {
    pub fn new(field: Field<'a>) -> Self {
        Self {
            field,
        }
    }
}
