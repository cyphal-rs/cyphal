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

impl Message<SMALL_MESSAGE_SIZE> for TestSmallMessage {
    fn priority(&self) -> Priority {
        self.priority
    }

    fn subject(&self) -> SubjectId {
        self.subject
    }

    fn source(&self) -> Option<NodeId> {
        self.source
    }

    fn data(&self) -> &[u8; SMALL_MESSAGE_SIZE] {
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

impl Message<LARGE_MESSAGE_SIZE> for TestLargeMessage {
    fn source(&self) -> Option<NodeId> {
        self.source
    }

    fn subject(&self) -> SubjectId {
        self.subject
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn data(&self) -> &[u8; LARGE_MESSAGE_SIZE] {
        &self.data
    }
}
