use crate::Frame;
use cyphal_can::{Can, CanError, CanId, CanResult, Frame as CyphalFrame, CLASSIC_PAYLOAD_SIZE};
use embedded_can::{Frame as _, Id};
use socketcan::{async_std::CanSocket as Socket, CanFrame};

/// Represents a CAN 2.0 Socket
pub struct CanSocket {
    socket: Socket,
}

impl CanSocket {
    /// Constructs a new CAN 2.0 Socket
    pub fn new(iface: &str) -> CanResult<Self> {
        match Socket::open(iface) {
            Ok(socket) => Ok(CanSocket { socket }),
            Err(_) => Err(CanError::Other),
        }
    }
}

impl Can<CLASSIC_PAYLOAD_SIZE> for CanSocket {
    type Frame = Frame;

    async fn transmit(&mut self, frame: &Self::Frame) -> CanResult<()> {
        let result = self.socket.write_frame(frame.inner_frame()).await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(CanError::Other),
        }
    }

    async fn receive(&mut self) -> CanResult<Self::Frame> {
        let result = self.socket.read_frame().await;

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

    #[async_std::test]
    #[ignore]
    async fn publish() {
        let socket = CanSocket::new("vcan0").unwrap();
        let mut transport = CanTransport::new(socket).unwrap();

        let data: Vec<u8> = (1..3).collect();
        let data: [u8; 2] = data.try_into().unwrap();
        let message = SingleFrameMessage::new(Priority::Nominal, 1, None, data).unwrap();
        transport.publish(&message).await.unwrap();
    }

    #[async_std::test]
    #[ignore]
    async fn publish_multi_frame() {
        let socket = CanSocket::new("vcan0").unwrap();
        let mut transport = CanTransport::new(socket).unwrap();

        let data: Vec<u8> = (1..66).collect();
        let data: [u8; 65] = data.try_into().unwrap();
        let message = MultiFrameMessage::new(Priority::Nominal, 1, None, data).unwrap();
        transport.publish(&message).await.unwrap();
    }
}
