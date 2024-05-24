use cyphal::{CyphalResult, Message, NodeId, Priority, SubjectId};

const HEARTBEAT_MESSAGE_SIZE: usize = 65;
const HEARTBEAT_PORT_ID: SubjectId = 7509;

pub struct HeartbeatMessage {
    source: NodeId,
    payload: [u8; HEARTBEAT_MESSAGE_SIZE],
}

impl HeartbeatMessage {
    pub fn new(source: NodeId, payload: [u8; HEARTBEAT_MESSAGE_SIZE]) -> CyphalResult<Self> {
        let heartbeat_message = Ok(Self { source, payload });
        heartbeat_message
    }
}

impl Message for HeartbeatMessage {
    const SIZE: usize = HEARTBEAT_MESSAGE_SIZE;

    fn source(&self) -> Option<NodeId> {
        Some(self.source)
    }

    fn subject(&self) -> SubjectId {
        HEARTBEAT_PORT_ID
    }

    fn priority(&self) -> Priority {
        Priority::Nominal
    }

    fn data(&self) -> &[u8] {
        &self.payload
    }
}
