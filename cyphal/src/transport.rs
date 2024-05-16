use crate::{CyphalResult, Message, NodeId, Request, ServiceId, SubjectId};

/// Trait representing the Cyphal transport
pub trait Transport {
    /// Node ID type used by the transport
    type NodeId: NodeId;

    /// Service ID type used by the transport
    type ServiceId: ServiceId;

    /// Subject ID type used by the transport
    type SubjectId: SubjectId;

    /// Publishes a message
    async fn publish<M>(&mut self, message: &M) -> CyphalResult<()>
    where
        M: Message<Self::SubjectId, Self::NodeId>;

    /// Invoques a service call
    async fn invoque<R>(&mut self, request: &R) -> CyphalResult<R::Response>
    where
        R: Request<Self::ServiceId, Self::NodeId>;
}

#[cfg(test)]
mod test {
    use crate::{
        test::{TestMessage, TestRequest, TestTransport, TEST_MESSAGE_SIZE, TEST_REQUEST_SIZE},
        Priority, Transport,
    };

    #[async_std::test]
    async fn test_publish() {
        let message = TestMessage::new(
            Priority::Nominal,
            1.try_into().unwrap(),
            None,
            [0; TEST_MESSAGE_SIZE],
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
}
