use crate::can::{Can, CanError};
use socketcan::{CanAnyFrame, CanFdFrame, Socket};

pub struct CanFdSocket {
    socket: socketcan::CanFdSocket,
}

impl CanFdSocket {
    pub fn new(socket: socketcan::CanFdSocket) -> Self {
        CanFdSocket { socket }
    }
}

impl Can for CanFdSocket {
    type Frame = CanFdFrame;

    type Error = CanError;

    fn is_fd(&self) -> bool {
        true
    }

    fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error> {
        let result = self.socket.write_frame(frame);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(CanError::Socketcan()),
        }
    }

    fn receive(&mut self) -> Result<Self::Frame, Self::Error> {
        let result = self.socket.read_frame();

        match result {
            Ok(f) => match f {
                CanAnyFrame::Fd(fd) => Ok(fd),
                _ => Err(CanError::Socketcan()),
            },
            Err(_) => Err(CanError::Socketcan()),
        }
    }
}
