extern crate alloc;

use super::{TestNodeId, TestServiceId, TestSubjectId, TestTransferId};
use crate::{CyphalResult, Message, Request, Response, TransferId, Transport};
use alloc::vec::Vec;

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

    async fn publish<M>(&mut self, message: &M) -> CyphalResult<()>
    where
        M: Message<Self::SubjectId, Self::NodeId>,
    {
        let _ = message.data();
        self.next_transfer();
        Ok(())
    }

    async fn invoque<R>(&mut self, request: &R) -> CyphalResult<R::Response>
    where
        R: Request<Self::ServiceId, Self::NodeId>,
    {
        let mut data: Vec<u8> = Vec::new();
        for i in 0..(R::Response::SIZE as u8) {
            data.push(i + 1);
        }

        Ok(R::Response::new(
            request.priority(),
            request.service(),
            request.destination(),
            request.source(),
            &data,
        )?)
    }
}
