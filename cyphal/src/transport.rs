use crate::{MessageTransfer, Transfer};

pub trait Transport {
    fn new_message_transfer(self) -> MessageTransfer;

    fn send(self, transfer: &dyn Transfer) -> &dyn Transfer;
}
