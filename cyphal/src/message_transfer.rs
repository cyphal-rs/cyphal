use crate::{CyphalResult, NodeId, Priority, SubjectId};

pub trait MessageTransfer<const PAYLOAD_SIZE: usize>: Sized {
    fn new(
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
        payload: [u8; PAYLOAD_SIZE],
    ) -> CyphalResult<Self>;

    fn source(&self) -> Option<NodeId>;

    fn subject(&self) -> SubjectId;

    fn priority(&self) -> Priority;

    fn payload(&self) -> &[u8];
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{
        transport::test::FakeTransport, CyphalResult, MessageTransfer, NodeId, Priority, SubjectId,
        Transport,
    };

    pub struct FakeMessageTransfer<const PAYLOAD_SIZE: usize> {
        priority: Priority,
        subject: u64,
        source: Option<NodeId>,
        payload: [u8; PAYLOAD_SIZE],
    }

    impl<const PAYLOAD_SIZE: usize> MessageTransfer<PAYLOAD_SIZE>
        for FakeMessageTransfer<PAYLOAD_SIZE>
    {
        fn new(
            priority: Priority,
            subject: SubjectId,
            source: Option<NodeId>,
            payload: [u8; PAYLOAD_SIZE],
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
        let transfer = FakeMessageTransfer::new(Priority::Nominal, 1, None, [0]).unwrap();
        assert_eq!(transfer.payload().len(), 1);

        let mut transport = FakeTransport::new();
        let id = transport.transmit_message(&transfer).unwrap();
        assert_eq!(id, 1);
    }
}
