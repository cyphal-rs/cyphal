use crate::{CanId, CanResult};

/// A CAN Frame
pub trait Frame<const MAX_PAYLOAD_SIZE: usize>: Sized {
    /// Creates a new frame.
    ///
    /// This will return an error if the data slice is too long.
    fn new(id: impl Into<CanId>, data: &[u8]) -> CanResult<Self>;

    /// Returns the frame identifier.
    fn id(&self) -> CanId;

    /// Returns the data length code (DLC) which is in the range 0..8 for CAN 2.0 and 0..64 for CAN FD.
    fn dlc(&self) -> usize;

    /// Returns the frame data which is 0..8 bytes in length for CAN 2.0 and 0..64  bytes in length for CAN FD.
    fn data(&self) -> &[u8];
}
