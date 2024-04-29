use crate::Can;
use cyphal::{Result, TransferId, Transport};

pub struct CanTransport<C: Can> {
    next_transfer_id: TransferId,
    can: C,
}

impl<C> CanTransport<C>
where
    C: Can,
{
    pub fn new(can: C) -> Result<CanTransport<C>> {
        Ok(CanTransport {
            next_transfer_id: 0,
            can,
        })
    }

    fn next_transfer_id(&mut self) -> TransferId {
        let transfer_id = self.next_transfer_id;

        self.next_transfer_id += 1;

        transfer_id
    }
}

impl<C> Transport for CanTransport<C>
where
    C: Can,
{
    fn new_message_transfer<'a, const PAYLOAD_SIZE: usize, T, M>(
        &self,
        priority: cyphal::Priority,
        subject: cyphal::SubjectId,
        source: Option<cyphal::NodeId>,
        payload: [u8; PAYLOAD_SIZE],
    ) -> Result<M>
    where
        T: Transport,
        M: cyphal::MessageTransfer<'a, PAYLOAD_SIZE, T>,
    {
        todo!()
    }

    fn transmit_message<'a, const PAYLOAD_SIZE: usize, T, M>(&self, message: &M) -> Result<()>
    where
        T: Transport,
        M: cyphal::MessageTransfer<'a, PAYLOAD_SIZE, T>,
    {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::Can;

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
