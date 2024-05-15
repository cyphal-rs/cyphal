use crate::{CyphalError, CyphalResult, ServiceId};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TestServiceId {
    value: u8,
}

impl ServiceId for TestServiceId {
    type T = u8;

    fn new(value: Self::T) -> CyphalResult<Self> {
        value.try_into()
    }

    fn value(&self) -> Self::T {
        self.value
    }
}

impl TryFrom<u8> for TestServiceId {
    type Error = CyphalError;

    fn try_from(value: u8) -> CyphalResult<Self> {
        if value > 127 {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self { value })
    }
}
