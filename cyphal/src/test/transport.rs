extern crate alloc;

use super::TestTransferId;
use crate::{
    test::TEST_REQUEST_SIZE, CyphalResult, Message, NodeId, Priority, Request, Response, Router,
    ServiceId, SubjectId, TransferId, Transport,
};
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
    /// Maximim Subject ID
    const MAX_SUBJECT_ID: SubjectId = 8191;

    /// Maximim Subject ID
    const MAX_SERVICE_ID: ServiceId = 511;

    /// Maximim Subject ID
    const MAX_NODE_ID: NodeId = 127;

    async fn publish<M>(&mut self, message: &M) -> CyphalResult<()>
    where
        M: Message,
    {
        let _ = message.data();
        self.next_transfer();
        Ok(())
    }

    async fn invoque<R>(&mut self, request: &R) -> CyphalResult<R::Response>
    where
        R: Request,
    {
        let mut data: Vec<u8> = Vec::new();
        for i in 0..(R::Response::SIZE as u8) {
            data.push(i + 1);
        }

        Ok(R::Response::new_raw(
            request.priority(),
            request.service(),
            request.source(),
            request.destination(),
            &data,
        )?)
    }

    async fn serve<R>(&mut self, router: R) -> CyphalResult<()>
    where
        R: Router,
    {
        let data: [u8; TEST_REQUEST_SIZE] = [1; TEST_REQUEST_SIZE];
        let _response = router
            .process_request(
                Priority::High,
                1.try_into().unwrap(),
                2.try_into().unwrap(),
                3.try_into().unwrap(),
                &data,
            )
            .await;

        Ok(())
    }
}
