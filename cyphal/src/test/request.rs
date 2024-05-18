use super::{TestResponse, TEST_REQUEST_SIZE};
use crate::{CyphalResult, NodeId, Priority, Request, ServiceId};

pub struct TestRequest {
    priority: Priority,
    service: ServiceId,
    destination: NodeId,
    source: NodeId,
    data: [u8; TEST_REQUEST_SIZE],
}

impl TestRequest {
    pub fn new(
        priority: Priority,
        service: ServiceId,
        destination: NodeId,
        source: NodeId,
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

impl Request for TestRequest {
    const SIZE: usize = TEST_REQUEST_SIZE;
    type Response = TestResponse;

    fn priority(&self) -> Priority {
        self.priority
    }

    fn service(&self) -> ServiceId {
        self.service
    }

    fn destination(&self) -> NodeId {
        self.destination
    }

    fn source(&self) -> NodeId {
        self.source
    }

    fn data(&self) -> &[u8] {
        &self.data
    }
}
