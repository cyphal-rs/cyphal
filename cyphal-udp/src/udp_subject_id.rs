use cyphal::{CyphalError, CyphalResult, SubjectId};

/// Represents the Subject ID for the UDP transport
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct UdpSubjectId {
    value: u16,
}

impl SubjectId for UdpSubjectId {
    type T = u16;

    fn value(&self) -> Self::T {
        self.value
    }
}

impl TryFrom<u16> for UdpSubjectId {
    type Error = CyphalError;

    fn try_from(value: u16) -> CyphalResult<Self> {
        if value > 32767 {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self { value })
    }
}
