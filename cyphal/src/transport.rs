use crate::{Box, MessageTransfer, NodeId, Priority};

pub trait Transport {
    fn send_message(
        &mut self,
        priority: Priority,
        subject: u64,
        source: Option<NodeId>,
        payload: Box<[u8]>,
    ) -> MessageTransfer;
}

#[cfg(test)]
mod test {
    extern crate alloc;
    use alloc::boxed::Box;

    use crate::{MessageTransfer, NodeId, Priority, TransferId, Transport};

    struct FakeTransport {
        transfer_id: TransferId,
    }

    impl Transport for FakeTransport {
        fn send_message(
            &mut self,
            priority: Priority,
            subject: u64,
            source: Option<NodeId>,
            payload: Box<[u8]>,
        ) -> crate::MessageTransfer {
            MessageTransfer::new(priority, subject, self.transfer_id, source, payload)
        }
    }

    #[test]
    fn send() {
        let mut transport = FakeTransport { transfer_id: 1 };
        let _ = transport.send_message(Priority::Nominal, 1, None, Box::new([]));
    }
}
