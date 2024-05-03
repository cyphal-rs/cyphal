use crate::{can::CanId, NodeId, Priority};

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
