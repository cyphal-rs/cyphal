use cyphal::{CyphalError, CyphalResult, NodeId};

/// Respresents a UDP Node ID
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default)]
pub struct UdpNodeId {
    value: u16,
}

impl NodeId for UdpNodeId {
    type T = u16;

    fn value(&self) -> Self::T {
        self.value
    }
}

impl TryFrom<u16> for UdpNodeId {
    type Error = CyphalError;

    fn try_from(value: u16) -> CyphalResult<Self> {
        if value == 65535 {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self { value })
    }
}
