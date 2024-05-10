use crate::{
    CyphalResult, Message, NodeId, Priority, Request, Response, ServiceId, SubjectId, TransferId,
    Transport,
};

pub const TEST_MESSAGE_SIZE: usize = 78;
pub const TEST_REQUEST_SIZE: usize = 15;
pub const TEST_RESPONSE_SIZE: usize = 86;
pub const TEST_MAX_TRANSFER_ID: u8 = 12;

pub struct TestMessage {
    priority: Priority,
    subject: SubjectId,
    source: Option<NodeId>,
    data: [u8; TEST_MESSAGE_SIZE],
}

impl TestMessage {
    pub fn new(
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
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

impl Message<TEST_MESSAGE_SIZE> for TestMessage {
    fn priority(&self) -> Priority {
        self.priority
    }

    fn subject(&self) -> SubjectId {
        self.subject
    }

    fn source(&self) -> Option<NodeId> {
        self.source
    }

    fn data(&self) -> &[u8; TEST_MESSAGE_SIZE] {
        &self.data
    }
}

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

impl Request<TEST_REQUEST_SIZE, TEST_RESPONSE_SIZE> for TestRequest {
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

    fn data(&self) -> &[u8; TEST_REQUEST_SIZE] {
        &self.data
    }
}

pub struct TestResponse {
    priority: Priority,
    service: ServiceId,
    destination: NodeId,
    source: NodeId,
    data: [u8; TEST_RESPONSE_SIZE],
}

impl Response<TEST_RESPONSE_SIZE> for TestResponse {
    fn new(
        priority: Priority,
        service: ServiceId,
        destination: NodeId,
        source: NodeId,
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

    fn service(&self) -> ServiceId {
        self.service
    }

    fn destination(&self) -> NodeId {
        self.destination
    }

    fn source(&self) -> NodeId {
        self.source
    }

    fn data(&self) -> &[u8; TEST_RESPONSE_SIZE] {
        &self.data
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TestTransferId {
    value: u8,
}

impl TransferId<u8> for TestTransferId {
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

impl Default for TestTransferId {
    fn default() -> Self {
        TestTransferId { value: 0 }
    }
}

pub struct TestTransport {
    pub transfer_id: TestTransferId,
}

impl TestTransport {
    pub fn new() -> Self {
        TestTransport {
            transfer_id: TestTransferId::default(),
        }
    }

    fn next_transfer_id(&mut self) -> TestTransferId {
        self.transfer_id = self.transfer_id.next();

        self.transfer_id
    }
}

impl Transport for TestTransport {
    async fn publish<const N: usize, M: Message<N>>(&mut self, message: &M) -> CyphalResult<()> {
        let _ = message.data();
        self.next_transfer_id();
        Ok(())
    }

    async fn invoque<const N: usize, const M: usize, R: Request<N, M>>(
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
