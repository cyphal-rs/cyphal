use crate::Frame;
use cyphal_can::{Can, CanError, CanId, CanResult, Frame as CyphalFrame, CLASSIC_PAYLOAD_SIZE};
use embedded_can::{Frame as EmbeddedFrame, Id};
use socketcan::{CanFrame, CanSocket as SocketcanSocket, Socket};

/// Represents a CAN 2.0 Socket
pub struct CanSocket {
    socket: SocketcanSocket,
}

impl CanSocket {
    /// Constructs a new CAN 2.0 Socket
    pub fn new(iface: &str) -> CanResult<Self> {
        match SocketcanSocket::open(iface) {
            Ok(socket) => Ok(CanSocket { socket }),
            Err(_) => Err(CanError::Other),
        }
    }
}

impl Can<CLASSIC_PAYLOAD_SIZE> for CanSocket {
    type Frame = Frame;

    fn transmit(&mut self, frame: &Self::Frame) -> CanResult<()> {
        let result = self.socket.write_frame(frame.inner_frame());

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(CanError::Other),
        }
    }

    fn receive(&mut self) -> CanResult<Self::Frame> {
        let result = self.socket.read_frame();
        match result {
            Ok(f) => match f {
                CanFrame::Data(f) => {
                    let id = f.id();
                    match id {
                        Id::Standard(_) => Err(CanError::Other),
                        Id::Extended(e) => {
                            Ok(Frame::new(CanId::new(e.as_raw()).unwrap(), f.data()).unwrap())
                        }
                    }
                }
                _ => Err(CanError::Other),
            },
            Err(_) => Err(CanError::Other),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::CanSocket;
    use cyphal::{CyphalResult, Message, NodeId, Priority, SubjectId, Transport};
    use cyphal_can::CanTransport;

    const SINGLE_SIZE: usize = 2;
    const MULTI_SIZE: usize = 65;

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
        type Payload = [u8; SINGLE_SIZE];

        fn priority(&self) -> Priority {
            self.priority
        }

        fn subject(&self) -> SubjectId {
            self.subject
        }

        fn source(&self) -> Option<NodeId> {
            self.source
        }

        fn payload(&self) -> &[u8] {
            &self.payload
        }
    }

    pub struct MultiFrameMessage {
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
        payload: [u8; MULTI_SIZE],
    }

    impl Message<MULTI_SIZE> for MultiFrameMessage {
        type Payload = [u8; MULTI_SIZE];

        fn source(&self) -> Option<NodeId> {
            self.source
        }

        fn subject(&self) -> SubjectId {
            self.subject
        }

        fn priority(&self) -> Priority {
            self.priority
        }

        fn payload(&self) -> &[u8] {
            &self.payload
        }
    }

    #[test]
    #[ignore]
    fn publish() {
        let socket = CanSocket::new("vcan0").unwrap();
        let mut transport = CanTransport::new(socket).unwrap();
        let message =
            SingleFrameMessage::new(Priority::Nominal, 1, None, 0x1234_u16.to_be_bytes()).unwrap();
        transport.publish(&message).unwrap();
    }
}
