use crate::CanResult;

/// Trait representing a CAN interface
pub trait Can<const MAX_PAYLOAD_SIZE: usize> {
    /// Associated frame type.
    type Frame: crate::Frame<MAX_PAYLOAD_SIZE>;

    /// Puts a frame in the transmit buffer. Blocks until space is available in the transmit buffer.
    fn transmit(&mut self, frame: &Self::Frame) -> CanResult<()>;

    /// Blocks until a frame is received or an error occurres.
    fn receive(&mut self) -> CanResult<Self::Frame>;
}
