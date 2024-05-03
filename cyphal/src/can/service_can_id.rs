use crate::{can::CanId, NodeId, Priority, ServiceId};
use embedded_can::{ExtendedId, Id};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct ServiceCanId {
    anonymous: bool,
    destination: NodeId,
    priority: Priority,
    service_id: ServiceId,
    source: NodeId,
}

impl ServiceCanId {
    pub fn anonymous(&self) -> bool {
        self.anonymous
    }

    pub fn destination(&self) -> NodeId {
        self.destination
    }

    pub fn service_id(&self) -> ServiceId {
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
