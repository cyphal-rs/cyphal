use super::CanError;
use socketcan::{CanAnyFrame, CanFdFrame, CanFdSocket, Socket};

pub struct Socketcan {
    socket: CanFdSocket,
}

impl super::Can for Socketcan {
    type Frame = CanFdFrame;

    type Error = CanError;

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
