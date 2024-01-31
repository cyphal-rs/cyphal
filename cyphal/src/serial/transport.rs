use crate::{MessageTransfer, Transfer};

pub struct Transport {}

impl crate::Transport for Transport {
    fn new_message_transfer(&self) -> MessageTransfer {
        todo!()
    }

    fn send(&self, _: &dyn Transfer) -> &dyn Transfer {
        todo!()
    }
}
