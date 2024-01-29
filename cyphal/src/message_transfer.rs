use crate::{NodeId, Priority, SubjectId, Transfer, TransferId, TransferKind};

pub struct MessageTransfer {
    priority: Priority,
    subject: u64,
    id: TransferId,
    source: NodeId,
}

impl MessageTransfer {
    pub fn subject(self) -> SubjectId {
        self.subject
    }
}

impl Transfer for MessageTransfer {
    fn priority(self) -> Priority {
        self.priority
    }

    fn id(self) -> TransferId {
        self.id
    }

    fn source(self) -> NodeId {
        self.source
    }

    fn kind(self) -> TransferKind {
        TransferKind::Message
    }
}
