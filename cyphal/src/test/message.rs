use super::{TestNodeId, TestSubjectId, TEST_MESSAGE_SIZE};
use crate::{CyphalResult, Message, Priority};

pub struct TestMessage {
    priority: Priority,
    subject: TestSubjectId,
    source: Option<TestNodeId>,
    data: [u8; TEST_MESSAGE_SIZE],
}

impl TestMessage {
    pub fn new(
        priority: Priority,
        subject: TestSubjectId,
        source: Option<TestNodeId>,
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

impl Message<TEST_MESSAGE_SIZE, TestSubjectId, TestNodeId> for TestMessage {
    fn priority(&self) -> Priority {
        self.priority
    }

    fn subject(&self) -> TestSubjectId {
        self.subject
    }

    fn source(&self) -> Option<TestNodeId> {
        self.source
    }

    fn data(&self) -> &[u8; TEST_MESSAGE_SIZE] {
        &self.data
    }
}
