use crate::{CanNodeId, CanServiceId};
use cyphal::{CyphalResult, Priority, Request, Response};

pub const TEST_REQUEST_SIZE: usize = 0;
pub const TEST_RESPONSE_SIZE: usize = 2;

pub struct TestRequest {
    priority: Priority,
    service: CanServiceId,
    destination: CanNodeId,
    source: CanNodeId,
    data: [u8; TEST_REQUEST_SIZE],
}

impl TestRequest {
    pub fn new(
        priority: Priority,
        service: CanServiceId,
        destination: CanNodeId,
        source: CanNodeId,
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

impl Request<TEST_REQUEST_SIZE, TEST_RESPONSE_SIZE, CanServiceId, CanNodeId> for TestRequest {
    type Response = TestResponse;

    fn priority(&self) -> Priority {
        self.priority
    }

    fn service(&self) -> CanServiceId {
        self.service
    }

    fn destination(&self) -> CanNodeId {
        self.destination
    }

    fn source(&self) -> CanNodeId {
        self.source
    }

    fn data(&self) -> &[u8; TEST_REQUEST_SIZE] {
        &self.data
    }
}

pub struct TestResponse {
    priority: Priority,
    service: CanServiceId,
    destination: CanNodeId,
    source: CanNodeId,
    data: [u8; TEST_RESPONSE_SIZE],
}

impl Response<TEST_RESPONSE_SIZE, CanServiceId, CanNodeId> for TestResponse {
    fn new(
        priority: Priority,
        service: CanServiceId,
        destination: CanNodeId,
        source: CanNodeId,
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

    fn service(&self) -> CanServiceId {
        self.service
    }

    fn destination(&self) -> CanNodeId {
        self.destination
    }

    fn source(&self) -> CanNodeId {
        self.source
    }

    fn data(&self) -> &[u8; TEST_RESPONSE_SIZE] {
        &self.data
    }
}
