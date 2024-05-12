use crate::{CanNodeId, CanResult, CanSubjectId};
use cyphal::{CyphalError, CyphalResult, NodeId, Priority, SubjectId};

/// Represents an extended CAN ID used for messages
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct MessageCanId {
    priority: Priority,
    anonymous: bool,
    subject: CanSubjectId,
    source: CanNodeId,
}

impl MessageCanId {
    /// Constructs a new message CAN ID
    pub fn new(
        priority: Priority,
        subject: CanSubjectId,
        source: Option<CanNodeId>,
    ) -> CanResult<Self> {
        match source {
            Some(s) => Ok(MessageCanId {
                priority,
                anonymous: false,
                subject,
                source: s,
            }),
            None => Ok(MessageCanId {
                priority,
                anonymous: true,
                subject,
                //FIXME: generate a pseudorandom pseudo-ID value for source
                source: 0.try_into().unwrap(),
            }),
        }
    }

    /// Returns the priority of the message
    pub fn priority(&self) -> Priority {
        self.priority
    }

    /// Indicates if the message is anonymous
    pub fn is_anonymous(&self) -> bool {
        self.anonymous
    }

    /// Returns the Subject ID of the message
    pub fn subject(&self) -> CanSubjectId {
        self.subject
    }

    /// Returns the Node ID from where the message originates.
    ///
    /// Note: Anonymous messages contain a generated pseudorandom pseudo-ID value
    pub fn source(&self) -> CanNodeId {
        self.source
    }

    /// Returns a `u32` representation of the message CAN ID
    pub fn as_raw(&self) -> u32 {
        // set priority bits 26 to 28
        let mut result: u32 = (u8::from(self.priority) as u32) << 26;

        // set anonymous bit 24
        if self.anonymous {
            result |= 1 << 24;
        }

        // set reserved bits 21 and 22
        result |= 0x3 << 21;

        // set subject id bits 8 to 20
        result |= (self.subject.value() as u32) << 8;

        // set source node id bits 0 to 7
        result | (self.source.value() as u32)
    }
}

impl TryFrom<u32> for MessageCanId {
    type Error = CyphalError;

    fn try_from(value: u32) -> CyphalResult<Self> {
        // make sure it's a message id
        if (value & 0x0200_0000) != 0 {
            return Err(CyphalError::OutOfRange);
        }

        // make sure reserved bit 7 is set to zero
        if (value & 0x80) != 0 {
            return Err(CyphalError::OutOfRange);
        }
        // make sure reserved bit 21 is set to one
        if (value & 0x0020_0000) == 0 {
            return Err(CyphalError::OutOfRange);
        }
        // make sure reserved bit 22 is set to one
        if (value & 0x0040_0000) == 0 {
            return Err(CyphalError::OutOfRange);
        }
        // make sure reserved bit 23 is set to zero
        if (value & 0x0080_0000) != 0 {
            return Err(CyphalError::OutOfRange);
        }

        let priority = match Priority::try_from((value >> 26) as u8) {
            Ok(p) => p,
            Err(_) => return Err(CyphalError::OutOfRange),
        };

        let anonymous = (value & 0x0100_0000) > 0;
        let source = ((value & 0x7F) as u8).try_into().unwrap();
        let subject = (((value >> 8) & 0x1FFF) as u16).try_into().unwrap();

        Ok(MessageCanId {
            priority,
            anonymous,
            subject,
            source,
        })
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::{CanNodeId, CanSubjectId, MessageCanId};
    use cyphal::{Priority, SubjectId};

    #[test]
    #[allow(non_snake_case)]
    fn test_0x107D552A() {
        // Arrange
        let priority = Priority::Nominal;
        let source: CanNodeId = 42.try_into().unwrap();
        let subject: CanSubjectId = 7509.try_into().unwrap();

        // Act
        let target = MessageCanId::new(priority, subject, Some(source)).unwrap();

        // Assert
        assert_eq!(target.priority(), priority);
        assert!(!target.is_anonymous());
        assert_eq!(target.subject(), subject);
        assert_eq!(target.source(), source);
        assert_eq!(target.as_raw(), 0x107D552A);
    }

    #[test]
    #[ignore = "need to generate a pseudorandom pseudo-ID value for source"]
    #[allow(non_snake_case)]
    fn test_0x11133775() {
        // uavcan.primitive.String.1.0 under subject-ID 4919 (133716) published by an anonymous node, the
        // string is “Hello world!” (ASCII); one byte of zero padding can be seen between the payload and the tail
        // byte  0001 - 0001 000 1  0011 0011 0111 - 0111 0101
        // 01 0  -- 001 0001 0011 0011 0111 -- 0111 0101

        // Arrange FIX
        let priority = Priority::Nominal;
        let subject: u16 = 4919;

        // Act
        let target = MessageCanId::new(priority, subject.try_into().unwrap(), None).unwrap();

        // Assert
        assert_eq!(target.priority(), priority);
        assert!(target.is_anonymous());
        assert_eq!(target.subject().value(), subject);
        //FIXME: generate a pseudorandom pseudo-ID value for source
        //assert_eq!(target.as_raw() & 0xFFFFFF00, 0x11133775 & 0xFFFFFF00);
    }
}
