use cyphal::{CyphalError, CyphalResult, SubjectId};

use crate::UdpSubjectId;

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

    /// Returns a `u32` representation of the group address
    pub fn as_raw(&self) -> u32 {
        0xEF000000 | self.subject.value() as u32
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
