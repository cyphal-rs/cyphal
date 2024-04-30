#![no_std]

#[cfg(feature = "can")]
pub mod can;

#[cfg(feature = "crc")]
pub mod crc;

mod error;
pub use error::{CyphalError, CyphalResult};

mod message_transfer;
pub use message_transfer::MessageTransfer;

mod priority;
pub use priority::Priority;

mod node;
pub use node::Node;

// mod service_transfer;
// pub use service_transfer::ServiceTransfer;

mod transport;
pub use transport::Transport;

pub type SubjectId = u64;
pub type TransferId = u64;
pub type NodeId = u64;
