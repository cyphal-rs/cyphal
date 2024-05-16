use cyphal::{CyphalResult, Message, Priority};
use cyphal_can::{CanNodeId, CanSubjectId};

const SINGLE_SIZE: usize = 2;
const MULTI_SIZE: usize = 65;

pub struct SingleFrameMessage {
    priority: Priority,
    subject: CanSubjectId,
    source: Option<CanNodeId>,
    payload: [u8; SINGLE_SIZE],
}

impl SingleFrameMessage {
    pub fn new(
        priority: Priority,
        subject: CanSubjectId,
        source: Option<CanNodeId>,
        payload: [u8; SINGLE_SIZE],
    ) -> CyphalResult<Self> {
        Ok(Self {
            priority,
            subject,
            source,
            payload,
        })
    }
}

impl Message<CanSubjectId, CanNodeId> for SingleFrameMessage {
    const SIZE: usize = SINGLE_SIZE;

    fn priority(&self) -> Priority {
        self.priority
    }

    fn subject(&self) -> CanSubjectId {
        self.subject
    }

    fn source(&self) -> Option<CanNodeId> {
        self.source
    }

    fn data(&self) -> &[u8] {
        &self.payload
    }
}

pub struct MultiFrameMessage {
    priority: Priority,
    subject: CanSubjectId,
    source: Option<CanNodeId>,
    payload: [u8; MULTI_SIZE],
}

impl MultiFrameMessage {
    pub fn new(
        priority: Priority,
        subject: CanSubjectId,
        source: Option<CanNodeId>,
        payload: [u8; MULTI_SIZE],
    ) -> CyphalResult<Self> {
        Ok(Self {
            priority,
            subject,
            source,
            payload,
        })
    }
}

impl Message<CanSubjectId, CanNodeId> for MultiFrameMessage {
    const SIZE: usize = MULTI_SIZE;

    fn source(&self) -> Option<CanNodeId> {
        self.source
    }

    fn subject(&self) -> CanSubjectId {
        self.subject
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn data(&self) -> &[u8] {
        &self.payload
    }
}
