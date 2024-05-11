use crate::{UdpError, UdpResult};
use cyphal::Priority;

/// Represents a payload header used for services
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct ServiceHeader {
    version: u8,
    priority: Priority,
    source: u16,
    destination: u16,
    data_specifier_snm: u16,
    transfer: u64,
    index: u32,
    user_data: u16,
    crc16: [u8; 2],
}

impl ServiceHeader {
    /// Returns the priority of the service
    pub fn priority(&self) -> Priority {
        self.priority
    }

    /// Returns a `u32` representation of the service header
    pub fn as_raw(&self) -> [u8; 24] {
        todo!()
    }
}

impl TryFrom<&[u8; 24]> for ServiceHeader {
    type Error = UdpError;

    fn try_from(_value: &[u8; 24]) -> UdpResult<Self> {
        todo!()
    }
}
