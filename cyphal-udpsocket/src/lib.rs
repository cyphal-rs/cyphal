#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![allow(async_fn_in_trait)]

mod udp_socket;
pub use udp_socket::UdpSocket;
