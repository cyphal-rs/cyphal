#![no_std]

#[cfg(feature = "can")]
pub mod can;

#[cfg(feature = "crc")]
pub mod crc;

mod error;
pub use error::{CyphalError, CyphalResult};

mod message;
pub use message::Message;

mod priority;
pub use priority::Priority;

mod node;
pub use node::Node;

mod service;
pub use service::{Request, Response};

mod transport;
pub use transport::Transport;

pub type SubjectId = u64;
pub type TransferId = u64;
pub type NodeId = u8;
pub type ServiceId = u16;
