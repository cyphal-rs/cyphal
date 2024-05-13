use crate::{
    CyphalError, CyphalResult, Message, NodeId, Priority, Request, Response, ServiceId, SubjectId,
    TransferId, Transport,
};

pub const TEST_MESSAGE_SIZE: usize = 78;
pub const TEST_REQUEST_SIZE: usize = 15;
pub const TEST_RESPONSE_SIZE: usize = 86;
pub const TEST_MAX_TRANSFER_ID: u8 = 12;

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

impl Request<TEST_REQUEST_SIZE, TEST_RESPONSE_SIZE, TestServiceId, TestNodeId> for TestRequest {
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

    fn data(&self) -> &[u8; TEST_REQUEST_SIZE] {
        &self.data
    }
}

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

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TestNodeId {
    value: u8,
}

impl NodeId for TestNodeId {
    type T = u8;

    fn value(&self) -> Self::T {
        self.value
    }
}

impl TryFrom<u8> for TestNodeId {
    type Error = CyphalError;

    fn try_from(value: u8) -> CyphalResult<Self> {
        if value > 127 {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self { value })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TestSubjectId {
    value: u16,
}

impl SubjectId for TestSubjectId {
    type T = u16;

    fn value(&self) -> Self::T {
        self.value
    }
}

impl TryFrom<u16> for TestSubjectId {
    type Error = CyphalError;

    fn try_from(value: u16) -> CyphalResult<Self> {
        if value > 8191 {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self { value })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TestServiceId {
    value: u8,
}

impl ServiceId for TestServiceId {
    type T = u8;

    fn value(&self) -> Self::T {
        self.value
    }
}

impl TryFrom<u8> for TestServiceId {
    type Error = CyphalError;

    fn try_from(value: u8) -> CyphalResult<Self> {
        if value > 127 {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self { value })
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct TestTransferId {
    value: u8,
}

impl TransferId for TestTransferId {
    type T = u8;

    fn value(&self) -> u8 {
        self.value
    }

    fn next(&self) -> Self {
        if self.value == TEST_MAX_TRANSFER_ID {
            TestTransferId { value: 0 }
        } else {
            TestTransferId {
                value: self.value + 1,
            }
        }
    }
}

impl TryFrom<u8> for TestTransferId {
    type Error = CyphalError;

    fn try_from(value: u8) -> CyphalResult<Self> {
        if value > 31 {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self { value })
    }
}

pub struct TestTransport {
    pub transfer: TestTransferId,
}

impl TestTransport {
    pub fn new() -> Self {
        TestTransport {
            transfer: TestTransferId::default(),
        }
    }

    fn next_transfer(&mut self) -> TestTransferId {
        self.transfer = self.transfer.next();

        self.transfer
    }
}

impl Transport for TestTransport {
    type NodeId = TestNodeId;
    type ServiceId = TestServiceId;
    type SubjectId = TestSubjectId;

    async fn publish<const N: usize, M: Message<N, Self::SubjectId, Self::NodeId>>(
        &mut self,
        message: &M,
    ) -> CyphalResult<()> {
        let _ = message.data();
        self.next_transfer();
        Ok(())
    }

    async fn invoque<
        const N: usize,
        const M: usize,
        R: Request<N, M, Self::ServiceId, Self::NodeId>,
    >(
        &mut self,
        request: &R,
    ) -> CyphalResult<R::Response> {
        Ok(R::Response::new(
            request.priority(),
            request.service(),
            request.destination(),
            request.source(),
            [0; M],
        )
        .unwrap())
    }
}
