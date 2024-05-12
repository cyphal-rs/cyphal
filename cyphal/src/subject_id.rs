/// Represents a Subject ID
pub trait SubjectId:
    Sized + Clone + Copy + Eq + PartialEq + Ord + PartialOrd + Default + TryFrom<Self::T>
{
    /// Underlying type on the Subject ID
    type T;

    /// Returns the value of the Subject ID
    fn value(&self) -> Self::T;
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::{test::TestSubjectId, SubjectId};

    #[test]
    fn test_default() {
        let id = TestSubjectId::default();

        assert_eq!(id.value(), 0);
    }
}
