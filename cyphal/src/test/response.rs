use super::{TestNodeId, TestServiceId, TEST_RESPONSE_SIZE};
use crate::{CyphalResult, Priority, Response};

pub struct TestResponse {
    priority: Priority,
    service: TestServiceId,
    destination: TestNodeId,
    source: TestNodeId,
    data: [u8; TEST_RESPONSE_SIZE],
}

impl Response<TEST_RESPONSE_SIZE, TestServiceId, TestNodeId> for TestResponse {
    fn new(
        priority: Priority,
        service: TestServiceId,
        destination: TestNodeId,
        source: TestNodeId,
        data: [u8; TEST_RESPONSE_SIZE],
    ) -> CyphalResult<Self> {
        Ok(Self {
            priority,
            service,
            destination,
            source,
            data,
        })
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn service(&self) -> TestServiceId {
        self.service
    }

    fn destination(&self) -> TestNodeId {
        self.destination
    }

    fn source(&self) -> TestNodeId {
        self.source
    }

    fn data(&self) -> &[u8; TEST_RESPONSE_SIZE] {
        &self.data
    }
}
