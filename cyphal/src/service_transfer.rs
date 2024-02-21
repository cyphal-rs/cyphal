use crate::{NodeId, Priority, Transfer, TransferId, TransferKind};

pub struct ServiceTransfer {
    priority: Priority,
    service: NodeId,
    id: TransferId,
    source: NodeId,
}

impl ServiceTransfer {
    pub fn service(&self) -> NodeId {
        self.service
    }

    pub fn source(&self) -> NodeId {
        self.source
    }
}

impl Transfer for ServiceTransfer {
    fn priority(&self) -> Priority {
        self.priority
    }

    fn id(&self) -> TransferId {
        self.id
    }

    fn kind(&self) -> TransferKind {
        TransferKind::Service
    }
}
