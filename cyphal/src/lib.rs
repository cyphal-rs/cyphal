#![no_std]

#[cfg(all(feature = "can", feature = "canfd"))]
compile_error!("feature \"can\" and feature \"canfd\" cannot be enabled at the same time");

#[cfg(any(feature = "can", feature = "canfd"))]
pub mod can;

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

mod transfer_id;
pub use transfer_id::TransferId;

mod transport;
pub use transport::Transport;

pub type SubjectId = u64;
pub type NodeId = u8;
pub type ServiceId = u16;
