use crate::{CanNodeId, CanServiceId};
use cyphal::{CyphalError, CyphalResult, Priority, Request, Response};

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

impl Request<CanServiceId, CanNodeId> for TestRequest {
    const SIZE: usize = TEST_REQUEST_SIZE;

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

    fn data(&self) -> &[u8] {
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

impl Response<CanServiceId, CanNodeId> for TestResponse {
    const SIZE: usize = TEST_RESPONSE_SIZE;

    fn new(
        priority: Priority,
        service: CanServiceId,
        destination: CanNodeId,
        source: CanNodeId,
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

    fn service(&self) -> CanServiceId {
        self.service
    }

    fn destination(&self) -> CanNodeId {
        self.destination
    }

    fn source(&self) -> CanNodeId {
        self.source
    }

    fn data(&self) -> &[u8] {
        &self.data
    }
}
