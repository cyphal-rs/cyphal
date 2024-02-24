use crate::{Box, MessageTransfer, NodeId, Priority, SubjectId};

pub struct UdpTransport {}

impl crate::Transport for UdpTransport {
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
