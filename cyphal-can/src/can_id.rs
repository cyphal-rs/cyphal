use crate::{CanResult, MessageCanId, ServiceCanId};
use cyphal::Priority;

/// Represets an Extended CAN ID
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CanId {
    /// Extended CAN ID used for messages
    Message(MessageCanId),

    /// Extended CAN ID used for services
    Service(ServiceCanId),
}

impl CanId {
    /// Constructs a new `CanID`
    pub fn new(id: u32) -> CanResult<CanId> {
        // check bit 25 to see what type of Id this is
        if (id & 0x0200_0000) == 0 {
            match MessageCanId::try_from(id) {
                Ok(i) => Ok(CanId::Message(i)),
                Err(e) => Err(e),
            }
        } else {
            match ServiceCanId::try_from(id) {
                Ok(i) => Ok(CanId::Service(i)),
                Err(e) => Err(e),
            }
        }
    }

    /// Returns a `u32` representation of the CAN ID
    pub fn as_raw(&self) -> u32 {
        match *self {
            CanId::Message(m) => m.as_raw(),
            CanId::Service(s) => s.as_raw(),
        }
    }

    /// Returns the `Priority` of the CAN ID
    pub fn priority(&self) -> Priority {
        match *self {
            CanId::Message(m) => m.priority(),
            CanId::Service(s) => s.priority(),
        }
    }
}

impl From<MessageCanId> for CanId {
    #[inline]
    fn from(id: MessageCanId) -> Self {
        CanId::Message(id)
    }
}

impl From<ServiceCanId> for CanId {
    #[inline]
    fn from(id: ServiceCanId) -> Self {
        CanId::Service(id)
    }
}

#[cfg(test)]
mod test {
    use crate::{CanId, MessageCanId, ServiceCanId};
    use cyphal::Priority;

    #[test]
    fn test_new_message_id() {
        let id = MessageCanId::new(Priority::Nominal, 1, None)
            .unwrap()
            .as_raw();

        let target = CanId::new(id).unwrap();
        assert_eq!(target.as_raw(), id);
    }

    #[test]
    fn test_priority() {
        let id1: CanId = MessageCanId::new(Priority::Nominal, 1, None)
            .unwrap()
            .into();
        let id2: CanId = ServiceCanId::new(Priority::High, true, 1, 1, 1)
            .unwrap()
            .into();

        assert_eq!(id1.priority(), Priority::Nominal);
        assert_eq!(id2.priority(), Priority::High);
    }

    #[test]
    fn test_new_invalid_message_id() {
        let id = 0x0080_0000_u32;

        assert!(CanId::new(id).is_err());
    }

    #[test]
    fn test_new_service_id() {
        let id = ServiceCanId::new(Priority::Nominal, true, 1, 1, 1)
            .unwrap()
            .as_raw();

        assert_eq!(CanId::new(id).unwrap().as_raw(), id);
    }

    #[test]
    fn test_new_invalid_service_id() {
        let id = 0x0280_0000_u32;

        assert!(CanId::new(id).is_err());
    }

    #[test]
    fn test_from_message_id() {
        let id = MessageCanId::new(Priority::Nominal, 1, None).unwrap();

        assert_eq!(CanId::from(id), CanId::Message(id));
    }

    #[test]
    fn test_from_service_id() {
        let id = ServiceCanId::new(Priority::Nominal, true, 1, 1, 1).unwrap();

        assert_eq!(CanId::from(id), CanId::Service(id));
    }
}
