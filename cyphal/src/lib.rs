#![no_std]

extern crate alloc;
use alloc::boxed::Box;

#[cfg(feature = "can")]
pub mod can;

#[cfg(any(feature = "serial", feature = "udp"))]
pub mod crc;

#[cfg(feature = "serial")]
pub mod serial;

#[cfg(feature = "udp")]
pub mod udp;

mod error;
pub use error::{CyphalError, Result};

mod message_transfer;
pub use message_transfer::MessageTransfer;

mod priority;
pub use priority::Priority;

mod node;
pub use node::Node;

mod service_transfer;
pub use service_transfer::ServiceTransfer;

mod transfer;
pub use transfer::{Transfer, TransferKind};

mod transport;
pub use transport::Transport;

pub type SubjectId = u64;
pub type TransferId = u64;
pub type NodeId = u64;
