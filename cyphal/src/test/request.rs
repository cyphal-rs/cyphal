use super::{TestNodeId, TestResponse, TestServiceId, TEST_REQUEST_SIZE};
use crate::{CyphalResult, Priority, Request};

pub struct TestRequest {
    priority: Priority,
    service: TestServiceId,
    destination: TestNodeId,
    source: TestNodeId,
    data: [u8; TEST_REQUEST_SIZE],
}

impl TestRequest {
    pub fn new(
        priority: Priority,
        service: TestServiceId,
        destination: TestNodeId,
        source: TestNodeId,
        data: [u8; TEST_REQUEST_SIZE],
    ) -> CyphalResult<Self> {
        Ok(Self {
            priority,
            service,
            destination,
            source,
            data,
        })
    }
}

impl Request<TestServiceId, TestNodeId> for TestRequest {
    const SIZE: usize = TEST_REQUEST_SIZE;
    type Response = TestResponse;

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

    fn data(&self) -> &[u8] {
        &self.data
    }
}
