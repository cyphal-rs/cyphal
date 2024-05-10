use cyphal::{CyphalResult, Message, NodeId, Priority, SubjectId, Transport};
use cyphal_can::CanTransport;
use cyphal_socketcan::CanSocket;

const SINGLE_SIZE: usize = 2;
const MULTI_SIZE: usize = 65;

#[async_std::main]
async fn main() {
    let socket = CanSocket::new("vcan0").unwrap();
    let mut transport = CanTransport::new(socket).unwrap();

    let data: Vec<u8> = (1..3).collect();
    let data: [u8; 2] = data.try_into().unwrap();
    let message = SingleFrameMessage::new(Priority::Nominal, 1, None, data).unwrap();

    let _ = transport.publish(&message).await;

    let data: Vec<u8> = (1..66).collect();
    let data: [u8; 65] = data.try_into().unwrap();
    let message = MultiFrameMessage::new(Priority::High, 2, None, data).unwrap();

    let _ = transport.publish(&message).await;
}

pub struct SingleFrameMessage {
    priority: Priority,
    subject: SubjectId,
    source: Option<NodeId>,
    payload: [u8; SINGLE_SIZE],
}

impl SingleFrameMessage {
    pub fn new(
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
        payload: [u8; SINGLE_SIZE],
    ) -> CyphalResult<Self> {
        Ok(Self {
            priority,
            subject,
            source,
            payload,
        })
    }
}

impl Message<SINGLE_SIZE> for SingleFrameMessage {
    fn priority(&self) -> Priority {
        self.priority
    }

    fn subject(&self) -> SubjectId {
        self.subject
    }

    fn source(&self) -> Option<NodeId> {
        self.source
    }

    fn data(&self) -> &[u8; SINGLE_SIZE] {
        &self.payload
    }
}

pub struct MultiFrameMessage {
    priority: Priority,
    subject: SubjectId,
    source: Option<NodeId>,
    payload: [u8; MULTI_SIZE],
}

impl MultiFrameMessage {
    pub fn new(
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
        payload: [u8; MULTI_SIZE],
    ) -> CyphalResult<Self> {
        Ok(Self {
            priority,
            subject,
            source,
            payload,
        })
    }
}

impl Message<MULTI_SIZE> for MultiFrameMessage {
    fn source(&self) -> Option<NodeId> {
        self.source
    }

    fn subject(&self) -> SubjectId {
        self.subject
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn data(&self) -> &[u8; MULTI_SIZE] {
        &self.payload
    }
}
