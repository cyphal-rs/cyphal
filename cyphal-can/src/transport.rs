extern crate alloc;

use crate::{
    Can, CanId, CanTransferId, Frame, InboundQueue, MessageCanId, OutboundQueue, ServiceCanId,
    CLASSIC_PAYLOAD_SIZE, FD_PAYLOAD_SIZE,
};
use core::cmp::Ordering;
use crc::Crc;
use cyphal::{CyphalError, CyphalResult, Message, Request, TransferId, Transport};

const CRC16: Crc<u16> = Crc::<u16>::new(&crc::CRC_16_IBM_3740);

/// Represents a CAN Transport
pub struct CanTransport<const PAYLOAD_SIZE: usize, C: Can<PAYLOAD_SIZE>> {
    transfer_id: CanTransferId,
    can: C,
    inbound_queue: InboundQueue<PAYLOAD_SIZE, C::Frame>,
    outbound_queue: OutboundQueue<PAYLOAD_SIZE, C::Frame>,
}

impl<const PAYLOAD_SIZE: usize, C: Can<PAYLOAD_SIZE>> CanTransport<PAYLOAD_SIZE, C> {
    /// Constructs a new CAN transport
    pub fn new(can: C) -> CyphalResult<CanTransport<PAYLOAD_SIZE, C>> {
        assert!(
            PAYLOAD_SIZE == CLASSIC_PAYLOAD_SIZE || PAYLOAD_SIZE == FD_PAYLOAD_SIZE,
            "Invalid PAYLOAD_SIZE value.  Must be 8 for CAN Classic or 64 for CAN FD"
        );

        Ok(CanTransport {
            transfer_id: CanTransferId::new(),
            can,
            inbound_queue: InboundQueue::default(),
            outbound_queue: OutboundQueue::default(),
        })
    }

    fn next_transfer_id(&mut self) -> CanTransferId {
        self.transfer_id = self.transfer_id.next();

        self.transfer_id
    }

    fn enqueue_frames(&mut self, can_id: CanId, mut data: &[u8]) -> CyphalResult<()> {
        let transfer_id = self.next_transfer_id();

        // is multiframe
        if data.len() > PAYLOAD_SIZE - 1 {
            let mut frame_count = 1;

            // CRC-16/CCITT-FALSE checksum
            let checksum = CRC16.checksum(data).to_be_bytes();

            while !data.is_empty() {
                if data.len() > PAYLOAD_SIZE - 1 {
                    let pieces = data.split_at(PAYLOAD_SIZE - 1);
                    data = pieces.1;

                    let mut payload: [u8; PAYLOAD_SIZE] = [0; PAYLOAD_SIZE];

                    // copy the data
                    payload[..pieces.0.len()].copy_from_slice(pieces.0);

                    // add the tail byte
                    payload[PAYLOAD_SIZE - 1] =
                        tail_byte(frame_count == 1, false, frame_count % 2 > 0, transfer_id);

                    match Frame::new(can_id, &payload) {
                        Ok(frame) => self.outbound_queue.push(frame),
                        Err(_) => return Err(CyphalError::Transport),
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

                            match Frame::new(can_id, &payload[..(data.len() + 3)]) {
                                Ok(frame) => self.outbound_queue.push(frame),
                                Err(_) => return Err(CyphalError::Transport),
                            }

                            break;
                        }
                        Ordering::Equal => {
                            // only the firt byte of the crc 16 checksum can fit in this frame
                            payload[PAYLOAD_SIZE - 2] = checksum[0];

                            // add the tail byte
                            payload[PAYLOAD_SIZE - 1] =
                                tail_byte(false, false, frame_count % 2 > 0, transfer_id);

                            match Frame::new(can_id, &payload) {
                                Ok(frame) => self.outbound_queue.push(frame),
                                Err(_) => return Err(CyphalError::Transport),
                            }

                            frame_count += 1;

                            let mut payload: [u8; 2] = [0; 2];

                            // the second byte of the crc 16 checksum goes in this frame
                            payload[0] = checksum[1];

                            // add the tail byte
                            payload[1] = tail_byte(false, true, frame_count % 2 > 0, transfer_id);

                            match Frame::new(can_id, &payload) {
                                Ok(frame) => self.outbound_queue.push(frame),
                                Err(_) => return Err(CyphalError::Transport),
                            }

                            break;
                        }
                        Ordering::Greater => {
                            // crc 16 chcksum must go in another frame

                            // add the tail byte
                            payload[PAYLOAD_SIZE - 1] =
                                tail_byte(false, false, frame_count % 2 > 0, transfer_id);

                            match Frame::new(can_id, &payload) {
                                Ok(frame) => self.outbound_queue.push(frame),
                                Err(_) => return Err(CyphalError::Transport),
                            }

                            frame_count += 1;

                            let mut payload: [u8; 3] = [0; 3];

                            // the crc 16 checksum goes in this frame
                            payload[0] = checksum[0];
                            payload[1] = checksum[1];

                            // add the tail byte
                            payload[2] = tail_byte(false, true, frame_count % 2 > 0, transfer_id);

                            match Frame::new(can_id, &payload) {
                                Ok(frame) => self.outbound_queue.push(frame),
                                Err(_) => return Err(CyphalError::Transport),
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
                Ok(frame) => self.outbound_queue.push(frame),
                Err(_) => return Err(CyphalError::Transport),
            }
        }

        Ok(())
    }
}

impl<const PAYLOAD_SIZE: usize, C: Can<PAYLOAD_SIZE>> Transport for CanTransport<PAYLOAD_SIZE, C> {
    async fn publish<const N: usize, M: Message<N>>(&mut self, message: &M) -> CyphalResult<()> {
        let id =
            MessageCanId::new(message.priority(), message.subject(), message.source()).unwrap();

        self.enqueue_frames(id.into(), message.payload())?;

        while let Some(frame) = self.outbound_queue.pop() {
            match self.can.transmit(&frame).await {
                Ok(()) => {}
                Err(_) => return Err(CyphalError::Transport),
            }
        }
        Ok(())
    }

    async fn invoque<const N: usize, const M: usize, R: Request<N, M>>(
        &mut self,
        request: &R,
    ) -> CyphalResult<R::Response> {
        let id = ServiceCanId::new(
            request.priority(),
            true,
            request.service(),
            request.destination(),
            request.source(),
        )
        .unwrap();

        self.enqueue_frames(id.into(), request.payload())?;

        while let Some(frame) = self.outbound_queue.pop() {
            match self.can.transmit(&frame).await {
                Ok(()) => {}
                Err(_) => return Err(CyphalError::Transport),
            }
        }

        while let Ok(frame) = self.can.receive().await {
            match self.inbound_queue.push(frame) {
                None => {}
                Some(_) => {
                    // FIXME: Need to build response
                    todo!()
                }
            }
        }

        Err(CyphalError::Transport)
    }
}

fn tail_byte(is_start: bool, is_end: bool, toggle: bool, transfer_id: CanTransferId) -> u8 {
    let mut tail_byte = transfer_id.value();
    if is_start {
        tail_byte |= 0x80;
    }
    if is_end {
        tail_byte |= 0x40;
    }
    if toggle {
        tail_byte |= 0x20;
    }

    tail_byte
}

#[cfg(test)]
mod test {
    extern crate std;

    use super::CRC16;
    use crate::{
        test::{
            check_classic_frame, check_fd_frame, TestCan, TestCanFd, TestLargeMessage,
            TestSmallMessage,
        },
        CanTransport,
    };
    use cyphal::{Priority, Transport as _};
    use std::vec::Vec;

    #[async_std::test]
    async fn transmit_small_message() {
        let can = TestCan {
            sent_frames: Vec::new(),
        };
        let mut transport = CanTransport::new(can).expect("Could not create transport");

        let message = TestSmallMessage::new(Priority::Nominal, 1, None, [0; 2]).unwrap();
        transport.publish(&message).await.unwrap();

        assert_eq!(transport.can.sent_frames.len(), 1);
    }

    #[async_std::test]
    async fn transmit_large_message() {
        let can = TestCan {
            sent_frames: Vec::new(),
        };
        let mut transport = CanTransport::new(can).expect("Could not create transport");

        let data: Vec<u8> = (0..65).collect();
        let data: [u8; 65] = data.try_into().unwrap();
        let checksum = CRC16.checksum(&data).to_be_bytes();

        let message = TestLargeMessage::new(Priority::Nominal, 1, None, data).unwrap();
        transport.publish(&message).await.unwrap();

        assert_eq!(transport.can.sent_frames.len(), 10);
        check_classic_frame(
            transport.can.sent_frames[0],
            [0, 1, 2, 3, 4, 5, 6],
            true,
            false,
            true,
        );
        check_classic_frame(
            transport.can.sent_frames[1],
            [7, 8, 9, 10, 11, 12, 13],
            false,
            false,
            false,
        );
        check_classic_frame(
            transport.can.sent_frames[2],
            [14, 15, 16, 17, 18, 19, 20],
            false,
            false,
            true,
        );
        check_classic_frame(
            transport.can.sent_frames[3],
            [21, 22, 23, 24, 25, 26, 27],
            false,
            false,
            false,
        );
        check_classic_frame(
            transport.can.sent_frames[4],
            [28, 29, 30, 31, 32, 33, 34],
            false,
            false,
            true,
        );
        check_classic_frame(
            transport.can.sent_frames[5],
            [35, 36, 37, 38, 39, 40, 41],
            false,
            false,
            false,
        );
        check_classic_frame(
            transport.can.sent_frames[6],
            [42, 43, 44, 45, 46, 47, 48],
            false,
            false,
            true,
        );
        check_classic_frame(
            transport.can.sent_frames[7],
            [49, 50, 51, 52, 53, 54, 55],
            false,
            false,
            false,
        );
        check_classic_frame(
            transport.can.sent_frames[8],
            [56, 57, 58, 59, 60, 61, 62],
            false,
            false,
            true,
        );

        assert_eq!(transport.can.sent_frames[9].dlc, 5);
        assert_eq!(transport.can.sent_frames[9].data[0], 63);
        assert_eq!(transport.can.sent_frames[9].data[1], 64);
        assert_eq!(transport.can.sent_frames[9].data[2], checksum[0]);
        assert_eq!(transport.can.sent_frames[9].data[3], checksum[1]);

        let tail_byte = transport.can.sent_frames[9].data[4];
        assert_eq!(tail_byte & 0x80 > 0, false);
        assert_eq!(tail_byte & 0x40 > 0, true);
        assert_eq!(tail_byte & 0x20 > 0, false);
    }

    #[async_std::test]
    async fn transmit_large_message_fd() {
        let can = TestCanFd {
            sent_frames: Vec::new(),
        };
        let mut transport = CanTransport::new(can).expect("Could not create transport");

        let data: Vec<u8> = (0..65).collect();
        let data: [u8; 65] = data.try_into().unwrap();
        let checksum = CRC16.checksum(&data).to_be_bytes();

        let message = TestLargeMessage::new(Priority::Nominal, 1, None, data).unwrap();
        transport.publish(&message).await.unwrap();

        assert_eq!(transport.can.sent_frames.len(), 2);

        let data: Vec<u8> = (0..63).collect();
        let data: [u8; 63] = data.try_into().unwrap();
        check_fd_frame(transport.can.sent_frames[0], data, true, false, true);

        assert_eq!(transport.can.sent_frames[1].dlc, 5);
        assert_eq!(transport.can.sent_frames[1].data[2], checksum[0]);
        assert_eq!(transport.can.sent_frames[1].data[3], checksum[1]);

        let tail_byte = transport.can.sent_frames[1].data[4];
        assert_eq!(tail_byte & 0x80 > 0, false);
        assert_eq!(tail_byte & 0x40 > 0, true);
        assert_eq!(tail_byte & 0x20 > 0, false);
    }
}
