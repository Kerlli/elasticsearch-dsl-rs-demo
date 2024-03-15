use serde::Serialize;
use crate::types::EqualsToDefault;

/// Floating point number used to decrease or increase the relevance
/// scores of a query. Defaults to 1.0.
#[derive(Clone, PartialEq, Serialize)]
pub struct Boost(pub f32);

impl Default for Boost {
    fn default() -> Self {
        Self(1.0)
    }
}

impl EqualsToDefault for Boost {}
