use crate::{CanError, CanNodeId, CanResult, CanServiceId};
use cyphal::{NodeId, Priority, ServiceId};

/// Represents an extended CAN ID used for services
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct ServiceCanId {
    priority: Priority,
    is_request: bool,
    service: CanServiceId,
    destination: CanNodeId,
    source: CanNodeId,
}

impl ServiceCanId {
    /// Constructs a new service CAN ID
    pub fn new(
        priority: Priority,
        is_request: bool,
        service: CanServiceId,
        destination: CanNodeId,
        source: CanNodeId,
    ) -> CanResult<Self> {
        Ok(ServiceCanId {
            priority,
            is_request,
            service,
            destination,
            source,
        })
    }

    /// Returns the priority of the service call
    pub fn priority(&self) -> Priority {
        self.priority
    }

    /// Indicates if its a service request if `true`, or a response if `false`
    pub fn is_request(&self) -> bool {
        self.is_request
    }

    /// Returns the Service ID.
    pub fn service(&self) -> CanServiceId {
        self.service
    }

    /// Returns the Node ID of the destination.
    pub fn destination(&self) -> CanNodeId {
        self.destination
    }

    /// Returns the Node ID from where the service call originates.
    pub fn source(&self) -> CanNodeId {
        self.source
    }

    /// Returns a `u32` representation of the service CAN ID
    pub fn as_raw(&self) -> u32 {
        // set priority bits 26 to 28
        let mut result: u32 = (u8::from(self.priority) as u32) << 26;

        // set is service bit 25
        result |= 0x0200_0000;

        if self.is_request {
            result |= 0x0100_0000;
        }

        // set service id bits 14 to 22
        result |= (self.service.value() as u32) << 14;

        // set subject id bits 8 to 20
        result |= (self.destination.value() as u32) << 7;

        // set source node id bits 0 to 7
        result | (self.source.value() as u32)
    }
}

impl TryFrom<u32> for ServiceCanId {
    type Error = CanError;

    fn try_from(value: u32) -> CanResult<Self> {
        // make sure it's a service id
        if (value & 0x0200_0000) == 0 {
            return Err(CanError::InvalidId);
        }

        // make sure reserved bit 23 is set to zero
        if (value & 0x0080_0000) > 0 {
            return Err(CanError::InvalidId);
        }

        let priority = match Priority::try_from((value >> 26) as u8) {
            Ok(p) => p,
            Err(_) => return Err(CanError::InvalidId),
        };
        let is_request = (value & 0x0100_0000) > 0;
        let service_id = (((value >> 14) & 0x01FF) as u16).try_into().unwrap();
        let destination = (((value >> 7) & 0x7F) as u8).try_into().unwrap();
        let source = ((value & 0x7F) as u8).try_into().unwrap();

        Ok(ServiceCanId {
            priority,
            is_request,
            service: service_id,
            destination,
            source,
        })
    }
}
