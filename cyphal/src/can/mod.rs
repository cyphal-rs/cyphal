mod can_id;
use can_id::CanId;

mod error;
pub use error::{CanError, CanResult};

mod message_can_id;
pub use message_can_id::MessageCanId;

mod service_can_id;
pub use service_can_id::ServiceCanId;

#[cfg(feature = "socketcan")]
mod socketcan;
#[cfg(feature = "socketcan")]
pub use socketcan::Socketcan;

mod transport;
pub use transport::CanTransport;

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
