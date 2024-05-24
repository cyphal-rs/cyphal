use crate::{CyphalResult, Message, NodeId, Request, Router, ServiceId, SubjectId};

/// Trait representing the Cyphal transport
pub trait Transport {
    /// Maximim Subject ID
    const MAX_SUBJECT_ID: SubjectId;

    /// Maximim Subject ID
    const MAX_SERVICE_ID: ServiceId;

    /// Maximim Subject ID
    const MAX_NODE_ID: NodeId;

    /// Publishes a message
    async fn publish<M>(&mut self, message: &M) -> CyphalResult<()>
    where
        M: Message;

    /// Invoques a service call
    async fn invoque<R>(&mut self, request: &R) -> CyphalResult<R::Response>
    where
        R: Request;

    /// Listen to incoming traffic
    async fn serve<R>(&mut self, router: R) -> CyphalResult<()>
    where
        R: Router;
}

#[cfg(test)]
mod test {
    use crate::{
        test::{
            TestMessage, TestRequest, TestRouter, TestTransport, TEST_MESSAGE_SIZE,
            TEST_REQUEST_SIZE,
        },
        Priority, Transport,
    };

    #[async_std::test]
    async fn test_publish() {
        let message = TestMessage::new(
            Priority::Nominal,
            1.try_into().unwrap(),
            None,
            [1; TEST_MESSAGE_SIZE],
        )
        .unwrap();

        let mut transport = TestTransport::new();
        let result = transport.publish(&message).await;

        assert!(result.is_ok())
    }

    #[async_std::test]
    async fn test_invoque() {
        let request = TestRequest::new(
            Priority::Nominal,
            1.try_into().unwrap(),
            2.try_into().unwrap(),
            3.try_into().unwrap(),
            [0; TEST_REQUEST_SIZE],
        )
        .unwrap();

        let mut transport = TestTransport::new();
        let result = transport.invoque(&request).await;

        assert!(result.is_ok())
    }

    #[async_std::test]
    async fn tesr_router() {
        let router = TestRouter {};

        let mut transport = TestTransport::new();
        let result = transport.serve(router).await;

        assert!(result.is_ok())
    }
}
