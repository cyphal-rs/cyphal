use super::{tail_byte, CanTransferId};
use crate::{
    can::{Can, CanError, MessageCanId, TransferId},
    CyphalError, CyphalResult, Message, Transport,
};
use embedded_can::Frame;

const CAN_MAX_PAYLOAD_SIZE: usize = 7;
const CAN_FD_MAX_PAYLOAD_SIZE: usize = 63;

pub struct CanTransport<C: Can> {
    transfer_id: CanTransferId,
    can: C,
}

impl<C> CanTransport<C>
where
    C: Can,
{
    pub fn new(can: C) -> CyphalResult<CanTransport<C>> {
        Ok(CanTransport {
            transfer_id: CanTransferId::new(0),
            can,
        })
    }

    fn next_transfer_id(&mut self) -> CanTransferId {
        self.transfer_id = self.transfer_id.next();

        self.transfer_id
    }
}

impl<C> Transport for CanTransport<C>
where
    C: Can,
{
    fn publish<const N: usize, M: Message<N>>(&mut self, message: &M) -> CyphalResult<()> {
        let transfer_id = self.next_transfer_id();
        let can_id =
            MessageCanId::new(message.priority(), message.source(), message.subject()).unwrap();

        let max_payload_size = if self.can.is_fd() {
            CAN_FD_MAX_PAYLOAD_SIZE
        } else {
            CAN_MAX_PAYLOAD_SIZE
        };
        let mut data = message.payload();

        while data.len() > 0 {
            if data.len() > max_payload_size {
                let pieces = data.split_at(max_payload_size);
                data = pieces.1;

                // Add tail byte
                let t = tail_byte(true, true, true, transfer_id);
                let payload = &[pieces.0, &[t]].concat();

                match Frame::new(can_id, payload) {
                    Some(frame) => match self.can.transmit(&frame) {
                        Ok(_) => {}
                        Err(e) => return Err(CyphalError::CanError(e)),
                    },
                    None => return Err(CyphalError::CanError(CanError::InvalidFrame)),
                }
            } else {
                match Frame::new(can_id, data) {
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

        fn is_fd(&self) -> bool {
            false
        }

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
