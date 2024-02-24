use crate::{Box, MessageTransfer, NodeId, Priority, SubjectId};

pub struct SerialTransport {}

impl crate::Transport for SerialTransport {
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
