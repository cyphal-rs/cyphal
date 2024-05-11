//! Open Cyphal UDP Rust Implementation
//!
#![no_std]
#![allow(async_fn_in_trait)]
#![warn(missing_docs)]

mod error;
pub use error::{UdpError, UdpResult};

mod transport;
pub use transport::UdpTransport;

mod udp;
pub use udp::Udp;
