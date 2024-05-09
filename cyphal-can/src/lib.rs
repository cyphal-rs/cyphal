//! Open Cyphal CAN Transport Rust Implementation
#![no_std]
#![allow(async_fn_in_trait)]
#![warn(missing_docs)]

/// Payload size for CAN 2.0
pub const CLASSIC_PAYLOAD_SIZE: usize = 8;

/// Payload size for CAN FD
pub const FD_PAYLOAD_SIZE: usize = 64;

mod can;
pub use can::Can;

mod can_id;
pub use can_id::CanId;

mod error;
pub use error::{CanError, CanResult};

mod frame;
pub use frame::Frame;

// mod inbound_queue;
// pub(crate) use inbound_queue::InboundQueue;

mod message_can_id;
pub use message_can_id::MessageCanId;

mod outbound_queue;
pub(crate) use outbound_queue::OutboundQueue;

mod service_can_id;
pub use service_can_id::ServiceCanId;

#[cfg(test)]
pub(crate) mod test;

mod transport;
pub use transport::CanTransport;

mod can_transfer_id;
pub use can_transfer_id::CanTransferId;
