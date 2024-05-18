use super::TEST_MESSAGE_SIZE;
use crate::{CyphalResult, Message, NodeId, Priority, SubjectId};

pub struct TestMessage {
    priority: Priority,
    subject: SubjectId,
    source: Option<NodeId>,
    data: [u8; TEST_MESSAGE_SIZE],
}

impl TestMessage {
    pub fn new(
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
        data: [u8; TEST_MESSAGE_SIZE],
    ) -> CyphalResult<Self> {
        Ok(Self {
            priority,
            subject,
            source,
            data,
        })
    }
}

impl Message for TestMessage {
    const SIZE: usize = TEST_MESSAGE_SIZE;

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
