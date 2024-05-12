/// Represents a Node ID
pub trait NodeId:
    Sized + Clone + Copy + Eq + PartialEq + Ord + PartialOrd + Default + TryFrom<Self::T>
{
    /// Underlying type on the Node ID
    type T;

    /// Returns the value of the Node ID
    fn value(&self) -> Self::T;
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::{test::TestNodeId, NodeId};

    #[test]
    fn test_default() {
        let id = TestNodeId::default();

        assert_eq!(id.value(), 0);
    }
}
