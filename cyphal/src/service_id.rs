/// Represents a Service ID
pub trait ServiceId:
    Sized + Clone + Copy + Eq + PartialEq + Ord + PartialOrd + TryFrom<Self::T>
{
    /// Underlying type on the Service ID
    type T;

    /// Returns the value of the Service ID
    fn value(&self) -> Self::T;
}
