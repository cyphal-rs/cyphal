use cyphal::{CyphalError, CyphalResult, NodeId};

/// Respresents a CAN Node ID
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct CanNodeId {
    value: u8,
}

impl NodeId for CanNodeId {
    type T = u8;

    fn value(&self) -> Self::T {
        self.value
    }
}

impl TryFrom<u8> for CanNodeId {
    type Error = CyphalError;

    fn try_from(value: u8) -> CyphalResult<Self> {
        if value > 127 {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self { value })
    }
}
