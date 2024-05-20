#![no_std]
#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
#![allow(async_fn_in_trait)]

mod error;
pub use error::{CyphalError, CyphalResult};

mod message;
pub use message::Message;

mod node_id;
pub use node_id::NodeId;

mod priority;
pub use priority::Priority;

mod request;
pub use request::Request;

mod response;
pub use response::Response;

mod router;
pub use router::Router;

mod service_id;
pub use service_id::ServiceId;

mod subject_id;
pub use subject_id::SubjectId;

#[cfg(test)]
pub(crate) mod test;

mod transfer_id;
pub use transfer_id::TransferId;

mod transport;
pub use transport::Transport;
