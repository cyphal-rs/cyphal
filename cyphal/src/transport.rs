use crate::{MessageTransfer, NodeId, Priority, Result, SubjectId};

pub trait Transport {
    fn new_message_transfer<'a, const PAYLOAD_SIZE: usize, T, M>(
        &self,
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
        payload: [u8; PAYLOAD_SIZE],
    ) -> Result<M>
    where
        T: Transport,
        M: MessageTransfer<'a, PAYLOAD_SIZE, T>;

    fn transmit_message<'a, const PAYLOAD_SIZE: usize, T, M>(&self, message: &M) -> Result<()>
    where
        T: Transport,
        M: MessageTransfer<'a, PAYLOAD_SIZE, T>;
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{MessageTransfer, TransferId, Transport};

    pub struct FakeTransport {
        transfer_id: TransferId,
    }

    impl FakeTransport {
        pub fn new() -> Self {
            FakeTransport { transfer_id: 0 }
        }
    }

    impl Transport for FakeTransport {
        fn new_message_transfer<'a, const PAYLOAD_SIZE: usize, T, M>(
            &self,
            priority: crate::Priority,
            subject: crate::SubjectId,
            source: Option<crate::NodeId>,
            payload: [u8; PAYLOAD_SIZE],
        ) -> crate::Result<M>
        where
            T: Transport,
            M: MessageTransfer<'a, PAYLOAD_SIZE, T>,
        {
            todo!()
        }

        fn transmit_message<'a, const PAYLOAD_SIZE: usize, T, M>(
            &self,
            message: &M,
        ) -> crate::Result<()>
        where
            T: Transport,
            M: MessageTransfer<'a, PAYLOAD_SIZE, T>,
        {
            let _ = message.payload();
            Ok(())
        }
    }
}
