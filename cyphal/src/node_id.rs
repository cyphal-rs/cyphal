/// Represents a Node ID
pub trait NodeId:
    Sized + Clone + Copy + Eq + PartialEq + Ord + PartialOrd + TryFrom<Self::T>
{
    /// Underlying type on the Node ID
    type T;

    /// Returns the value of the Node ID
    fn value(&self) -> Self::T;
}
