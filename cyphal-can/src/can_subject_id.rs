use cyphal::{CyphalError, CyphalResult, SubjectId};

/// Represents the Subject ID for the CAN transport
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct CanSubjectId {
    value: u16,
}

impl SubjectId for CanSubjectId {
    type T = u16;

    fn new(value: Self::T) -> CyphalResult<Self> {
        value.try_into()
    }

    fn value(&self) -> Self::T {
        self.value
    }
}

impl TryFrom<u16> for CanSubjectId {
    type Error = CyphalError;

    fn try_from(value: u16) -> CyphalResult<Self> {
        if value > 8191 {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self { value })
    }
}