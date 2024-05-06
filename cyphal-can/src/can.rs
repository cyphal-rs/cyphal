use crate::CanResult;

pub trait Can {
    /// Associated frame type.
    type Frame: crate::Frame;

    /// Puts a frame in the transmit buffer. Blocks until space is available in
    /// the transmit buffer.
    fn transmit(&mut self, frame: &Self::Frame) -> CanResult<()>;

    /// Blocks until a frame was received or an error occurred.
    fn receive(&mut self) -> CanResult<Self::Frame>;
}
