use crate::CyphalResult;

/// Represents a Service ID
pub trait ServiceId:
    Sized + Clone + Copy + Eq + PartialEq + Ord + PartialOrd + TryFrom<Self::T>
{
    /// Underlying type on the Service ID
    type T;

    /// Constructs a new Service ID
    fn new(value: Self::T) -> CyphalResult<Self>;

    /// Returns the value of the Service ID
    fn value(&self) -> Self::T;
}
