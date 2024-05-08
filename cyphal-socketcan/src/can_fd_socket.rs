use crate::FdFrame;
use cyphal_can::{Can, CanError, CanId, CanResult, Frame as CyphalFrame, FD_PAYLOAD_SIZE};
use embedded_can::{Frame as EmbeddedFrame, Id};
use socketcan::{CanAnyFrame, CanFdSocket as SocketcanFdSocket, Socket};

/// Represents a CAN FD Socket
pub struct CanFdSocket {
    socket: SocketcanFdSocket,
}

impl CanFdSocket {
    /// Constructs a new CAN FD Socket
    pub fn new(iface: &str) -> CanResult<Self> {
        match SocketcanFdSocket::open(iface) {
            Ok(socket) => Ok(CanFdSocket { socket }),
            Err(_) => Err(CanError::Other),
        }
    }
}

impl Can<FD_PAYLOAD_SIZE> for CanFdSocket {
    type Frame = FdFrame;

    fn transmit(&mut self, frame: &Self::Frame) -> CanResult<()> {
        let result = self
            .socket
            .write_frame(&CanAnyFrame::Fd(*frame.inner_frame()));

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(CanError::Other),
        }
    }

    fn receive(&mut self) -> CanResult<Self::Frame> {
        let result = self.socket.read_frame();
        match result {
            Ok(f) => match f {
                CanAnyFrame::Normal(f) => {
                    let id = f.id();
                    match id {
                        Id::Standard(_) => Err(CanError::Other),
                        Id::Extended(e) => {
                            Ok(FdFrame::new(CanId::new(e.as_raw()).unwrap(), f.data()).unwrap())
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
    use crate::CanFdSocket;
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
    fn publish_single_frame() {
        let socket = CanFdSocket::new("vcan1").unwrap();
        let mut transport = CanTransport::new(socket).unwrap();

        let data: Vec<u8> = (1..3).collect();
        let data: [u8; 2] = data.try_into().unwrap();
        let message = SingleFrameMessage::new(Priority::Nominal, 1, None, data).unwrap();
        transport.publish(&message).unwrap();
    }

    #[test]
    #[ignore]
    fn publish_multi_frame() {
        let socket = CanFdSocket::new("vcan1").unwrap();
        let mut transport = CanTransport::new(socket).unwrap();

        let data: Vec<u8> = (1..66).collect();
        let data: [u8; 65] = data.try_into().unwrap();
        let message = MultiFrameMessage::new(Priority::Nominal, 1, None, data).unwrap();
        transport.publish(&message).unwrap();
    }
}
