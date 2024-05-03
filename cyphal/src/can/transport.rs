use crate::{can::Can, CyphalError, CyphalResult, MessageTransfer, TransferId, Transport};
use embedded_can::Frame;

use super::MessageCanId;

pub struct CanTransport<C: Can> {
    transfer_id: TransferId,
    can: C,
}

impl<C> CanTransport<C>
where
    C: Can,
{
    pub fn new(can: C) -> CyphalResult<CanTransport<C>> {
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
    fn transmit_message<const PAYLOAD_SIZE: usize, M>(
        &mut self,
        message: &M,
    ) -> CyphalResult<TransferId>
    where
        M: MessageTransfer<PAYLOAD_SIZE>,
    {
        let id = self.next_transfer_id();

        let can_id =
            MessageCanId::new(message.priority(), message.source(), message.subject()).unwrap();
        //TODO: send payload
        let mut payload = message.payload();
        while payload.len() > 64 {
            let pieces = payload.split_at(64);
            payload = pieces.1;

            let option = Frame::new(can_id, pieces.0);
            match option {
                Some(frame) => {
                    let result = self.can.transmit(&frame);
                    match result {
                        Ok(_) => {}
                        Err(e) => return Err(CyphalError::CanError(e)),
                    }
                }
                None => return Err(CyphalError::NotDefined),
            }
        }

        Ok(id)
    }
}

#[cfg(test)]
mod test {
    use crate::can::{Can, CanResult};

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

        fn transmit(&mut self, _: &Self::Frame) -> CanResult<()> {
            Ok(())
        }

        fn receive(&mut self) -> CanResult<Self::Frame> {
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
