use serde::Serialize;
use crate::types::EqualsToDefault;

/// Case insensitive, defaults to false
#[derive(Clone, PartialEq, Serialize)]
pub struct CaseInsensitive(pub bool);

impl Default for CaseInsensitive {
    fn default() -> Self {
        Self(false)
    }
}

impl EqualsToDefault for CaseInsensitive {}
