use crate::{
    can::{Can, CanError, MessageCanId},
    CyphalError, CyphalResult, Message, TransferId, Transport,
};
use embedded_can::Frame;

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
    fn publish<const N: usize, M: Message<N>>(&mut self, message: &M) -> CyphalResult<()> {
        let _ = self.next_transfer_id();
        let can_id =
            MessageCanId::new(message.priority(), message.source(), message.subject()).unwrap();

        let mut payload = message.payload();
        while payload.len() > 0 {
            if payload.len() > 64 {
                let pieces = payload.split_at(64);
                payload = pieces.1;

                match Frame::new(can_id, pieces.0) {
                    Some(frame) => match self.can.transmit(&frame) {
                        Ok(_) => {}
                        Err(e) => return Err(CyphalError::CanError(e)),
                    },
                    None => return Err(CyphalError::CanError(CanError::InvalidFrame)),
                }
            } else {
                match Frame::new(can_id, payload) {
                    Some(frame) => match self.can.transmit(&frame) {
                        Ok(_) => {}
                        Err(e) => return Err(CyphalError::CanError(e)),
                    },
                    None => return Err(CyphalError::CanError(CanError::InvalidFrame)),
                }
                break;
            }
        }

        Ok(())
    }

    fn invoque<const N: usize, const M: usize, R: crate::Request<N, M>>(
        &mut self,
        _: &R,
    ) -> CyphalResult<R::Response> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        can::{Can, CanResult, CanTransport},
        message::test::{MockLargeMessage, MockMessage},
        Priority, Transport,
    };

    struct MockFrame {}
    impl embedded_can::Frame for MockFrame {
        fn new(_: impl Into<embedded_can::Id>, _: &[u8]) -> Option<Self> {
            Some(MockFrame {})
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
    struct MockError {}
    impl embedded_can::Error for MockError {
        fn kind(&self) -> embedded_can::ErrorKind {
            todo!()
        }
    }

    struct MockCan {
        pub sent_frames: u16,
    }

    impl Can for MockCan {
        type Frame = MockFrame;

        type Error = MockError;

        fn transmit(&mut self, _: &Self::Frame) -> CanResult<()> {
            self.sent_frames += 1;
            Ok(())
        }

        fn receive(&mut self) -> CanResult<Self::Frame> {
            todo!()
        }
    }

    #[test]
    fn transmit_small_message() {
        let can = MockCan { sent_frames: 0 };
        let mut transport = CanTransport::new(can).expect("Could not create transport");

        let message = MockMessage::new(Priority::Nominal, 1, None, [0]).unwrap();
        transport.publish(&message).unwrap();

        assert_eq!(transport.can.sent_frames, 1);
    }

    #[test]
    fn transmit_large_message() {
        let can = MockCan { sent_frames: 0 };
        let mut transport = CanTransport::new(can).expect("Could not create transport");

        let message = MockLargeMessage::new(Priority::Nominal, 1, None, [0; 65]).unwrap();
        transport.publish(&message).unwrap();

        assert_eq!(transport.can.sent_frames, 2);
    }
}
