use crate::FdFrame;
use cyphal_can::{Can, CanError, CanId, CanResult, Frame as CyphalFrame, FD_PAYLOAD_SIZE};
use embedded_can::{Frame as EmbeddedFrame, Id};
use socketcan::{async_std::CanFdSocket as FdSocket, CanAnyFrame};

/// Represents a CAN FD Socket
pub struct CanFdSocket {
    socket: FdSocket,
}

impl CanFdSocket {
    /// Constructs a new CAN FD Socket
    pub fn new(iface: &str) -> CanResult<Self> {
        match FdSocket::open(iface) {
            Ok(socket) => Ok(CanFdSocket { socket }),
            Err(_) => Err(CanError::Other),
        }
    }
}

impl Can<FD_PAYLOAD_SIZE> for CanFdSocket {
    type Frame = FdFrame;

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
    use crate::{
        test::{MultiFrameMessage, SingleFrameMessage},
        CanFdSocket,
    };
    use cyphal::{Priority, Transport};
    use cyphal_can::CanTransport;

    #[async_std::test]
    #[ignore = "need to have vcan setup"]
    async fn publish_single_frame() {
        let socket = CanFdSocket::new("vcan1").unwrap();
        let mut transport = CanTransport::new(socket).unwrap();

        let data: Vec<u8> = (1..3).collect();
        let data: [u8; 2] = data.try_into().unwrap();
        let message = SingleFrameMessage::new(Priority::Nominal, 1, None, data).unwrap();
        transport.publish(&message).await.unwrap();
    }

    #[async_std::test]
    #[ignore = "need to have vcan setup"]
    async fn publish_multi_frame() {
        let socket = CanFdSocket::new("vcan1").unwrap();
        let mut transport = CanTransport::new(socket).unwrap();

        let data: Vec<u8> = (1..66).collect();
        let data: [u8; 65] = data.try_into().unwrap();
        let message = MultiFrameMessage::new(Priority::High, 2, Some(123), data).unwrap();
        transport.publish(&message).await.unwrap();
    }
}
