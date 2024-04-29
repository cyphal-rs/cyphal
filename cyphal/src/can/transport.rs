use crate::{can::Can, MessageTransfer, Result, TransferId, Transport};

pub struct CanTransport<C: Can> {
    transfer_id: TransferId,
    can: C,
}

impl<C> CanTransport<C>
where
    C: Can,
{
    pub fn new(can: C) -> Result<CanTransport<C>> {
        Ok(CanTransport {
            transfer_id: 0,
            can,
        })
    }

    fn next_transfer_id(&mut self) -> TransferId {
        self.transfer_id += 1;

        self.transfer_id
    }
}

impl<C> Transport for CanTransport<C>
where
    C: Can,
{
    fn transmit_message<const PAYLOAD_SIZE: usize, M>(&mut self, message: &M) -> Result<TransferId>
    where
        M: MessageTransfer<PAYLOAD_SIZE>,
    {
        let id = self.next_transfer_id();

        //TODO: send payload
        let _ = message.payload();

        Ok(id)
    }
}

#[cfg(test)]
mod test {
    use crate::can::Can;

    struct FakeFrame {}
    impl embedded_can::Frame for FakeFrame {
        fn new(_: impl Into<embedded_can::Id>, _: &[u8]) -> Option<Self> {
            todo!()
        }

        fn new_remote(_: impl Into<embedded_can::Id>, _: usize) -> Option<Self> {
            todo!()
        }

        fn is_extended(&self) -> bool {
            todo!()
        }

        fn is_remote_frame(&self) -> bool {
            todo!()
        }

        fn id(&self) -> embedded_can::Id {
            todo!()
        }

        fn dlc(&self) -> usize {
            todo!()
        }

        fn data(&self) -> &[u8] {
            todo!()
        }
    }

    #[derive(Debug)]
    struct FakeError {}
    impl embedded_can::Error for FakeError {
        fn kind(&self) -> embedded_can::ErrorKind {
            todo!()
        }
    }

    struct FakeCan {}

    impl Can for FakeCan {
        type Frame = FakeFrame;

        type Error = FakeError;

        fn transmit(&mut self, _: &Self::Frame) -> Result<(), Self::Error> {
            Ok(())
        }

        fn receive(&mut self) -> Result<Self::Frame, Self::Error> {
            todo!()
        }
    }

    // #[test]
    // #[ignore = "not implemented"]
    // fn create_transport() {
    //     let can = FakeCan {};
    //     let mut transport = CanTransport::new(can).expect("Could not create transport");
    //     let _ = transport.new_message_transfer(Priority::Nominal, 1, Some(2), &[0]);
    // }
}
