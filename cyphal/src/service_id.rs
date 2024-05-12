/// Represents a Service ID
pub trait ServiceId:
    Sized + Clone + Copy + Eq + PartialEq + Ord + PartialOrd + Default + TryFrom<Self::T>
{
    /// Underlying type on the Service ID
    type T;

    /// Returns the value of the Service ID
    fn value(&self) -> Self::T;
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::{test::TestServiceId, ServiceId};

    #[test]
    fn test_default() {
        let id = TestServiceId::default();

        assert_eq!(id.value(), 0);
    }
}
