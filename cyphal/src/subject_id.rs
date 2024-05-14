use crate::CyphalResult;

/// Represents a Subject ID
pub trait SubjectId:
    Sized + Clone + Copy + Eq + PartialEq + Ord + PartialOrd + TryFrom<Self::T>
{
    /// Underlying type on the Subject ID
    type T;

    /// Constructs a new Subject ID
    fn new(value: Self::T) -> CyphalResult<Self>;

    /// Returns the value of the Subject ID
    fn value(&self) -> Self::T;
}
