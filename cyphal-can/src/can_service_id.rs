use cyphal::{CyphalError, CyphalResult, ServiceId};

/// Represents the Service ID for the CAN transport
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct CanServiceId {
    value: u16,
}

impl ServiceId for CanServiceId {
    type T = u16;

    fn value(&self) -> Self::T {
        self.value
    }
}

impl TryFrom<u16> for CanServiceId {
    type Error = CyphalError;

    fn try_from(value: u16) -> CyphalResult<Self> {
        if value > 511 {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self { value })
    }
}
