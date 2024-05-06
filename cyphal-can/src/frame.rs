use crate::{CanId, CanResult};

/// A CAN Frame
pub trait Frame<const MAX_PAYLOAD_SIZE: usize>: Sized {
    const PAYLOAD_SIZE: usize = MAX_PAYLOAD_SIZE;

    /// Creates a new frame.
    ///
    /// This will return `None` if the data slice is too long.
    fn new(id: impl Into<CanId>, data: &[u8]) -> CanResult<Self>;

    /// Returns the frame identifier.
    fn id(&self) -> CanId;

    /// Returns the data length code (DLC) which is in the range 0..8.
    ///
    /// For data frames the DLC value always matches the length of the data.
    /// Remote frames do not carry any data, yet the DLC can be greater than 0.
    fn dlc(&self) -> usize;

    /// Returns the frame data (0..8 bytes in length).
    fn data(&self) -> &[u8];
}
