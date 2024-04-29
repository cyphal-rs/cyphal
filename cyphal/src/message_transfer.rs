// #[cfg(feature = "crc")]
// use crate::crc::crc32c;

use crate::{NodeId, Priority, Result, SubjectId, TransferId, Transport};

pub trait MessageTransfer<'a, const PAYLOAD_SIZE: usize, T: Transport> {
    fn new(
        transport: &'a T,
        id: TransferId,
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
        payload: [u8; PAYLOAD_SIZE],
    ) -> Self;

    fn source(&self) -> Option<NodeId>;

    fn subject(&self) -> SubjectId;

    fn priority(&self) -> Priority;

    fn id(&self) -> TransferId;

    fn payload(&self) -> &[u8];

    fn transmit(&self) -> Result<()>;
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{
        transport::test::FakeTransport, MessageTransfer, NodeId, Priority, SubjectId, TransferId,
        Transport,
    };

    pub struct MockMessageTransfer<'a, const PAYLOAD_SIZE: usize, T: Transport> {
        transport: &'a T,
        id: TransferId,
        priority: Priority,
        subject: u64,
        source: Option<NodeId>,
        payload: [u8; PAYLOAD_SIZE],
    }

    impl<'a, const PAYLOAD_SIZE: usize, T: Transport> MessageTransfer<'a, PAYLOAD_SIZE, T>
        for MockMessageTransfer<'a, PAYLOAD_SIZE, T>
    {
        fn new(
            transport: &'a T,
            id: TransferId,
            priority: Priority,
            subject: SubjectId,
            source: Option<NodeId>,
            payload: [u8; PAYLOAD_SIZE],
        ) -> Self {
            Self {
                transport,
                id,
                priority,
                subject,
                source,
                payload,
            }
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

        fn id(&self) -> TransferId {
            self.id
        }

        // #[cfg(feature = "crc")]
        // fn crc(&self) -> u32 {
        //     crc32c(&self.payload)
        // }

        fn payload(&self) -> &[u8] {
            &self.payload
        }

        fn transmit(&self) -> crate::Result<()> {
            self.transport.transmit_message(self)
        }
    }

    #[test]
    fn new() {
        let transport = FakeTransport::new();
        let transfer = MockMessageTransfer::new(&transport, 1, Priority::Nominal, 1, None, [0]);

        assert_eq!(transfer.payload().len(), 1);
    }
}
