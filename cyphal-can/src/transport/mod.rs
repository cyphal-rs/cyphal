mod inbound_queue;
use inbound_queue::InboundQueue;

mod outbound_queue;
use outbound_queue::OutboundQueue;

extern crate alloc;

use crate::{
    Can, CanId, CanNodeId, CanServiceId, CanSubjectId, CanTransferId, Frame, MessageCanId,
    ServiceCanId, CLASSIC_PAYLOAD_SIZE, FD_PAYLOAD_SIZE,
};
use alloc::vec::Vec;
use core::cmp::Ordering;
use crc::Crc;
use cyphal::{CyphalError, CyphalResult, Message, Request, Response, TransferId, Transport};

const CRC16: Crc<u16> = Crc::<u16>::new(&crc::CRC_16_IBM_3740);

/// Represents a CAN Transport
pub struct CanTransport<const PAYLOAD_SIZE: usize, C: Can<PAYLOAD_SIZE>> {
    can: C,
    transfer: CanTransferId,
    inbound_queue: InboundQueue<PAYLOAD_SIZE, C::Frame>,
    outbound_queue: OutboundQueue<PAYLOAD_SIZE, C::Frame>,
}

impl<const PAYLOAD_SIZE: usize, C: Can<PAYLOAD_SIZE>> CanTransport<PAYLOAD_SIZE, C> {
    /// Constructs a new CAN transport
    pub fn new(can: C) -> CyphalResult<Self> {
        assert!(
            PAYLOAD_SIZE == CLASSIC_PAYLOAD_SIZE || PAYLOAD_SIZE == FD_PAYLOAD_SIZE,
            "Invalid PAYLOAD_SIZE value.  Must be 8 for CAN Classic or 64 for CAN FD"
        );

        Ok(Self {
            can,
            transfer: CanTransferId::default(),
            inbound_queue: InboundQueue::default(),
            outbound_queue: OutboundQueue::default(),
        })
    }

    fn next_transfer(&mut self) -> CanTransferId {
        self.transfer = self.transfer.next();

        self.transfer
    }

    fn enqueue_frames(&mut self, can_id: CanId, mut data: &[u8]) -> CyphalResult<CanTransferId> {
        let transfer = self.next_transfer();

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
                        tail_byte(frame_count == 1, false, frame_count % 2 > 0, transfer);

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
                                tail_byte(false, true, frame_count % 2 > 0, transfer);

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
                                tail_byte(false, false, frame_count % 2 > 0, transfer);

                            match Frame::new(can_id, &payload) {
                                Ok(frame) => self.outbound_queue.push(frame),
                                Err(_) => return Err(CyphalError::Transport),
                            }

                            frame_count += 1;

                            let mut payload: [u8; 2] = [0; 2];

                            // the second byte of the crc 16 checksum goes in this frame
                            payload[0] = checksum[1];

                            // add the tail byte
                            payload[1] = tail_byte(false, true, frame_count % 2 > 0, transfer);

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
                                tail_byte(false, false, frame_count % 2 > 0, transfer);

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
                            payload[2] = tail_byte(false, true, frame_count % 2 > 0, transfer);

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
            payload[PAYLOAD_SIZE - 1] = tail_byte(true, true, true, transfer);

            match Frame::new(can_id, &payload) {
                Ok(frame) => self.outbound_queue.push(frame),
                Err(_) => return Err(CyphalError::Transport),
            }
        }

        Ok(transfer)
    }
}

impl<const PAYLOAD_SIZE: usize, C: Can<PAYLOAD_SIZE>> Transport for CanTransport<PAYLOAD_SIZE, C> {
    type NodeId = CanNodeId;
    type ServiceId = CanServiceId;
    type SubjectId = CanSubjectId;

    async fn publish<M>(&mut self, message: &M) -> CyphalResult<()>
    where
        M: Message<Self::SubjectId, Self::NodeId>,
    {
        let id =
            MessageCanId::new(message.priority(), message.subject(), message.source()).unwrap();

        self.enqueue_frames(id.into(), message.data())?;

        while let Some(frame) = self.outbound_queue.pop() {
            match self.can.transmit(&frame).await {
                Ok(()) => {}
                Err(_) => return Err(CyphalError::Transport),
            }
        }
        Ok(())
    }

    async fn invoque<R>(&mut self, request: &R) -> CyphalResult<R::Response>
    where
        R: Request<Self::ServiceId, Self::NodeId>,
    {
        let id = ServiceCanId::new(
            request.priority(),
            true,
            request.service(),
            request.destination(),
            request.source(),
        )
        .unwrap();

        let transfer = self.enqueue_frames(id.into(), request.data())?;

        while let Some(frame) = self.outbound_queue.pop() {
            match self.can.transmit(&frame).await {
                Ok(()) => {}
                Err(_) => return Err(CyphalError::Transport),
            }
        }

        while let Ok(frame) = self.can.receive().await {
            self.inbound_queue.push(frame);

            match self.inbound_queue.get_transfer_frames(transfer) {
                None => {}
                Some(mut queue) => {
                    let first_frame = queue.pop_front().unwrap();
                    let id = match first_frame.id() {
                        CanId::Message(_) => return Err(CyphalError::Transport),
                        CanId::Service(id) => id,
                    };

                    let mut payload: Vec<u8> = Vec::new();

                    if first_frame.is_single_trame_transfer() {
                        if R::Response::SIZE > PAYLOAD_SIZE - 1 {
                            // something went wrong
                            return Err(CyphalError::Transport);
                        }

                        let data = first_frame.data();
                        for value in data.iter().take(R::Response::SIZE) {
                            payload.push(*value);
                        }

                        return R::Response::new(
                            id.priority(),
                            id.service(),
                            id.destination(),
                            id.source(),
                            &payload,
                        );
                    } else {
                        if !first_frame.is_start_of_transfer() || !first_frame.is_toggle_bit_set() {
                            // something went wrong
                            return Err(CyphalError::Transport);
                        }

                        payload.extend_from_slice(&first_frame.data()[..first_frame.dlc() - 2]);

                        let mut toogle = false;

                        while let Some(frame) = queue.pop_front() {
                            if frame.is_start_of_transfer() || frame.is_toggle_bit_set() != toogle {
                                // something went wrong
                                return Err(CyphalError::Transport);
                            }
                            if frame.is_end_of_transfer() && !queue.is_empty() {
                                // something went wrong
                                return Err(CyphalError::Transport);
                            }

                            payload.extend_from_slice(&frame.data()[..frame.dlc() - 2]);
                            toogle = !toogle
                        }

                        if payload.len() != R::Response::SIZE {
                            // something went wrong
                            return Err(CyphalError::Transport);
                        }

                        return R::Response::new(
                            id.priority(),
                            id.service(),
                            id.destination(),
                            id.source(),
                            &payload,
                        );
                    }
                }
            }
        }

        Err(CyphalError::Transport)
    }
}

fn tail_byte(is_start: bool, is_end: bool, toggle: bool, transfer: CanTransferId) -> u8 {
    let mut tail_byte = transfer.value();
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
            check_classic_frame, check_fd_frame, TestCan, TestCanFd, TestFrame, TestLargeMessage,
            TestRequest, TestSmallMessage, LARGE_MESSAGE_SIZE, TEST_REQUEST_SIZE,
        },
        CanError, CanTransport, Frame, ServiceCanId,
    };
    use cyphal::{Priority, Response, Transport as _};
    use std::vec::Vec;

    #[async_std::test]
    async fn transmit_small_message() {
        let can = TestCan {
            sent_frames: Vec::new(),
            receive_fn: || Err(CanError::Other),
        };
        let mut transport = CanTransport::new(can).expect("Could not create transport");

        let message =
            TestSmallMessage::new(Priority::Nominal, 1.try_into().unwrap(), None, [1, 2]).unwrap();
        transport.publish(&message).await.unwrap();

        assert_eq!(transport.can.sent_frames.len(), 1);
    }

    #[async_std::test]
    async fn transmit_large_message() {
        let can = TestCan {
            sent_frames: Vec::new(),
            receive_fn: || Err(CanError::Other),
        };
        let mut transport = CanTransport::new(can).expect("Could not create transport");

        let data: Vec<u8> = (0..LARGE_MESSAGE_SIZE as u8).collect();
        let data: [u8; LARGE_MESSAGE_SIZE] = data.try_into().unwrap();
        let checksum = CRC16.checksum(&data).to_be_bytes();

        let message =
            TestLargeMessage::new(Priority::Nominal, 1.try_into().unwrap(), None, data).unwrap();
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

        let message =
            TestLargeMessage::new(Priority::Nominal, 1.try_into().unwrap(), None, data).unwrap();
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

    #[async_std::test]
    async fn test_invoque() {
        let can = TestCan {
            sent_frames: Vec::new(),
            receive_fn: || {
                let id = ServiceCanId::new(
                    Priority::Nominal,
                    false,
                    1.try_into().unwrap(),
                    3.try_into().unwrap(),
                    2.try_into().unwrap(),
                )
                .unwrap();
                let frame = TestFrame::new(id, &[1, 2, 0, 0, 0, 0, 0, 0xE1]).unwrap();

                Ok(frame)
            },
        };
        let mut transport = CanTransport::new(can).expect("Could not create transport");

        let data: Vec<u8> = (0..TEST_REQUEST_SIZE as u8).collect();
        let data: [u8; TEST_REQUEST_SIZE] = data.try_into().unwrap();

        let request = TestRequest::new(
            Priority::Nominal,
            1.try_into().unwrap(),
            2.try_into().unwrap(),
            3.try_into().unwrap(),
            data,
        )
        .unwrap();
        let response = transport.invoque(&request).await.unwrap();

        assert_eq!(transport.can.sent_frames.len(), 1);
        assert_eq!(response.data()[0], 1);
        assert_eq!(response.data()[1], 2);
    }
}
