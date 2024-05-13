use crate::{UdpNodeId, UdpServiceId, UdpTransferId};
use cyphal::{CyphalError, CyphalResult, NodeId, Priority, ServiceId, TransferId};

/// Represents a payload header used for services
#[derive(Debug, Copy, Clone)]
pub struct ServiceHeader {
    priority: Priority,
    source: UdpNodeId,
    destination: UdpNodeId,
    is_request: bool,
    service: UdpServiceId,
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
        service: UdpServiceId,
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
            service,
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
        self.service
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
        let index_mask: u8 = if self.end_of_transfer { 1 } else { 0 };
        // If this is a service response transfer, this value equals the service-ID.
        // If this is a service request transfer, this value equals 16384 + service-ID.
        let service_id = if self.is_request {
            self.service.value() + 16384
        } else {
            self.service.value()
        };
        let specifier = [
            (service_id >> 7) as u8,
            (((service_id & 0xFF) << 1) | 0x01) as u8,
        ];
        let transfer = self.transfer.value();

        [
            1,
            self.priority as u8,
            (self.source.value() >> 8) as u8,
            (self.source.value() & 0xFF) as u8,
            (self.destination.value() >> 8) as u8,
            self.destination.value() as u8,
            specifier[0],
            specifier[1],
            (transfer >> 56) as u8,
            ((transfer >> 48) & 0xFF) as u8,
            ((transfer >> 40) & 0xFF) as u8,
            ((transfer >> 32) & 0xFF) as u8,
            ((transfer >> 24) & 0xFF) as u8,
            ((transfer >> 16) & 0xFF) as u8,
            ((transfer >> 8) & 0xFF) as u8,
            (transfer & 0xFF) as u8,
            (self.index >> 7) as u8,
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

impl TryFrom<&[u8; 24]> for ServiceHeader {
    type Error = CyphalError;

    fn try_from(value: &[u8; 24]) -> CyphalResult<Self> {
        // ensure version = 1, not a service,
        if value[0] != 1 || (value[7] & 1) != 1 {
            return Err(CyphalError::OutOfRange);
        }

        let priority = Priority::try_from(value[1] << 5)?;
        let service_id = ((value[6] as u16) << 3) & ((value[7] as u16) >> 1);
        let service: UdpServiceId = (service_id & 0x3FFF).try_into()?;
        let is_request = service_id & 0x4000 > 0;
        let transfer: UdpTransferId = (((value[8] as u64) << 56)
            & ((value[9] as u64) << 48)
            & ((value[10] as u64) << 40)
            & ((value[11] as u64) << 32)
            & ((value[12] as u64) << 24)
            & ((value[13] as u64) << 16)
            & ((value[14] as u64) << 8)
            & (value[15] as u64))
            .try_into()?;
        let index = ((value[16] as u32) << 23)
            & ((value[17] as u32) << 15)
            & ((value[18] as u32) << 7)
            & ((value[19] as u32) >> 1);
        let end_of_transfer = value[19] & 0x01 == 1;
        let crc16 = [value[20], value[21]];

        Ok(Self {
            priority,
            source: (((value[2] as u16) << 8) & (value[3] as u16)).try_into()?,
            destination: (((value[4] as u16) << 8) & (value[5] as u16)).try_into()?,
            service,
            is_request,
            transfer,
            index,
            end_of_transfer,
            crc16,
        })
    }
}
