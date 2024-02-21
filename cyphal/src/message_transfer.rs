extern crate alloc;
use alloc::boxed::Box;

use crate::{crc::crc32c, NodeId, Priority, SubjectId, Transfer, TransferId, TransferKind};

pub struct MessageTransfer {
    priority: Priority,
    subject: u64,
    id: TransferId,
    source: Option<NodeId>,
    payload: Box<[u8]>,
}

impl MessageTransfer {
    pub fn source(&self) -> Option<NodeId> {
        self.source
    }

    pub fn subject(&self) -> SubjectId {
        self.subject
    }
}

impl Transfer for MessageTransfer {
    fn priority(&self) -> Priority {
        self.priority
    }

    fn id(&self) -> TransferId {
        self.id
    }

    fn crc(&self) -> Option<u32> {
        #[cfg(any(feature = "serial", feature = "udp"))]
        crc32c(&self.payload);

        None
    }

    fn kind(&self) -> TransferKind {
        TransferKind::Message
    }

    fn payload(&self) -> &[u8] {
        &self.payload
    }
}
