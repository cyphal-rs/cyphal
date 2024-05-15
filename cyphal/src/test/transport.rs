use super::{TestNodeId, TestServiceId, TestSubjectId, TestTransferId};
use crate::{CyphalResult, Message, Request, Response, TransferId, Transport};

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
