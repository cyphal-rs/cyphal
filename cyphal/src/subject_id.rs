/// Represents a Subject ID
pub trait SubjectId:
    Sized + Clone + Copy + Eq + PartialEq + Ord + PartialOrd + TryFrom<Self::T>
{
    /// Underlying type on the Subject ID
    type T;

    /// Returns the value of the Subject ID
    fn value(&self) -> Self::T;
}
