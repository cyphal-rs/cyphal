use crate::CanResult;

pub trait Can<const MAX_PAYLOAD_SIZE: usize> {
    const MAX_PAYLOAD_SIZE: usize = MAX_PAYLOAD_SIZE;

    /// Associated frame type.
    type Frame: crate::Frame<MAX_PAYLOAD_SIZE>;

    /// Puts a frame in the transmit buffer. Blocks until space is available in
    /// the transmit buffer.
    fn transmit(&mut self, frame: &Self::Frame) -> CanResult<()>;

    /// Blocks until a frame was received or an error occurred.
    fn receive(&mut self) -> CanResult<Self::Frame>;
}
