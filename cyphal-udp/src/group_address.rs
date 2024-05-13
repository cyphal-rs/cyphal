use crate::{MessageGroupAddress, ServiceGroupAddress};
use cyphal::{CyphalError, CyphalResult};

/// Represents an IP Multicast Group Address
#[derive(Debug, Copy, Clone)]
pub enum GroupAddress {
    /// Represents a Message IP multicast group address
    Message(MessageGroupAddress),

    /// Represents a Service IP multicast group address
    Service(ServiceGroupAddress),
}

impl GroupAddress {
    /// Constructs a new `GroupAddress`
    pub fn new(address: u32) -> CyphalResult<Self> {
        // verify that the predefined bits are correct
        if ((address & 0xEF0000) != 0xEF0000) || (address & 0xFE00) > 1 || (address & 0xE000) > 1 {
            return Err(CyphalError::OutOfRange);
        }

        if (address & 0x100) == 0 {
            match MessageGroupAddress::try_from(address) {
                Ok(m) => Ok(GroupAddress::Message(m)),
                Err(e) => Err(e),
            }
        } else {
            match ServiceGroupAddress::try_from(address) {
                Ok(s) => Ok(GroupAddress::Service(s)),
                Err(e) => Err(e),
            }
        }
    }

    /// Returns a `u32` representation of the group address
    pub fn as_raw(&self) -> u32 {
        match self {
            GroupAddress::Message(m) => m.as_raw(),
            GroupAddress::Service(s) => s.as_raw(),
        }
    }
}

impl From<MessageGroupAddress> for GroupAddress {
    #[inline]
    fn from(address: MessageGroupAddress) -> Self {
        GroupAddress::Message(address)
    }
}

impl From<ServiceGroupAddress> for GroupAddress {
    #[inline]
    fn from(address: ServiceGroupAddress) -> Self {
        GroupAddress::Service(address)
    }
}
