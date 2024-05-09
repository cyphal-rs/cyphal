extern crate alloc;

use crate::Frame;
use alloc::collections::VecDeque;
use cyphal::Priority;

pub struct Queue<const PAYLOAD_SIZE: usize, F: Frame<PAYLOAD_SIZE>> {
    frames: [VecDeque<F>; 8],
}

impl<const PAYLOAD_SIZE: usize, F: Frame<PAYLOAD_SIZE>> Queue<PAYLOAD_SIZE, F> {
    pub fn new() -> Self {
        Queue {
            frames: [
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
            ],
        }
    }

    pub fn push(&mut self, frame: F) {
        match frame.id().priority() {
            Priority::Exceptional => self.frames[0].push_back(frame),
            Priority::Immediate => self.frames[1].push_back(frame),
            Priority::Fast => self.frames[2].push_back(frame),
            Priority::High => self.frames[3].push_back(frame),
            Priority::Nominal => self.frames[4].push_back(frame),
            Priority::Low => self.frames[5].push_back(frame),
            Priority::Slow => self.frames[6].push_back(frame),
            Priority::Optional => self.frames[7].push_back(frame),
        }
    }

    pub fn pop(&mut self) -> Option<F> {
        if let Some(frame) = self.frames[0].pop_front() {
            return Some(frame);
        }
        if let Some(frame) = self.frames[1].pop_front() {
            return Some(frame);
        }
        if let Some(frame) = self.frames[2].pop_front() {
            return Some(frame);
        }
        if let Some(frame) = self.frames[3].pop_front() {
            return Some(frame);
        }
        if let Some(frame) = self.frames[4].pop_front() {
            return Some(frame);
        }
        if let Some(frame) = self.frames[5].pop_front() {
            return Some(frame);
        }
        if let Some(frame) = self.frames[6].pop_front() {
            return Some(frame);
        }
        if let Some(frame) = self.frames[7].pop_front() {
            return Some(frame);
        }

        None
    }
}

impl<const PAYLOAD_SIZE: usize, F: Frame<PAYLOAD_SIZE>> Default for Queue<PAYLOAD_SIZE, F> {
    fn default() -> Self {
        Self::new()
    }
}
