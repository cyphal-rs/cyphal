//! Open Cyphal CAN Transport Rust Implementation
#![no_std]
#![allow(async_fn_in_trait)]
#![warn(missing_docs)]

mod error;
pub use error::{CyphalError, CyphalResult};

mod message;
pub use message::Message;

mod priority;
pub use priority::Priority;

mod request;
pub use request::Request;

mod response;
pub use response::Response;

#[cfg(test)]
pub(crate) mod test;

mod transfer_id;
pub use transfer_id::TransferId;

mod transport;
pub use transport::Transport;

/// Represents a Subject ID
pub type SubjectId = u16;

/// Represents a Node ID
pub type NodeId = u8;

/// Represents a Service ID
pub type ServiceId = u16;
