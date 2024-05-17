extern crate alloc;

use crate::{CanId, CanTransferId, Frame, MessageCanId, ServiceCanId};
use alloc::collections::{BTreeMap, VecDeque};
use cyphal::Priority;

pub struct InboundQueue<const PAYLOAD_SIZE: usize, F: Frame<PAYLOAD_SIZE>> {
    pending_messages: BTreeMap<(MessageCanId, CanTransferId), VecDeque<F>>,
    complete_messages: BTreeMap<(MessageCanId, CanTransferId), VecDeque<F>>,
    pending_requests: BTreeMap<(ServiceCanId, CanTransferId), VecDeque<F>>,
    complete_requests: BTreeMap<(ServiceCanId, CanTransferId), VecDeque<F>>,
    pending_responses: BTreeMap<CanTransferId, VecDeque<F>>,
    complete_responses: BTreeMap<CanTransferId, VecDeque<F>>,
}

impl<const PAYLOAD_SIZE: usize, F: Frame<PAYLOAD_SIZE>> InboundQueue<PAYLOAD_SIZE, F> {
    pub fn new() -> Self {
        Self {
            pending_messages: BTreeMap::new(),
            complete_messages: BTreeMap::new(),
            pending_requests: BTreeMap::new(),
            complete_requests: BTreeMap::new(),
            pending_responses: BTreeMap::new(),
            complete_responses: BTreeMap::new(),
        }
    }

    pub fn push(&mut self, frame: F) {
        match frame.id() {
            CanId::Message(id) => {
                // Check to see if it's a single frame transfer
                if frame.is_single_trame_transfer() {
                    let transfer = frame.transfer();
                    let queue = VecDeque::from([frame]);
                    self.complete_messages.insert((id, transfer), queue);

                    return;
                }

                let transfer = frame.transfer();

                // if it's the last frame, remove the frames from pending and add to complete
                if frame.is_end_of_transfer() {
                    if let Some(mut queue) = self.pending_messages.remove(&(id, transfer)) {
                        queue.push_back(frame);
                        self.complete_messages.insert((id, transfer), queue);
                    }
                } else {
                    match self.pending_messages.get_mut(&(id, transfer)) {
                        Some(queue) => queue.push_back(frame),
                        None => {
                            // only add transmission if it's the start of one
                            if frame.is_start_of_transfer() {
                                let queue = VecDeque::from([frame]);
                                self.pending_messages.insert((id, transfer), queue);
                            }
                        }
                    }
                }
            }
            CanId::Service(id) => {
                if id.is_request() {
                    // Check to see if it's a single frame transfer
                    if frame.is_single_trame_transfer() {
                        let transfer = frame.transfer();
                        let queue = VecDeque::from([frame]);
                        self.complete_requests.insert((id, transfer), queue);

                        return;
                    }

                    let transfer = frame.transfer();

                    // if it's the last frame, remove the frames from pending and add to complete
                    if frame.is_end_of_transfer() {
                        if let Some(mut queue) = self.pending_requests.remove(&(id, transfer)) {
                            queue.push_back(frame);
                            self.complete_requests.insert((id, transfer), queue);
                        }
                    } else {
                        match self.pending_requests.get_mut(&(id, transfer)) {
                            Some(queue) => queue.push_back(frame),
                            None => {
                                // only add transmission if it's the start of one
                                if frame.is_start_of_transfer() {
                                    let queue = VecDeque::from([frame]);
                                    self.pending_requests.insert((id, transfer), queue);
                                }
                            }
                        }
                    }
                } else {
                    // Check to see if it's a single frame transfer
                    if frame.is_single_trame_transfer() {
                        let transfer = frame.transfer();
                        let queue = VecDeque::from([frame]);
                        self.complete_responses.insert(transfer, queue);

                        return;
                    }

                    let transfer = frame.transfer();

                    // if it's the last frame, remove the frames from pending and add to complete
                    if frame.is_end_of_transfer() {
                        if let Some(mut queue) = self.pending_responses.remove(&transfer) {
                            queue.push_back(frame);
                            self.complete_responses.insert(transfer, queue);
                        }
                    } else {
                        match self.pending_responses.get_mut(&transfer) {
                            Some(queue) => queue.push_back(frame),
                            None => {
                                // only add transmission if it's the start of one
                                if frame.is_start_of_transfer() {
                                    let queue = VecDeque::from([frame]);
                                    self.pending_responses.insert(transfer, queue);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_message_frames(&mut self) -> Option<VecDeque<(MessageCanId, VecDeque<F>)>> {
        let keys = self.complete_messages.keys();
        if keys.len() == 0 {
            return None;
        }

        let mut p0: VecDeque<(MessageCanId, VecDeque<F>)> = VecDeque::new();
        let mut p1: VecDeque<(MessageCanId, VecDeque<F>)> = VecDeque::new();
        let mut p2: VecDeque<(MessageCanId, VecDeque<F>)> = VecDeque::new();
        let mut p3: VecDeque<(MessageCanId, VecDeque<F>)> = VecDeque::new();
        let mut p4: VecDeque<(MessageCanId, VecDeque<F>)> = VecDeque::new();
        let mut p5: VecDeque<(MessageCanId, VecDeque<F>)> = VecDeque::new();
        let mut p6: VecDeque<(MessageCanId, VecDeque<F>)> = VecDeque::new();
        let mut p7: VecDeque<(MessageCanId, VecDeque<F>)> = VecDeque::new();
        let mut p8: VecDeque<(MessageCanId, VecDeque<F>)> = VecDeque::new();

        while let Some(kvp) = self.complete_messages.pop_first() {
            match kvp.0 .0.priority() {
                Priority::Exceptional => p0.push_back((kvp.0 .0, kvp.1)),
                Priority::Immediate => p1.push_back((kvp.0 .0, kvp.1)),
                Priority::Fast => p2.push_back((kvp.0 .0, kvp.1)),
                Priority::High => p3.push_back((kvp.0 .0, kvp.1)),
                Priority::Nominal => p4.push_back((kvp.0 .0, kvp.1)),
                Priority::Low => p5.push_back((kvp.0 .0, kvp.1)),
                Priority::Slow => p6.push_back((kvp.0 .0, kvp.1)),
                Priority::Optional => p7.push_back((kvp.0 .0, kvp.1)),
            }
        }

        let mut result: VecDeque<(MessageCanId, VecDeque<F>)> = VecDeque::new();
        result.append(&mut p0);
        result.append(&mut p1);
        result.append(&mut p2);
        result.append(&mut p3);
        result.append(&mut p4);
        result.append(&mut p5);
        result.append(&mut p6);
        result.append(&mut p7);
        result.append(&mut p8);

        Some(result)
    }

    pub fn get_request_frames(&mut self) -> Option<VecDeque<(ServiceCanId, VecDeque<F>)>> {
        let keys = self.complete_requests.keys();
        if keys.len() == 0 {
            return None;
        }

        let mut p0: VecDeque<(ServiceCanId, VecDeque<F>)> = VecDeque::new();
        let mut p1: VecDeque<(ServiceCanId, VecDeque<F>)> = VecDeque::new();
        let mut p2: VecDeque<(ServiceCanId, VecDeque<F>)> = VecDeque::new();
        let mut p3: VecDeque<(ServiceCanId, VecDeque<F>)> = VecDeque::new();
        let mut p4: VecDeque<(ServiceCanId, VecDeque<F>)> = VecDeque::new();
        let mut p5: VecDeque<(ServiceCanId, VecDeque<F>)> = VecDeque::new();
        let mut p6: VecDeque<(ServiceCanId, VecDeque<F>)> = VecDeque::new();
        let mut p7: VecDeque<(ServiceCanId, VecDeque<F>)> = VecDeque::new();
        let mut p8: VecDeque<(ServiceCanId, VecDeque<F>)> = VecDeque::new();

        while let Some(kvp) = self.complete_requests.pop_first() {
            match kvp.0 .0.priority() {
                Priority::Exceptional => p0.push_back((kvp.0 .0, kvp.1)),
                Priority::Immediate => p1.push_back((kvp.0 .0, kvp.1)),
                Priority::Fast => p2.push_back((kvp.0 .0, kvp.1)),
                Priority::High => p3.push_back((kvp.0 .0, kvp.1)),
                Priority::Nominal => p4.push_back((kvp.0 .0, kvp.1)),
                Priority::Low => p5.push_back((kvp.0 .0, kvp.1)),
                Priority::Slow => p6.push_back((kvp.0 .0, kvp.1)),
                Priority::Optional => p7.push_back((kvp.0 .0, kvp.1)),
            }
        }

        let mut result: VecDeque<(ServiceCanId, VecDeque<F>)> = VecDeque::new();
        result.append(&mut p0);
        result.append(&mut p1);
        result.append(&mut p2);
        result.append(&mut p3);
        result.append(&mut p4);
        result.append(&mut p5);
        result.append(&mut p6);
        result.append(&mut p7);
        result.append(&mut p8);

        Some(result)
    }

    pub fn get_response_frames(&mut self, transfer: CanTransferId) -> Option<VecDeque<F>> {
        self.complete_responses.remove(&transfer)
    }
}

impl<const PAYLOAD_SIZE: usize, F: Frame<PAYLOAD_SIZE>> Default for InboundQueue<PAYLOAD_SIZE, F> {
    fn default() -> Self {
        Self::new()
    }
}
