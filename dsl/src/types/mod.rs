pub mod number;

pub trait EqualsToDefault
where
    Self: Default + PartialEq,
{
    fn equals_to_default(&self) -> bool {
        *self == Self::default()
    }
}
