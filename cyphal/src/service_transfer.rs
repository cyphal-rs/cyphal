extern crate alloc;
use alloc::boxed::Box;

use crate::{crc::crc32c, NodeId, Priority, Transfer, TransferId, TransferKind};

pub struct ServiceTransfer {
    priority: Priority,
    service: NodeId,
    id: TransferId,
    source: NodeId,
    payload: Box<[u8]>,
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

    fn crc(&self) -> Option<u32> {
        #[cfg(any(feature = "serial", feature = "udp"))]
        crc32c(&self.payload);

        None
    }

    fn kind(&self) -> TransferKind {
        TransferKind::Service
    }

    fn payload(&self) -> &[u8] {
        &self.payload
    }
}
