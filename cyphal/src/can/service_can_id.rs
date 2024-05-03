use crate::{can::CanId, NodeId, Priority};
use embedded_can::{ExtendedId, Id};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct ServiceCanId {
    anonymous: bool,
    destination: u8,
    priority: Priority,
    service_id: u8,
    source: NodeId,
}

impl ServiceCanId {
    pub fn anonymous(&self) -> bool {
        self.anonymous
    }

    pub fn destination(&self) -> u8 {
        self.destination
    }

    pub fn service_id(&self) -> u8 {
        self.service_id
    }
}

impl CanId for ServiceCanId {
    fn priority(&self) -> Priority {
        self.priority
    }

    fn is_service(&self) -> bool {
        true
    }

    fn source(&self) -> NodeId {
        self.source
    }

    fn as_raw(&self) -> u32 {
        todo!()
    }
}

impl Into<Id> for ServiceCanId {
    fn into(self) -> Id {
        Id::Extended(ExtendedId::new(self.as_raw()).unwrap())
    }
}
