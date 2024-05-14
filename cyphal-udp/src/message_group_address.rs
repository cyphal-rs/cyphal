use crate::UdpSubjectId;
use core::net::Ipv4Addr;
use cyphal::{CyphalError, CyphalResult, SubjectId};

/// Represents a Message IP multicast group address
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct MessageGroupAddress {
    subject: UdpSubjectId,
}

impl MessageGroupAddress {
    /// Returns the Message Subject ID
    pub fn subject(&self) -> UdpSubjectId {
        self.subject
    }
}

impl TryFrom<u32> for MessageGroupAddress {
    type Error = CyphalError;

    fn try_from(address: u32) -> CyphalResult<Self> {
        // verify that the predefined bits are correct
        if ((address & 0xEF0000) != 0xEF0000) || (address & 0xFE00) > 0 || (address & 0xE000) > 1 {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self {
            subject: ((address & 0x1FFF) as u16).try_into()?,
        })
    }
}

impl Into<Ipv4Addr> for MessageGroupAddress {
    fn into(self) -> Ipv4Addr {
        let id = self.subject.value();
        Ipv4Addr::new(0xEF, 0x00, (id >> 8) as u8, (id & 0xFF) as u8)
    }
}
