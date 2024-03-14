use std::marker::PhantomData;
use serde::Serialize;

#[derive(Serialize)]
pub struct Empty<'a> {
    #[serde(skip_serializing)]
    _unused: PhantomData<&'a ()>,
}

impl<'a> Empty<'a> {
    pub fn new() -> Self {
        Self {
            _unused: PhantomData,
        }
    }
}
