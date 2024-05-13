use crate::{UdpError, UdpNodeId, UdpResult, UdpSubjectId, UdpTransferId};
use cyphal::{CyphalError, CyphalResult, NodeId, Priority, SubjectId, TransferId};

/// Represents a UDP payload header used for messages
#[derive(Debug, Copy, Clone)]
pub struct MessageHeader {
    priority: Priority,
    source: Option<UdpNodeId>,
    destination: Option<UdpNodeId>,
    subject: UdpSubjectId,
    transfer: UdpTransferId,
    index: u32,
    end_of_transfer: bool,
    crc16: [u8; 2],
}

impl MessageHeader {
    /// Constructs a new message header
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        priority: Priority,
        source: Option<UdpNodeId>,
        destination: Option<UdpNodeId>,
        subject: UdpSubjectId,
        transfer: UdpTransferId,
        index: u32,
        end_of_transfer: bool,
        crc16: [u8; 2],
    ) -> CyphalResult<Self> {
        if index > 0x7FFFFFFF {
            return Err(CyphalError::OutOfRange);
        }

        Ok(Self {
            priority,
            source,
            destination,
            subject,
            transfer,
            index,
            end_of_transfer,
            crc16,
        })
    }

    /// Returns the priority of the message
    pub fn priority(&self) -> Priority {
        self.priority
    }

    /// Returns the source of the message  'None' is returned if it's anonymous
    pub fn source(&self) -> Option<UdpNodeId> {
        self.source
    }

    /// Returns the destination of the message.  'None' is returned if it's a broadcast
    pub fn destination(&self) -> Option<UdpNodeId> {
        self.destination
    }

    /// Returns the subject of the message
    pub fn subject(&self) -> UdpSubjectId {
        self.subject
    }

    /// Returns the transfer ID of the message
    pub fn transfer(&self) -> UdpTransferId {
        self.transfer
    }

    /// Returns the index of the message
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Returns `true` if it's the end of the message
    pub fn end_of_transfer(&self) -> bool {
        self.end_of_transfer
    }

    /// Returns the crc16 of the message
    pub fn crc16(&self) -> &[u8; 2] {
        &self.crc16
    }

    /// Returns a `&[u8; 24]` representation of the message header
    pub fn as_raw(&self) -> [u8; 24] {
        let source: [u8; 2] = match self.source {
            Some(i) => [(i.value() >> 4) as u8, (i.value() & 0xFF) as u8],
            None => [0xFF, 0xFF],
        };
        let destination: [u8; 2] = match self.destination {
            Some(i) => [(i.value() >> 4) as u8, (i.value() & 0xFF) as u8],
            None => [0xFF, 0xFF],
        };
        let transfer = self.transfer.value();
        let index_mask: u8 = if self.end_of_transfer { 1 } else { 0 };
        [
            1,
            self.priority as u8,
            source[0],
            source[1],
            destination[0],
            destination[1],
            (self.subject.value() >> 3) as u8,
            ((self.subject.value() & 0x7F) << 1) as u8,
            (transfer >> 28) as u8,
            ((transfer >> 24) & 0xFF) as u8,
            ((transfer >> 20) & 0xFF) as u8,
            ((transfer >> 16) & 0xFF) as u8,
            ((transfer >> 12) & 0xFF) as u8,
            ((transfer >> 8) & 0xFF) as u8,
            ((transfer >> 4) & 0xFF) as u8,
            (transfer & 0xFF) as u8,
            (self.index >> 3) as u8,
            ((self.index & 0x7F) << 1) as u8 | index_mask,
            0,
            0,
            self.crc16[0],
            self.crc16[1],
            0,
            0,
        ]
    }
}

impl TryFrom<&[u8; 24]> for MessageHeader {
    type Error = UdpError;

    fn try_from(_value: &[u8; 24]) -> UdpResult<Self> {
        todo!()
    }
}
