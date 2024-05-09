use crate::{CyphalResult, Message, Request};

/// Trait representing the Cyphal transport
pub trait Transport {
    /// Publishes a message
    async fn publish<const N: usize, M: Message<N>>(&mut self, message: &M) -> CyphalResult<()>;

    /// Invoques a service call
    async fn invoque<const N: usize, const M: usize, R: Request<N, M>>(
        &mut self,
        request: &R,
    ) -> CyphalResult<R::Response>;
}

#[cfg(test)]
mod test {
    use crate::{
        test::{TestMessage, TestRequest, TestTransport, TEST_MESSAGE_SIZE, TEST_REQUEST_SIZE},
        Priority, Transport,
    };

    #[async_std::test]
    async fn test_publish() {
        let message = TestMessage::new(Priority::Nominal, 1, None, [0; TEST_MESSAGE_SIZE]).unwrap();

        let mut transport = TestTransport::new();
        let result = transport.publish(&message).await;

        assert!(result.is_ok())
    }

    #[async_std::test]
    async fn test_invoque() {
        let request = TestRequest::new(Priority::Nominal, 1, 2, 3, [0; TEST_REQUEST_SIZE]).unwrap();

        let mut transport = TestTransport::new();
        let result = transport.invoque(&request).await;

        assert!(result.is_ok())
    }
}
