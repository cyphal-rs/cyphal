use crate::SocketcanFdFrame;
use cyphal_can::{Can, CanError, CanId, CanResult, Frame};
use embedded_can::{Frame as EmbeddedFrame, Id};
use socketcan::{CanAnyFrame, Socket};

pub struct CanFdSocket {
    socket: socketcan::CanFdSocket,
}

impl CanFdSocket {
    pub fn new(socket: socketcan::CanFdSocket) -> Self {
        CanFdSocket { socket }
    }
}

impl Can<64> for CanFdSocket {
    type Frame = SocketcanFdFrame;

    fn transmit(&mut self, frame: &Self::Frame) -> CanResult<()> {
        let result = self
            .socket
            .write_frame(&CanAnyFrame::Normal(*frame.inner_frame()));

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
                        Id::Extended(e) => Ok(SocketcanFdFrame::new(
                            CanId::new(e.as_raw()).unwrap(),
                            f.data(),
                        )
                        .unwrap()),
                    }
                }
                _ => Err(CanError::Other),
            },
            Err(_) => Err(CanError::Other),
        }
    }
}
