mod can_id;
use can_id::CanId;

mod error;
pub use error::{CanError, CanResult};

mod message_can_id;
pub use message_can_id::MessageCanId;

mod service_can_id;
pub use service_can_id::ServiceCanId;

#[cfg(feature = "socketcan")]
pub mod socketcan;

mod transport;
pub use transport::CanTransport;

mod can_transfer_id;
pub use can_transfer_id::CanTransferId;

use crate::TransferId;

fn tail_byte(is_start: bool, is_end: bool, toggle: bool, transfer_id: CanTransferId) -> u8 {
    let mut tail_byte = transfer_id.value();
    if is_start {
        tail_byte = tail_byte | 0x80;
    }
    if is_end {
        tail_byte = tail_byte | 0x40;
    }
    if toggle {
        tail_byte = tail_byte | 0x20;
    }

    tail_byte
}

pub trait Can {
    /// Associated frame type.
    type Frame: embedded_can::Frame;

    /// Associated error type.
    type Error: embedded_can::Error;

    /// Puts a frame in the transmit buffer. Blocks until space is available in
    /// the transmit buffer.
    fn transmit(&mut self, frame: &Self::Frame) -> CanResult<()>;

    /// Blocks until a frame was received or an error occurred.
    fn receive(&mut self) -> CanResult<Self::Frame>;
}
