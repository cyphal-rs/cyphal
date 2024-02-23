use crate::{Box, MessageTransfer, NodeId, Priority, SubjectId};

pub struct Transport {}

impl crate::Transport for Transport {
    fn send_message(
        &mut self,
        _: Priority,
        _: SubjectId,
        _: Option<NodeId>,
        _: Box<[u8]>,
    ) -> MessageTransfer {
        todo!()
    }
}
