use crate::CanId;
use cyphal::{NodeId, Priority, ServiceId};

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

    pub fn priority(&self) -> Priority {
        self.priority
    }

    pub fn is_service(&self) -> bool {
        true
    }

    pub fn source(&self) -> NodeId {
        self.source
    }

    pub fn as_raw(&self) -> u32 {
        todo!()
    }
}

impl From<ServiceCanId> for CanId {
    #[inline]
    fn from(id: ServiceCanId) -> Self {
        CanId::Service(id)
    }
}
