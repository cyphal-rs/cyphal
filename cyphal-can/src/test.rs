extern crate std;

use crate::{Can, CanError, CanId, CanResult, Frame, CLASSIC_PAYLOAD_SIZE, FD_PAYLOAD_SIZE};
use cyphal::{CyphalResult, Message, NodeId, Priority, SubjectId};
use std::vec::Vec;

pub const SMALL_MESSAGE_SIZE: usize = 2;
pub const LARGE_MESSAGE_SIZE: usize = 65;

#[derive(Debug, Copy, Clone)]
pub struct TestFrame {
    pub id: CanId,
    pub dlc: usize,
    pub data: [u8; CLASSIC_PAYLOAD_SIZE],
}

impl Frame<CLASSIC_PAYLOAD_SIZE> for TestFrame {
    fn new(id: impl Into<CanId>, data: &[u8]) -> CanResult<Self> {
        match data.len() {
            dlc if dlc <= CLASSIC_PAYLOAD_SIZE => {
                let mut bytes: [u8; CLASSIC_PAYLOAD_SIZE] = [0; CLASSIC_PAYLOAD_SIZE];
                bytes[..dlc].copy_from_slice(data);
                Ok(TestFrame {
                    id: id.into(),
                    dlc,
                    data: bytes,
                })
            }
            _ => Err(CanError::Other),
        }
    }

    fn id(&self) -> CanId {
        self.id
    }

    fn dlc(&self) -> usize {
        self.dlc
    }

    fn data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TestFdFrame {
    pub id: CanId,
    pub dlc: usize,
    pub data: [u8; FD_PAYLOAD_SIZE],
}

impl Frame<FD_PAYLOAD_SIZE> for TestFdFrame {
    fn new(id: impl Into<CanId>, data: &[u8]) -> CanResult<Self> {
        match data.len() {
            dlc if dlc <= FD_PAYLOAD_SIZE => {
                let mut bytes: [u8; FD_PAYLOAD_SIZE] = [0; FD_PAYLOAD_SIZE];
                bytes[..dlc].copy_from_slice(data);
                Ok(TestFdFrame {
                    id: id.into(),
                    dlc,
                    data: bytes,
                })
            }
            _ => Err(CanError::Other),
        }
    }

    fn id(&self) -> CanId {
        self.id
    }

    fn dlc(&self) -> usize {
        self.dlc
    }

    fn data(&self) -> &[u8] {
        &self.data
    }
}

pub struct TestSmallMessage {
    priority: Priority,
    subject: SubjectId,
    source: Option<NodeId>,
    payload: [u8; SMALL_MESSAGE_SIZE],
}

impl TestSmallMessage {
    pub fn new(
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
        payload: [u8; SMALL_MESSAGE_SIZE],
    ) -> CyphalResult<Self> {
        Ok(Self {
            priority,
            subject,
            source,
            payload,
        })
    }
}

impl Message<1> for TestSmallMessage {
    type Payload = [u8; SMALL_MESSAGE_SIZE];

    fn priority(&self) -> Priority {
        self.priority
    }

    fn subject(&self) -> SubjectId {
        self.subject
    }

    fn source(&self) -> Option<NodeId> {
        self.source
    }

    fn payload(&self) -> &[u8] {
        &self.payload
    }
}

pub struct TestLargeMessage {
    priority: Priority,
    subject: SubjectId,
    source: Option<NodeId>,
    payload: [u8; LARGE_MESSAGE_SIZE],
}

impl TestLargeMessage {
    pub fn new(
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
        payload: [u8; LARGE_MESSAGE_SIZE],
    ) -> CyphalResult<Self> {
        Ok(Self {
            priority,
            subject,
            source,
            payload,
        })
    }
}

impl Message<LARGE_MESSAGE_SIZE> for TestLargeMessage {
    type Payload = [u8; LARGE_MESSAGE_SIZE];

    fn source(&self) -> Option<NodeId> {
        self.source
    }

    fn subject(&self) -> SubjectId {
        self.subject
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn payload(&self) -> &[u8] {
        &self.payload
    }
}

pub struct TestCan {
    pub sent_frames: Vec<TestFrame>,
}

impl Can<8> for TestCan {
    type Frame = TestFrame;

    async fn transmit(&mut self, frame: &Self::Frame) -> CanResult<()> {
        self.sent_frames.push(*frame);
        Ok(())
    }

    async fn receive(&mut self) -> CanResult<Self::Frame> {
        todo!()
    }
}
pub struct TestCanFd {
    pub sent_frames: Vec<TestFdFrame>,
}

impl Can<64> for TestCanFd {
    type Frame = TestFdFrame;

    async fn transmit(&mut self, frame: &Self::Frame) -> CanResult<()> {
        self.sent_frames.push(*frame);
        Ok(())
    }

    async fn receive(&mut self) -> CanResult<Self::Frame> {
        todo!()
    }
}

pub fn check_classic_frame(
    frame: TestFrame,
    data: [u8; CLASSIC_PAYLOAD_SIZE - 1],
    start_of_transfer: bool,
    end_of_transfer: bool,
    toogle: bool,
) {
    assert_eq!(frame.dlc, CLASSIC_PAYLOAD_SIZE);

    for (i, v) in data.iter().enumerate() {
        assert_eq!(frame.data[i], *v);
    }

    let tail_byte = frame.data[CLASSIC_PAYLOAD_SIZE - 1];
    assert_eq!(tail_byte & 0x80 > 0, start_of_transfer);
    assert_eq!(tail_byte & 0x40 > 0, end_of_transfer);
    assert_eq!(tail_byte & 0x20 > 0, toogle);
}

pub fn check_fd_frame(
    frame: TestFdFrame,
    data: [u8; FD_PAYLOAD_SIZE - 1],
    start_of_transfer: bool,
    end_of_transfer: bool,
    toogle: bool,
) {
    assert_eq!(frame.dlc, FD_PAYLOAD_SIZE);

    for (i, v) in data.iter().enumerate() {
        assert_eq!(frame.data[i], *v);
    }

    let tail_byte = frame.data[FD_PAYLOAD_SIZE - 1];
    assert_eq!(tail_byte & 0x80 > 0, start_of_transfer);
    assert_eq!(tail_byte & 0x40 > 0, end_of_transfer);
    assert_eq!(tail_byte & 0x20 > 0, toogle);
}
