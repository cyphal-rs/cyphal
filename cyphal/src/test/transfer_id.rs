use super::TEST_MAX_TRANSFER_ID;
use crate::{CyphalError, CyphalResult, TransferId};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct TestTransferId {
    value: u8,
}

impl TransferId for TestTransferId {
    type T = u8;

    fn value(&self) -> u8 {
        self.value
    }

    fn next(&self) -> Self {
        if self.value == TEST_MAX_TRANSFER_ID {
            TestTransferId { value: 0 }
        } else {
            TestTransferId {
                value: self.value + 1,
            }
        }
    }
}

impl TryFrom<u8> for TestTransferId {
    type Error = CyphalError;

    fn try_from(value: u8) -> CyphalResult<Self> {
        if value > 31 {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self { value })
    }
}
