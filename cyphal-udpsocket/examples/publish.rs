use cyphal::{CyphalResult, Message, Priority, Transport};
use cyphal_udp::{UdpNodeId, UdpSubjectId, UdpTransport};
use cyphal_udpsocket::UdpSocket;

const MESSAGE_SIZE: usize = 65;
const MAX_PAYLOAD_SIZE: usize = 565;

#[async_std::main]
async fn main() {
    let socket: UdpSocket<MAX_PAYLOAD_SIZE> = UdpSocket::new("127.0.0.1:8080").unwrap();
    let mut transport = UdpTransport::new(socket).unwrap();

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
    subject: UdpSubjectId,
    source: Option<UdpNodeId>,
    payload: [u8; MESSAGE_SIZE],
}

impl TestMessage {
    pub fn new(
        priority: Priority,
        subject: UdpSubjectId,
        source: Option<UdpNodeId>,
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

impl Message<MESSAGE_SIZE, UdpSubjectId, UdpNodeId> for TestMessage {
    fn source(&self) -> Option<UdpNodeId> {
        self.source
    }

    fn subject(&self) -> UdpSubjectId {
        self.subject
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn data(&self) -> &[u8; MESSAGE_SIZE] {
        &self.payload
    }
}
