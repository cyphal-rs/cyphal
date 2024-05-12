/// Represents a Transfer ID
pub trait TransferId<T>: Sized + Copy + Clone + Default + TryFrom<T> {
    /// Returns the value of the Transfer ID
    fn value(&self) -> T;

    /// Returns the next Transfer ID
    fn next(&self) -> Self;
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::{
        test::{TestTransferId, TEST_MAX_TRANSFER_ID},
        TransferId,
    };
    use std::vec::Vec;

    #[test]
    fn test_default() {
        let id = TestTransferId::default();

        assert_eq!(id.value(), 0);
    }

    #[test]
    fn test_next() {
        let mut id = TestTransferId::default();
        let data: Vec<u8> = (0..255).collect();

        for i in data {
            assert_eq!(id.value(), i % (TEST_MAX_TRANSFER_ID + 1));
            id = id.next();
        }
    }
}
