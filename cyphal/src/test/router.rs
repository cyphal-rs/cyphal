use super::{TestNodeId, TestServiceId, TestSubjectId};
use crate::{CyphalError, CyphalResult, Router};
use core::future::Future;

pub struct TestRouter {}

impl Router for TestRouter {
    type Error = CyphalError;

    type NodeId = TestNodeId;

    type ServiceId = TestServiceId;

    type SubjectId = TestSubjectId;

    fn route_message<const MESSAGE_SIZE: usize, M, F, Fut>(
        &self,
        subject: Self::SubjectId,
        callback: F,
    ) where
        M: crate::Message<MESSAGE_SIZE, Self::SubjectId, Self::NodeId>,
        F: Fn(&M) -> Fut + Sync + Send + 'static,
        Fut: Future<Output = ()> + Send,
    {
        todo!()
    }

    fn route_request<const REQUEST_SIZE: usize, const RESPONSE_SIZE: usize, R, F, Fut>(
        &self,
        service: Self::ServiceId,
        callback: F,
    ) where
        R: crate::Request<REQUEST_SIZE, RESPONSE_SIZE, Self::ServiceId, Self::NodeId>,
        F: Fn(&R) -> Fut + Sync + Send + 'static,
        Fut: Future<Output = CyphalResult<R::Response>> + Send,
    {
        todo!()
    }

    async fn process_message<const MESSAGE_SIZE: usize, M>(
        &self,
        message: &M,
    ) -> Result<(), Self::Error>
    where
        M: crate::Message<MESSAGE_SIZE, Self::SubjectId, Self::NodeId>,
    {
        todo!()
    }

    async fn process_request<const REQUEST_SIZE: usize, const RESPONSE_SIZE: usize, R>(
        &self,
        request: &R,
    ) -> Result<R::Response, Self::Error>
    where
        R: crate::Request<REQUEST_SIZE, RESPONSE_SIZE, Self::ServiceId, Self::NodeId>,
    {
        todo!()
    }
}
