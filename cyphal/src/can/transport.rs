use crate::{Box, MessageTransfer, NodeId, Priority, Result, SubjectId, TransferId};
use embedded_can::nb::Can;

pub struct Transport {
    next_transfer_id: TransferId,
}

impl Transport {
    pub fn new<C: Can>(_: C) -> Result<Transport> {
        todo!()
    }

    fn next_transfer_id(&mut self) -> TransferId {
        let transfer_id = self.next_transfer_id;

        self.next_transfer_id += 1;

        transfer_id
    }
}

impl crate::Transport for Transport {
    fn send_message(
        &mut self,
        priority: Priority,
        subject: SubjectId,
        source: Option<NodeId>,
        payload: Box<[u8]>,
    ) -> MessageTransfer {
        self.next_transfer_id += 1;
        MessageTransfer::new(priority, subject, self.next_transfer_id(), source, payload)
    }
}

#[cfg(test)]
mod test {
    use crate::{Box, Priority, Transport};

    extern crate socketcan;
    use socketcan::{CanSocket, Socket};

    #[test]
    #[ignore = "not implemented"]
    fn create_transport() {
        let can = CanSocket::open("vcan0").expect("Could not open can socket");
        let mut transport = super::Transport::new(can).expect("Could not create transport");
        let _ = transport.send_message(Priority::Nominal, 1, Some(2), Box::new([]));
    }
}
