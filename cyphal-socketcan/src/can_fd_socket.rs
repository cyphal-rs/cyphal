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

    async fn transmit(&mut self, frame: &Self::Frame) -> CanResult<()> {
        let result = self
            .socket
            .write_frame(&CanAnyFrame::Fd(*frame.inner_frame()));

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(CanError::Other),
        }
    }

    async fn receive(&mut self) -> CanResult<Self::Frame> {
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
    use crate::{
        test::{MultiFrameMessage, SingleFrameMessage},
        CanFdSocket,
    };
    use cyphal::{Priority, Transport};
    use cyphal_can::CanTransport;

    #[async_std::test]
    #[ignore]
    async fn publish_single_frame() {
        let socket = CanFdSocket::new("vcan1").unwrap();
        let mut transport = CanTransport::new(socket).unwrap();

        let data: Vec<u8> = (1..3).collect();
        let data: [u8; 2] = data.try_into().unwrap();
        let message = SingleFrameMessage::new(Priority::Nominal, 1, None, data).unwrap();
        transport.publish(&message).await.unwrap();
    }

    #[async_std::test]
    #[ignore]
    async fn publish_multi_frame() {
        let socket = CanFdSocket::new("vcan1").unwrap();
        let mut transport = CanTransport::new(socket).unwrap();

        let data: Vec<u8> = (1..66).collect();
        let data: [u8; 65] = data.try_into().unwrap();
        let message = MultiFrameMessage::new(Priority::Nominal, 1, None, data).unwrap();
        transport.publish(&message).await.unwrap();
    }
}
