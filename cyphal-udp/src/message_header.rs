use crate::{UdpError, UdpNodeId, UdpResult};
use cyphal::Priority;

/// Represents a UDP payload header used for messages
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct MessageHeader {
    version: u8,
    priority: Priority,
    source: UdpNodeId,
    destination: u16,
    data_specifier_snm: u16,
    transfer: u64,
    index: u32,
    user_data: u16,
    crc16: [u8; 2],
}

impl MessageHeader {
    /// Returns the priority of the message
    pub fn priority(&self) -> Priority {
        self.priority
    }

    /// Returns a `&[u8; 24]` representation of the message header
    pub fn as_raw(&self) -> [u8; 24] {
        todo!()
    }
}

impl TryFrom<&[u8; 24]> for MessageHeader {
    type Error = UdpError;

    fn try_from(_value: &[u8; 24]) -> UdpResult<Self> {
        todo!()
    }
}
