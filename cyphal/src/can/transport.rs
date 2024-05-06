use super::{tail_byte, CanTransferId};
use crate::{
    can::{Can, CanError, MessageCanId, TransferId},
    CyphalError, CyphalResult, Message, Transport,
};
use core::cmp::Ordering;
use crc::Crc;
use embedded_can::Frame;

const CRC16: Crc<u16> = Crc::<u16>::new(&crc::CRC_16_IBM_3740);

#[cfg(feature = "can")]
const PAYLOAD_SIZE: usize = 8;

#[cfg(feature = "canfd")]
const PAYLOAD_SIZE: usize = 64;

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

    fn send_frame(
        &mut self,
        can_id: MessageCanId,
        payload: &[u8],
    ) -> Option<Result<(), CyphalError>> {
        match Frame::new(can_id, &payload) {
            Some(frame) => match self.can.transmit(&frame) {
                Ok(_) => {}
                Err(e) => return Some(Err(CyphalError::CanError(e))),
            },
            None => return Some(Err(CyphalError::CanError(CanError::InvalidFrame))),
        }
        None
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

        let mut data = message.payload();

        // is multiframe
        if data.len() > PAYLOAD_SIZE - 1 {
            let mut frame_count = 1;

            // CRC-16/CCITT-FALSE checksum
            let checksum = CRC16.checksum(data).to_be_bytes();

            while data.len() > 0 {
                if data.len() > PAYLOAD_SIZE - 1 {
                    let pieces = data.split_at(PAYLOAD_SIZE - 1);
                    data = pieces.1;

                    let mut payload: [u8; PAYLOAD_SIZE] = [0; PAYLOAD_SIZE];

                    // copy the data
                    payload[..pieces.0.len()].copy_from_slice(pieces.0);

                    // add the tail byte
                    payload[PAYLOAD_SIZE - 1] =
                        tail_byte(frame_count == 1, false, frame_count % 2 > 0, transfer_id);

                    if let Some(value) = self.send_frame(can_id, &payload) {
                        return value;
                    }
                } else {
                    let mut payload: [u8; PAYLOAD_SIZE] = [0; PAYLOAD_SIZE];

                    // copy the data
                    payload[..data.len()].copy_from_slice(data);

                    match data.len().cmp(&(PAYLOAD_SIZE - 2)) {
                        Ordering::Less => {
                            // crc 16 checksum can fit in this frame
                            payload[data.len()] = checksum[0];
                            payload[data.len() + 1] = checksum[1];

                            // add the tail byte
                            payload[data.len() + 2] =
                                tail_byte(false, true, frame_count % 2 > 0, transfer_id);

                            if let Some(value) =
                                self.send_frame(can_id, &payload[..(data.len() + 3)])
                            {
                                return value;
                            }

                            break;
                        }
                        Ordering::Equal => {
                            // only the firt byte of the crc 16 checksum can fit in this frame
                            payload[PAYLOAD_SIZE - 2] = checksum[0];

                            // add the tail byte
                            payload[PAYLOAD_SIZE - 1] =
                                tail_byte(false, false, frame_count % 2 > 0, transfer_id);

                            if let Some(value) = self.send_frame(can_id, &payload) {
                                return value;
                            }

                            frame_count += 1;

                            let mut payload: [u8; 2] = [0; 2];

                            // the second byte of the crc 16 checksum goes in this frame
                            payload[0] = checksum[1];

                            // add the tail byte
                            payload[1] = tail_byte(false, true, frame_count % 2 > 0, transfer_id);

                            if let Some(value) = self.send_frame(can_id, &payload) {
                                return value;
                            }

                            break;
                        }
                        Ordering::Greater => {
                            // crc 16 chcksum must go in another frame

                            // add the tail byte
                            payload[PAYLOAD_SIZE - 1] =
                                tail_byte(false, false, frame_count % 2 > 0, transfer_id);

                            if let Some(value) = self.send_frame(can_id, &payload) {
                                return value;
                            }

                            frame_count += 1;

                            let mut payload: [u8; 3] = [0; 3];

                            // the crc 16 checksum goes in this frame
                            payload[0] = checksum[0];
                            payload[1] = checksum[1];

                            // add the tail byte
                            payload[2] = tail_byte(false, true, frame_count % 2 > 0, transfer_id);

                            if let Some(value) = self.send_frame(can_id, &payload) {
                                return value;
                            }

                            break;
                        }
                    }
                }
                frame_count += 1;
            }
        } else {
            // single frame
            let mut payload: [u8; PAYLOAD_SIZE] = [0; PAYLOAD_SIZE];

            // copy the data
            payload[..data.len()].copy_from_slice(data);

            // add the tail byte
            payload[PAYLOAD_SIZE - 1] = tail_byte(true, true, true, transfer_id);

            match Frame::new(can_id, &payload) {
                Some(frame) => match self.can.transmit(&frame) {
                    Ok(_) => {}
                    Err(e) => return Err(CyphalError::CanError(e)),
                },
                None => return Err(CyphalError::CanError(CanError::InvalidFrame)),
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
    extern crate std;

    use super::PAYLOAD_SIZE;
    use crate::{
        can::{Can, CanResult, CanTransport},
        message::test::{MockLargeMessage, MockMessage},
        Priority, Transport,
    };
    use embedded_can::{Frame, Id};
    use std::vec::Vec;

    #[derive(Debug, Copy, Clone)]
    struct MockFrame {
        dlc: usize,
        data: [u8; PAYLOAD_SIZE],
    }
    impl Frame for MockFrame {
        fn new(_: impl Into<Id>, data: &[u8]) -> Option<Self> {
            match data.len() {
                n if n <= PAYLOAD_SIZE => {
                    let mut bytes: [u8; PAYLOAD_SIZE] = [0; PAYLOAD_SIZE];
                    bytes[..n].copy_from_slice(data);
                    Some(MockFrame {
                        dlc: data.len(),
                        data: bytes,
                    })
                }
                _ => None,
            }
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
        pub sent_frames: Vec<MockFrame>,
    }

    impl Can for MockCan {
        type Frame = MockFrame;

        type Error = MockError;

        fn transmit(&mut self, frame: &Self::Frame) -> CanResult<()> {
            self.sent_frames.push(*frame);
            Ok(())
        }

        fn receive(&mut self) -> CanResult<Self::Frame> {
            todo!()
        }
    }

    #[test]
    fn transmit_small_message() {
        let can = MockCan {
            sent_frames: Vec::new(),
        };
        let mut transport = CanTransport::new(can).expect("Could not create transport");

        let message = MockMessage::new(Priority::Nominal, 1, None, [0]).unwrap();
        transport.publish(&message).unwrap();

        assert_eq!(transport.can.sent_frames.len(), 1);
    }

    #[test]
    #[cfg(feature = "can")]
    fn transmit_large_message() {
        use crate::can::transport::CRC16;

        let can = MockCan {
            sent_frames: Vec::new(),
        };
        let mut transport = CanTransport::new(can).expect("Could not create transport");

        let data: [u8; 65] = [255; 65];
        let checksum = CRC16.checksum(&data).to_be_bytes();

        let message = MockLargeMessage::new(Priority::Nominal, 1, None, data).unwrap();
        transport.publish(&message).unwrap();

        assert_eq!(transport.can.sent_frames.len(), 10);
        check_frame(transport.can.sent_frames[0], true, false, true);
        check_frame(transport.can.sent_frames[1], false, false, false);
        check_frame(transport.can.sent_frames[2], false, false, true);
        check_frame(transport.can.sent_frames[3], false, false, false);
        check_frame(transport.can.sent_frames[4], false, false, true);
        check_frame(transport.can.sent_frames[5], false, false, false);
        check_frame(transport.can.sent_frames[6], false, false, true);
        check_frame(transport.can.sent_frames[7], false, false, false);
        check_frame(transport.can.sent_frames[8], false, false, true);

        assert_eq!(transport.can.sent_frames[9].dlc, 5);
        assert_eq!(transport.can.sent_frames[9].data[2], checksum[0]);
        assert_eq!(transport.can.sent_frames[9].data[3], checksum[1]);

        let tail_byte = transport.can.sent_frames[9].data[4];
        assert_eq!(tail_byte & 0x80 > 0, false);
        assert_eq!(tail_byte & 0x40 > 0, true);
        assert_eq!(tail_byte & 0x20 > 0, false);
    }

    #[test]
    #[cfg(feature = "canfd")]
    fn transmit_large_message() {
        let can = MockCan {
            sent_frames: Vec::new(),
        };
        let mut transport = CanTransport::new(can).expect("Could not create transport");

        let message = MockLargeMessage::new(Priority::Nominal, 1, None, [255; 65]).unwrap();
        transport.publish(&message).unwrap();

        assert_eq!(transport.can.sent_frames.len(), 2);
        check_frame(transport.can.sent_frames[0], true, false, true);
        check_frame(transport.can.sent_frames[1], false, true, false);
    }

    fn check_frame(frame: MockFrame, start_of_transfer: bool, end_of_transfer: bool, toogle: bool) {
        #[cfg(feature = "can")]
        assert_eq!(frame.dlc, 8);

        #[cfg(feature = "canfd")]
        assert_eq!(frame.dlc, 64);

        #[cfg(feature = "can")]
        let tail_byte = frame.data[7];

        #[cfg(feature = "canfd")]
        let tail_byte = frame.data[63];
        assert_eq!(tail_byte & 0x80 > 0, start_of_transfer);
        assert_eq!(tail_byte & 0x40 > 0, end_of_transfer);
        assert_eq!(tail_byte & 0x20 > 0, toogle);
    }
}
