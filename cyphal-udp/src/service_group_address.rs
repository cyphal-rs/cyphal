use crate::UdpNodeId;
use cyphal::{CyphalError, CyphalResult, NodeId};

/// Represents a Service IP multicast group address
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct ServiceGroupAddress {
    destination: UdpNodeId,
}

impl ServiceGroupAddress {
    /// Returns the Service Destination Node ID
    pub fn destination(&self) -> UdpNodeId {
        self.destination
    }

    /// Returns a `u32` representation of the group address
    pub fn as_raw(&self) -> u32 {
        0xEF010000 | self.destination.value() as u32
    }
}

impl TryFrom<u32> for ServiceGroupAddress {
    type Error = CyphalError;

    fn try_from(address: u32) -> CyphalResult<Self> {
        // verify that the predefined bits are correct
        if ((address & 0xEF0000) != 0xEF0000) || (address & 0xFE00) == 0 || (address & 0xE000) > 1 {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self {
            destination: ((address & 0xFFFF) as u16).try_into()?,
        })
    }
}
