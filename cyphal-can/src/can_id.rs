use cyphal::Priority;

use crate::{CanResult, MessageCanId, ServiceCanId};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CanId {
    Message(MessageCanId),

    Service(ServiceCanId),
}

impl CanId {
    pub fn new(id: u32) -> CanResult<CanId> {
        // check bit 25 to see what type of Id this is
        if (id & 0x0200_0000) == 0 {
            match MessageCanId::from_raw(id) {
                Ok(i) => Ok(CanId::Message(i)),
                Err(e) => Err(e),
            }
        } else {
            match ServiceCanId::from_raw(id) {
                Ok(i) => Ok(CanId::Service(i)),
                Err(e) => Err(e),
            }
        }
    }

    pub fn as_raw(&self) -> u32 {
        match *self {
            CanId::Message(m) => m.as_raw(),
            CanId::Service(s) => s.as_raw(),
        }
    }

    pub fn priority(&self) -> Priority {
        match *self {
            CanId::Message(m) => m.priority(),
            CanId::Service(s) => s.priority(),
        }
    }
}

impl From<MessageCanId> for CanId {
    #[inline]
    fn from(id: MessageCanId) -> Self {
        CanId::Message(id)
    }
}

impl From<ServiceCanId> for CanId {
    #[inline]
    fn from(id: ServiceCanId) -> Self {
        CanId::Service(id)
    }
}
