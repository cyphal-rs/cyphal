extern crate std;

use crate::{Can, CanError, CanId, CanResult, Frame, CLASSIC_PAYLOAD_SIZE};
use std::vec::Vec;

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

pub struct TestCan {
    pub sent_frames: Vec<TestFrame>,
    pub receive_fn: fn() -> CanResult<TestFrame>,
}

impl Can<CLASSIC_PAYLOAD_SIZE> for TestCan {
    type Frame = TestFrame;

    async fn transmit(&mut self, frame: &Self::Frame) -> CanResult<()> {
        self.sent_frames.push(*frame);
        Ok(())
    }

    async fn receive(&mut self) -> CanResult<Self::Frame> {
        (self.receive_fn)()
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
