#![no_std]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![allow(async_fn_in_trait)]

mod can;
pub use can::Can;

mod can_id;
pub use can_id::CanId;

mod can_transfer_id;
pub use can_transfer_id::CanTransferId;

mod error;
pub use error::{CanError, CanResult};

mod frame;
pub use frame::Frame;

mod message_can_id;
pub use message_can_id::MessageCanId;

mod service_can_id;
pub use service_can_id::ServiceCanId;

#[cfg(test)]
pub(crate) mod test;

mod transport;
pub use transport::CanTransport;

/// Payload size for CAN 2.0
pub const CLASSIC_PAYLOAD_SIZE: usize = 8;

/// Payload size for CAN FD
pub const FD_PAYLOAD_SIZE: usize = 64;
