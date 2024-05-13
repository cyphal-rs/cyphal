use crate::{UdpError, UdpNodeId, UdpResult, UdpServiceId, UdpTransferId};
use cyphal::{CyphalError, CyphalResult, Priority};

/// Represents a payload header used for services
#[derive(Debug, Copy, Clone)]
pub struct ServiceHeader {
    priority: Priority,
    source: UdpNodeId,
    destination: UdpNodeId,
    is_request: bool,
    service_id: UdpServiceId,
    transfer: UdpTransferId,
    index: u32,
    end_of_transfer: bool,
    crc16: [u8; 2],
}

impl ServiceHeader {
    /// Constructs a new service header
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        priority: Priority,
        source: UdpNodeId,
        destination: UdpNodeId,
        is_request: bool,
        service_id: UdpServiceId,
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
            is_request,
            service_id,
            transfer,
            index,
            end_of_transfer,
            crc16,
        })
    }

    /// Returns the priority of the service
    pub fn priority(&self) -> Priority {
        self.priority
    }

    /// Returns the source of the service
    pub fn source(&self) -> UdpNodeId {
        self.source
    }

    /// Returns the destination of the service
    pub fn destination(&self) -> UdpNodeId {
        self.destination
    }

    /// Returns the service ID
    pub fn service_id(&self) -> UdpServiceId {
        self.service_id
    }

    /// Returns `true` if it's a request of the service
    pub fn is_request(&self) -> bool {
        self.is_request
    }

    /// Returns the transfer ID of the service
    pub fn transfer(&self) -> UdpTransferId {
        self.transfer
    }

    /// Returns the index of the service
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Returns `true` if it's the end of the service
    pub fn end_of_transfer(&self) -> bool {
        self.end_of_transfer
    }

    /// Returns the crc16 of the service
    pub fn crc16(&self) -> &[u8; 2] {
        &self.crc16
    }

    /// Returns a `u32` representation of the service header
    pub fn as_raw(&self) -> [u8; 24] {
        todo!()
    }
}

impl TryFrom<&[u8; 24]> for ServiceHeader {
    type Error = UdpError;

    fn try_from(_value: &[u8; 24]) -> UdpResult<Self> {
        todo!()
    }
}
