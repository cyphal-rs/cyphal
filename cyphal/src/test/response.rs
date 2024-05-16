use super::{TestNodeId, TestServiceId, TEST_RESPONSE_SIZE};
use crate::{CyphalError, CyphalResult, Priority, Response};

pub struct TestResponse {
    priority: Priority,
    service: TestServiceId,
    destination: TestNodeId,
    source: TestNodeId,
    data: [u8; TEST_RESPONSE_SIZE],
}

impl Response<TestServiceId, TestNodeId> for TestResponse {
    const SIZE: usize = TEST_RESPONSE_SIZE;

    fn new(
        priority: Priority,
        service: TestServiceId,
        destination: TestNodeId,
        source: TestNodeId,
        data: &[u8],
    ) -> CyphalResult<Self> {
        if data.len() != Self::SIZE {
            return Err(CyphalError::OutOfRange);
        }

        let mut d: [u8; Self::SIZE] = [0; Self::SIZE];
        d.copy_from_slice(data);

        Ok(Self {
            priority,
            service,
            destination,
            source,
            data: d,
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

    fn data(&self) -> &[u8] {
        &self.data
    }
}
