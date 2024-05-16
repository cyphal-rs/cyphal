use crate::{CanNodeId, CanSubjectId};
use cyphal::{CyphalResult, Message, Priority};

pub const SMALL_MESSAGE_SIZE: usize = 2;
pub const LARGE_MESSAGE_SIZE: usize = 65;

pub struct TestSmallMessage {
    priority: Priority,
    subject: CanSubjectId,
    source: Option<CanNodeId>,
    data: [u8; SMALL_MESSAGE_SIZE],
}

impl TestSmallMessage {
    pub fn new(
        priority: Priority,
        subject: CanSubjectId,
        source: Option<CanNodeId>,
        data: [u8; SMALL_MESSAGE_SIZE],
    ) -> CyphalResult<Self> {
        Ok(Self {
            priority,
            subject,
            source,
            data,
        })
    }
}

impl Message<CanSubjectId, CanNodeId> for TestSmallMessage {
    const SIZE: usize = SMALL_MESSAGE_SIZE;

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
        &self.data
    }
}

pub struct TestLargeMessage {
    priority: Priority,
    subject: CanSubjectId,
    source: Option<CanNodeId>,
    data: [u8; LARGE_MESSAGE_SIZE],
}

impl TestLargeMessage {
    pub fn new(
        priority: Priority,
        subject: CanSubjectId,
        source: Option<CanNodeId>,
        data: [u8; LARGE_MESSAGE_SIZE],
    ) -> CyphalResult<Self> {
        Ok(Self {
            priority,
            subject,
            source,
            data,
        })
    }
}

impl Message<CanSubjectId, CanNodeId> for TestLargeMessage {
    const SIZE: usize = LARGE_MESSAGE_SIZE;

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
        &self.data
    }
}
