use cyphal::{CyphalResult, Message, NodeId, Priority, Router, ServiceId, SubjectId, Transport};
use cyphal_can::CanTransport;
use cyphal_socketcan::CanSocket;

const MESSAGE_SIZE: usize = 65;

#[async_std::main]
async fn main() {
    let socket = CanSocket::new("vcan0").unwrap();
    let mut transport = CanTransport::new(socket).unwrap();

    let data: Vec<u8> = (1..(MESSAGE_SIZE + 1) as u8).collect();
    let data: [u8; MESSAGE_SIZE] = data.try_into().unwrap();
    let message = TestMessage::new(Priority::High, 2.try_into().unwrap(), None, data).unwrap();

    match transport.publish(&message).await {
        Ok(_) => println!("Message sent successfully"),
        Err(e) => println!("Failed to send message: {}", e),
    }
}

pub struct TestRouter {}

impl Router for TestRouter {
    async fn process_message(
        &self,
        _priority: Priority,
        _subject: SubjectId,
        _source: NodeId,
        _data: &[u8],
    ) {
        todo!()
    }

    async fn process_request(
        &self,
        _priority: Priority,
        _service: ServiceId,
        _source: NodeId,
        _destination: NodeId,
        _data: &[u8],
    ) {
        todo!()
    }
}

pub struct TestMessage {
    priority: Priority,
    subject: SubjectId,
    source: Option<NodeId>,
    payload: [u8; MESSAGE_SIZE],
}

impl TestMessage {
    pub fn new(
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
        payload: [u8; MESSAGE_SIZE],
    ) -> CyphalResult<Self> {
        Ok(Self {
            priority,
            subject,
            source,
            payload,
        })
    }
}

impl Message for TestMessage {
    const SIZE: usize = MESSAGE_SIZE;

    fn source(&self) -> Option<NodeId> {
        self.source
    }

    fn subject(&self) -> SubjectId {
        self.subject
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn data(&self) -> &[u8] {
        &self.payload
    }
}
