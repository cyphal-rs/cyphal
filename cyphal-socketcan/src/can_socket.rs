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
    use crate::{
        test::{MultiFrameMessage, SingleFrameMessage},
        CanSocket,
    };
    use cyphal::{Priority, Transport};
    use cyphal_can::CanTransport;

    #[test]
    #[ignore]
    fn publish() {
        let socket = CanSocket::new("vcan0").unwrap();
        let mut transport = CanTransport::new(socket).unwrap();

        let data: Vec<u8> = (1..3).collect();
        let data: [u8; 2] = data.try_into().unwrap();
        let message = SingleFrameMessage::new(Priority::Nominal, 1, None, data).unwrap();
        transport.publish(&message).unwrap();
    }

    #[test]
    #[ignore]
    fn publish_multi_frame() {
        let socket = CanSocket::new("vcan0").unwrap();
        let mut transport = CanTransport::new(socket).unwrap();

        let data: Vec<u8> = (1..66).collect();
        let data: [u8; 65] = data.try_into().unwrap();
        let message = MultiFrameMessage::new(Priority::Nominal, 1, None, data).unwrap();
        transport.publish(&message).unwrap();
    }
}
