use crate::SocketcanFrame;
use cyphal_can::{Can, CanError, CanId, CanResult, Frame};
use embedded_can::{Frame as EmbeddedFrame, Id};
use socketcan::{CanFrame, Socket};

pub struct CanSocket {
    socket: socketcan::CanSocket,
}

impl CanSocket {
    pub fn new(socket: socketcan::CanSocket) -> Self {
        CanSocket { socket }
    }
}

impl Can<8> for CanSocket {
    type Frame = SocketcanFrame;

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
                        Id::Extended(e) => Ok(SocketcanFrame::new(
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
