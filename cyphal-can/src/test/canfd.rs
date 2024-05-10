extern crate std;

use crate::{Can, CanError, CanId, CanResult, Frame, FD_PAYLOAD_SIZE};
use std::vec::Vec;

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
