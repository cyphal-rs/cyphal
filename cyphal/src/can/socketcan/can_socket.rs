use crate::can::{Can, CanError};
use socketcan::{CanFrame, Socket};

pub struct CanSocket {
    socket: socketcan::CanSocket,
}

impl CanSocket {
    pub fn new(socket: socketcan::CanSocket) -> Self {
        CanSocket { socket }
    }
}

impl Can for CanSocket {
    type Frame = CanFrame;

    type Error = CanError;

    fn is_fd(&self) -> bool {
        false
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
                CanFrame::Data(f) => Ok(Self::Frame::from(f)),
                _ => Err(CanError::Socketcan()),
            },
            Err(_) => Err(CanError::Socketcan()),
        }
    }
}
