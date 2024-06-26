use cyphal::{CyphalResult, Message, NodeId, Priority, SubjectId};

pub const SMALL_MESSAGE_SIZE: usize = 2;
pub const LARGE_MESSAGE_SIZE: usize = 65;

pub struct TestSmallMessage {
    priority: Priority,
    subject: SubjectId,
    source: Option<NodeId>,
    data: [u8; SMALL_MESSAGE_SIZE],
}

impl TestSmallMessage {
    pub fn new(
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
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

impl Message for TestSmallMessage {
    const SIZE: usize = SMALL_MESSAGE_SIZE;

    fn priority(&self) -> Priority {
        self.priority
    }

    fn subject(&self) -> SubjectId {
        self.subject
    }

    fn source(&self) -> Option<NodeId> {
        self.source
    }

    fn data(&self) -> &[u8] {
        &self.data
    }
}

pub struct TestLargeMessage {
    priority: Priority,
    subject: SubjectId,
    source: Option<NodeId>,
    data: [u8; LARGE_MESSAGE_SIZE],
}

impl TestLargeMessage {
    pub fn new(
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
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

impl Message for TestLargeMessage {
    const SIZE: usize = LARGE_MESSAGE_SIZE;

    fn source(&self) -> Option<NodeId> {
        self.source
    }

    fn subject(&self) -> SubjectId {
        self.subject
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn data(&self) -> &[u8] {
        &self.data
    }
}
