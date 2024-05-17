use super::{TestNodeId, TestServiceId, TestSubjectId};
use crate::{Priority, Router};

pub struct TestRouter {}

impl Router<TestSubjectId, TestServiceId, TestNodeId> for TestRouter {
    async fn process_message(
        &self,
        _priority: Priority,
        _subject: TestSubjectId,
        _source: TestNodeId,
        _data: &[u8],
    ) {
    }

    async fn process_request(
        &self,
        _priority: Priority,
        _service: TestServiceId,
        _source: TestNodeId,
        _destination: TestNodeId,
        _data: &[u8],
    ) {
    }
}
