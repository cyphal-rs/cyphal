extern crate alloc;

use crate::{CanTransferId, Frame};
use alloc::collections::{BTreeMap, VecDeque};
use cyphal::TransferId;

pub struct InboundQueue<const PAYLOAD_SIZE: usize, F: Frame<PAYLOAD_SIZE>> {
    pending: BTreeMap<u8, VecDeque<F>>,
    complete: BTreeMap<u8, VecDeque<F>>,
}

impl<const PAYLOAD_SIZE: usize, F: Frame<PAYLOAD_SIZE>> InboundQueue<PAYLOAD_SIZE, F> {
    pub fn new() -> Self {
        Self {
            pending: BTreeMap::new(),
            complete: BTreeMap::new(),
        }
    }

    pub fn push(&mut self, frame: F) {
        // Check to see if it's a single frame transfer
        if frame.is_single_trame_transfer() {
            let transfer_id = frame.transfer_id().value();
            let queue = VecDeque::from([frame]);
            self.complete.insert(transfer_id, queue);

            return;
        }

        let transfer_id = frame.transfer_id().value();

        // if it's the last frame, remove the frames from pending and add to complete
        if frame.is_end_of_transfer() {
            if let Some(mut queue) = self.pending.remove(&transfer_id) {
                queue.push_back(frame);
                self.complete.insert(transfer_id, queue);
            }
        } else {
            match self.pending.get_mut(&transfer_id) {
                Some(queue) => queue.push_back(frame),
                None => {
                    // only add transmission if it's the start of one
                    if frame.is_start_of_transfer() {
                        let queue = VecDeque::from([frame]);
                        self.pending.insert(transfer_id, queue);
                    }
                }
            }
        }
    }

    pub fn get_transfer_frames(&mut self, transfer_id: CanTransferId) -> Option<VecDeque<F>> {
        self.complete.remove(&transfer_id.value())
    }
}

impl<const PAYLOAD_SIZE: usize, F: Frame<PAYLOAD_SIZE>> Default for InboundQueue<PAYLOAD_SIZE, F> {
    fn default() -> Self {
        Self::new()
    }
}
