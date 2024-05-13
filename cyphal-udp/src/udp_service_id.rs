use cyphal::{CyphalError, CyphalResult, ServiceId};

/// Represents the Service ID for the UDP transport
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct UdpServiceId {
    value: u16,
}

impl ServiceId for UdpServiceId {
    type T = u16;

    fn value(&self) -> Self::T {
        self.value
    }
}

impl TryFrom<u16> for UdpServiceId {
    type Error = CyphalError;

    fn try_from(value: u16) -> CyphalResult<Self> {
        if value >= 16384 {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self { value })
    }
}
