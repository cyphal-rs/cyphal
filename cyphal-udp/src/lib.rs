#![no_std]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![allow(async_fn_in_trait)]

/// Multigroup Address Port used by Cyphal
pub const MULTIGROUP_ADDRESS_PORT: u16 = 9382;

mod error;
pub use error::{UdpError, UdpResult};

mod group_address;
pub use group_address::GroupAddress;

mod header;
pub use header::Header;

mod message_group_address;
pub use message_group_address::MessageGroupAddress;

mod message_header;
pub use message_header::MessageHeader;

mod service_header;
pub use service_header::ServiceHeader;

mod service_group_address;
pub use service_group_address::ServiceGroupAddress;

mod transport;
pub use transport::UdpTransport;

mod udp;
pub use udp::Udp;

mod udp_node_id;
pub use udp_node_id::UdpNodeId;

mod udp_service_id;
pub use udp_service_id::UdpServiceId;

mod udp_subject_id;
pub use udp_subject_id::UdpSubjectId;

mod udp_transfer_id;
pub use udp_transfer_id::UdpTransferId;
