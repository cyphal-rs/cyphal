use crate::{NodeId, Priority, Router, ServiceId, SubjectId};

pub struct TestRouter {}

impl Router for TestRouter {
    async fn process_message(
        &self,
        _priority: Priority,
        _subject: SubjectId,
        _source: NodeId,
        _data: &[u8],
    ) {
    }

    async fn process_request(
        &self,
        _priority: Priority,
        _service: ServiceId,
        _source: NodeId,
        _destination: NodeId,
        _data: &[u8],
    ) {
    }
}
