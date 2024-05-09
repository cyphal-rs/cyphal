extern crate alloc;

use crate::{CanId, Frame};
use alloc::collections::{BTreeMap, VecDeque};

pub struct InboundQueue<const PAYLOAD_SIZE: usize, F: Frame<PAYLOAD_SIZE>> {
    frames: BTreeMap<CanId, VecDeque<F>>,
}

impl<const PAYLOAD_SIZE: usize, F: Frame<PAYLOAD_SIZE>> InboundQueue<PAYLOAD_SIZE, F> {
    pub fn new() -> Self {
        Self {
            frames: BTreeMap::new(),
        }
    }

    pub fn push(&mut self, frame: F) -> Option<VecDeque<F>> {
        let id = frame.id();

        // Check to see if it's a single frame transfer
        if frame.dlc() == PAYLOAD_SIZE && frame.data()[PAYLOAD_SIZE - 1] & 0xE0 == 0xE0 {
            return Some(VecDeque::from([frame]));
        }

        if let Some(queue) = self.frames.get_mut(&id) {
            // Check to see if it's the last frame of a Multi-frame transfers
            let is_last = frame.data()[frame.dlc() - 1] & 0x40 != 0;

            queue.push_back(frame);

            if is_last {
                return Some(self.frames.remove(&id).unwrap());
            }
        } else {
            self.frames.insert(frame.id(), VecDeque::from([frame]));
        }

        None
    }
}

impl<const PAYLOAD_SIZE: usize, F: Frame<PAYLOAD_SIZE>> Default for InboundQueue<PAYLOAD_SIZE, F> {
    fn default() -> Self {
        Self::new()
    }
}
