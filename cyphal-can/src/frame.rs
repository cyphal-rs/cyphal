use crate::{CanId, CanResult, CanTransferId};

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

    /// Returns true if it's a Single Frame Transfer
    fn is_single_trame_transfer(&self) -> bool {
        self.dlc() == MAX_PAYLOAD_SIZE && (self.data()[MAX_PAYLOAD_SIZE - 1] & 0xE0) == 0xE0
    }

    /// Returns true if it's the start of a Transfer
    fn is_start_of_transfer(&self) -> bool {
        if self.is_single_trame_transfer() {
            return true;
        }

        self.data()[self.dlc() - 1] & 0x80 != 0
    }

    /// Returns true if it's the end of a Transfer
    fn is_end_of_transfer(&self) -> bool {
        if self.is_single_trame_transfer() {
            return true;
        }

        self.data()[self.dlc() - 1] & 0x40 != 0
    }

    /// Returns true if the Toggle bit is set
    fn is_toggle_bit_set(&self) -> bool {
        if self.is_single_trame_transfer() {
            return true;
        }

        self.data()[self.dlc() - 1] & 0x20 != 0
    }

    /// Returns the Transfer ID
    fn transfer_id(&self) -> CanTransferId {
        let value = if self.is_single_trame_transfer() {
            self.data()[MAX_PAYLOAD_SIZE - 1] & 0x0F
        } else {
            self.data()[self.dlc() - 1] & 0xF
        };

        CanTransferId::new(value).unwrap()
    }
}
