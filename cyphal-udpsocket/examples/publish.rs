use cyphal::{CyphalResult, Message, Priority, SubjectId, Transport};
use cyphal_udp::{MessageGroupAddress, UdpNodeId, UdpSubjectId, UdpTransport};
use cyphal_udpsocket::UdpSocket;

const MESSAGE_SIZE: usize = 65;
const MAX_PAYLOAD_SIZE: usize = 565;

#[async_std::main]
async fn main() {
    let subject = UdpSubjectId::new(123).unwrap();
    let address = MessageGroupAddress::new(subject);
    let socket: UdpSocket<MAX_PAYLOAD_SIZE> = UdpSocket::new(address.into()).unwrap();
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

impl Message<UdpSubjectId, UdpNodeId> for TestMessage {
    const SIZE: usize = MESSAGE_SIZE;

    fn source(&self) -> Option<UdpNodeId> {
        self.source
    }

    fn subject(&self) -> UdpSubjectId {
        self.subject
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn data(&self) -> &[u8] {
        &self.payload
    }
}
