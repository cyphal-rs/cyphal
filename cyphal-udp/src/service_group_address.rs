use core::net::Ipv4Addr;
use cyphal::{CyphalError, CyphalResult, NodeId};

/// Represents a Service IP multicast group address
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct ServiceGroupAddress {
    destination: NodeId,
}

impl ServiceGroupAddress {
    /// Constructs a new Service Group Address
    pub fn new(destination: NodeId) -> Self {
        Self { destination }
    }

    /// Returns the Service Destination Node ID
    pub fn destination(&self) -> NodeId {
        self.destination
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
            destination: (address & 0xFFFF) as u16,
        })
    }
}

#[allow(clippy::from_over_into)]
impl Into<Ipv4Addr> for ServiceGroupAddress {
    fn into(self) -> Ipv4Addr {
        let id = self.destination;
        Ipv4Addr::new(0xEF, 0x01, (id >> 8) as u8, (id & 0xFF) as u8)
    }
}
