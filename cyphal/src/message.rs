use crate::{CyphalResult, NodeId, Priority, SubjectId};

pub trait Message<const PAYLOAD_SIZE: usize>: Sized {
    type Payload;

    fn new(
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
        payload: Self::Payload,
    ) -> CyphalResult<Self>;

    fn source(&self) -> Option<NodeId>;

    fn subject(&self) -> SubjectId;

    fn priority(&self) -> Priority;

    fn payload(&self) -> &[u8];
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{
        transport::test::MockTransport, CyphalResult, Message, NodeId, Priority, SubjectId,
        Transport,
    };

    pub struct MockMessage {
        priority: Priority,
        subject: u64,
        source: Option<NodeId>,
        payload: [u8; 1],
    }

    impl Message<1> for MockMessage {
        type Payload = [u8; 1];

        fn new(
            priority: Priority,
            subject: SubjectId,
            source: Option<NodeId>,
            payload: Self::Payload,
        ) -> CyphalResult<Self> {
            Ok(Self {
                priority,
                subject,
                source,
                payload,
            })
        }

        fn source(&self) -> Option<NodeId> {
            self.source
        }

        fn subject(&self) -> SubjectId {
            self.subject
        }

        fn priority(&self) -> Priority {
            self.priority
        }

        fn payload(&self) -> &[u8] {
            &self.payload
        }
    }

    pub struct MockLargeMessage {
        priority: Priority,
        subject: u64,
        source: Option<NodeId>,
        payload: [u8; 65],
    }

    impl Message<65> for MockLargeMessage {
        type Payload = [u8; 65];

        fn new(
            priority: Priority,
            subject: SubjectId,
            source: Option<NodeId>,
            payload: Self::Payload,
        ) -> CyphalResult<Self> {
            Ok(Self {
                priority,
                subject,
                source,
                payload,
            })
        }

        fn source(&self) -> Option<NodeId> {
            self.source
        }

        fn subject(&self) -> SubjectId {
            self.subject
        }

        fn priority(&self) -> Priority {
            self.priority
        }

        fn payload(&self) -> &[u8] {
            &self.payload
        }
    }

    #[test]
    fn new() {
        let message = MockMessage::new(Priority::Nominal, 1, None, [0]).unwrap();
        assert_eq!(message.payload().len(), 1);

        let mut transport = MockTransport::new();
        let id = transport.transmit_message(&message).unwrap();
        assert_eq!(id, 1);
    }
}
