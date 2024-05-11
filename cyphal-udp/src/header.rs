use crate::{MessageHeader, ServiceHeader, UdpResult};
use core::cmp::Ordering;
use cyphal::Priority;

/// Represents a payload header
#[derive(Debug, Copy, Clone)]
pub enum Header {
    /// Payload header used for messages
    Message(MessageHeader),

    /// Payload header used for services
    Service(ServiceHeader),
}

impl Header {
    /// Constructs a new `Header`
    pub fn new(header: &[u8; 24]) -> UdpResult<Header> {
        // check bit 25 to see what type of Id this is
        if (header[9] & 0x01) == 0 {
            match MessageHeader::try_from(header) {
                Ok(header) => Ok(Header::Message(header)),
                Err(e) => Err(e),
            }
        } else {
            match ServiceHeader::try_from(header) {
                Ok(header) => Ok(Header::Service(header)),
                Err(e) => Err(e),
            }
        }
    }

    /// Returns a `&[u8; 24]` representation of the `Header`
    pub fn as_raw(&self) -> [u8; 24] {
        match *self {
            Header::Message(m) => m.as_raw(),
            Header::Service(s) => s.as_raw(),
        }
    }

    /// Returns the `Priority` of the `Header``
    pub fn priority(&self) -> Priority {
        match *self {
            Header::Message(m) => m.priority(),
            Header::Service(s) => s.priority(),
        }
    }
}

impl From<MessageHeader> for Header {
    #[inline]
    fn from(id: MessageHeader) -> Self {
        Header::Message(id)
    }
}

impl From<ServiceHeader> for Header {
    #[inline]
    fn from(id: ServiceHeader) -> Self {
        Header::Service(id)
    }
}

impl Ord for Header {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_raw().cmp(&other.as_raw())
    }
}

impl PartialOrd for Header {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Header {
    fn eq(&self, other: &Self) -> bool {
        self.as_raw() == other.as_raw()
    }
}

impl Eq for Header {}
