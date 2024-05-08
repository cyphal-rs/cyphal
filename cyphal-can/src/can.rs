use crate::CanResult;

/// Trait representing a CAN interface
pub trait Can<const MAX_PAYLOAD_SIZE: usize> {
    /// Maximum payload size supported by the CAN interface.  This will be 8 for CAN 2.0 or 64 for CAN FD.
    const MAX_PAYLOAD_SIZE: usize = MAX_PAYLOAD_SIZE;

    /// Associated frame type.
    type Frame: crate::Frame<MAX_PAYLOAD_SIZE>;

    /// Puts a frame in the transmit buffer. Blocks until space is available in the transmit buffer.
    fn transmit(&mut self, frame: &Self::Frame) -> CanResult<()>;

    /// Blocks until a frame is received or an error occurres.
    fn receive(&mut self) -> CanResult<Self::Frame>;
}
