use crate::{CyphalResult, Message, TransferId};

pub trait Transport {
    fn transmit_message<const PAYLOAD_SIZE: usize>(
        &mut self,
        message: &impl Message<{ PAYLOAD_SIZE }>,
    ) -> CyphalResult<TransferId>;
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{CyphalResult, Message, TransferId, Transport};

    pub struct MockTransport {
        transfer_id: TransferId,
    }

    impl MockTransport {
        pub fn new() -> Self {
            MockTransport { transfer_id: 0 }
        }

        fn next_transfer_id(&mut self) -> TransferId {
            self.transfer_id += 1;

            self.transfer_id
        }
    }

    impl Transport for MockTransport {
        fn transmit_message<const PAYLOAD_SIZE: usize>(
            &mut self,
            message: &impl Message<PAYLOAD_SIZE>,
        ) -> CyphalResult<TransferId> {
            let _ = message.payload();

            Ok(self.next_transfer_id())
        }
    }
}
