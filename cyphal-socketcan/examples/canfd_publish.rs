use cyphal::{CyphalResult, Message, Priority, Transport};
use cyphal_can::{CanNodeId, CanSubjectId, CanTransport};
use cyphal_socketcan::CanFdSocket;

const MESSAGE_SIZE: usize = 65;

#[async_std::main]
async fn main() {
    let socket = CanFdSocket::new("vcan1").unwrap();
    let mut transport = CanTransport::new(socket).unwrap();

    let data: Vec<u8> = (1..(MESSAGE_SIZE + 1) as u8).collect();
    let data: [u8; MESSAGE_SIZE] = data.try_into().unwrap();
    let message = TestMessage::new(Priority::High, 2.try_into().unwrap(), None, data).unwrap();

    match transport.publish(&message).await {
        Ok(_) => println!("Message sent successfully"),
        Err(e) => println!("Failed to send message: {}", e),
    }
}

pub struct TestMessage {
    priority: Priority,
    subject: CanSubjectId,
    source: Option<CanNodeId>,
    payload: [u8; MESSAGE_SIZE],
}

impl TestMessage {
    pub fn new(
        priority: Priority,
        subject: CanSubjectId,
        source: Option<CanNodeId>,
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

impl Message<CanSubjectId, CanNodeId> for TestMessage {
    const SIZE: usize = MESSAGE_SIZE;

    fn source(&self) -> Option<CanNodeId> {
        self.source
    }

    fn subject(&self) -> CanSubjectId {
        self.subject
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn data(&self) -> &[u8] {
        &self.payload
    }
}
