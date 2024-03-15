pub mod number;

pub(crate) trait EqualsToDefault
where
    Self: Default + PartialEq,
{
    fn equals_to_default(&self) -> bool {
        *self == Self::default()
    }
}
