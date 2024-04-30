use crate::{CyphalResult, MessageTransfer, TransferId};

pub trait Transport {
    fn transmit_message<const PAYLOAD_SIZE: usize, M>(
        &mut self,
        message: &M,
    ) -> CyphalResult<TransferId>
    where
        M: MessageTransfer<PAYLOAD_SIZE>;
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{CyphalResult, MessageTransfer, TransferId, Transport};

    pub struct FakeTransport {
        transfer_id: TransferId,
    }

    impl FakeTransport {
        pub fn new() -> Self {
            FakeTransport { transfer_id: 0 }
        }

        fn next_transfer_id(&mut self) -> TransferId {
            self.transfer_id += 1;

            self.transfer_id
        }
    }

    impl Transport for FakeTransport {
        fn transmit_message<const PAYLOAD_SIZE: usize, M>(
            &mut self,
            message: &M,
        ) -> CyphalResult<TransferId>
        where
            M: MessageTransfer<PAYLOAD_SIZE>,
        {
            let _ = message.payload();

            Ok(self.next_transfer_id())
        }
    }
}
