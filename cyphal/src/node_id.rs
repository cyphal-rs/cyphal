use crate::CyphalError;

pub struct NodeId(u16);

const NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST: u16 = 0xffff;

impl TryFrom<u16> for NodeId {
    type Error = CyphalError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST {
            Err(CyphalError::OutOfRange)
        } else {
            Ok(NodeId(value))
        }
    }
}
impl From<NodeId> for u16 {
    fn from(id: NodeId) -> Self {
        id.0
    }
}
impl From<NodeId> for u32 {
    fn from(id: NodeId) -> Self {
        id.0.into()
    }
}

impl From<NodeId> for usize {
    fn from(id: NodeId) -> Self {
        id.0.into()
    }
}
