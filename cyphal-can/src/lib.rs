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

mod can_node_id;
pub use can_node_id::CanNodeId;

mod can_service_id;
pub use can_service_id::CanServiceId;

mod can_subject_id;
pub use can_subject_id::CanSubjectId;

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
