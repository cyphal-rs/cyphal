use crate::{CanError, CanResult};
use cyphal::{NodeId, Priority, SubjectId};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct MessageCanId {
    priority: Priority,
    anonymous: bool,
    subject_id: SubjectId,
    source: NodeId,
}

impl MessageCanId {
    pub fn new(
        priority: Priority,
        subject_id: SubjectId,
        source: Option<NodeId>,
    ) -> CanResult<Self> {
        if subject_id > 8191 {
            return Err(CanError::InvalidId);
        }
        if source.is_some_and(|s| s > 127) {
            return Err(CanError::InvalidId);
        }

        match source {
            Some(s) => Ok(MessageCanId {
                priority,
                anonymous: false,
                subject_id,
                source: s,
            }),
            None => Ok(MessageCanId {
                priority,
                anonymous: true,
                subject_id,
                //FIXME: generate a pseudorandom pseudo-ID value for source
                source: 0,
            }),
        }
    }

    pub fn anonymous(&self) -> bool {
        self.anonymous
    }

    pub fn subject_id(&self) -> SubjectId {
        self.subject_id
    }

    pub fn priority(&self) -> Priority {
        self.priority
    }

    pub fn is_service(&self) -> bool {
        false
    }

    pub fn source(&self) -> NodeId {
        self.source
    }

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
        result |= (self.subject_id as u32) << 8;

        // set source node id bits 0 to 7
        result | (self.source as u32)
    }
}

impl TryFrom<u32> for MessageCanId {
    type Error = CanError;

    fn try_from(value: u32) -> CanResult<Self> {
        // make sure it's a message id
        if (value & 0x0200_0000) > 0 {
            return Err(CanError::InvalidId);
        }

        // make sure reserved bit seven is set to zero
        if (value & 0x80) > 0 {
            return Err(CanError::InvalidId);
        }

        let priority = match Priority::try_from((value >> 26) as u8) {
            Ok(p) => p,
            Err(_) => return Err(CanError::InvalidId),
        };

        let anonymous = (value & 0x0100_0000) > 0;
        let source = (value & 0x7F) as NodeId;
        let subject_id = ((value >> 8) & 0x1FFF) as SubjectId;

        Ok(MessageCanId {
            priority,
            anonymous,
            subject_id,
            source,
        })
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::MessageCanId;
    use cyphal::{NodeId, Priority, SubjectId};

    #[test]
    #[allow(non_snake_case)]
    fn test_0x107D552A() {
        // Arrange
        let priority = Priority::Nominal;
        let source: NodeId = 42;
        let subject_id: SubjectId = 7509;

        // Act
        let target = MessageCanId::new(priority, subject_id, Some(source)).unwrap();

        // Assert
        assert!(!target.anonymous());
        assert_eq!(target.subject_id(), subject_id);
        assert_eq!(target.priority(), priority);
        assert!(!target.is_service());
        assert_eq!(target.source(), source);
        assert_eq!(target.as_raw(), 0x107D552A);
    }

    /*
    #[test]
    #[allow(non_snake_case)]
    fn test_0x11133775() {
        // uavcan.primitive.String.1.0 under subject-ID 4919 (133716) published by an anonymous node, the
        // string is “Hello world!” (ASCII); one byte of zero padding can be seen between the payload and the tail
        // byte  0001 - 0001 000 1  0011 0011 0111 - 0111 0101
        // 01 0  -- 001 0001 0011 0011 0111 -- 0111 0101

        // Arrange FIX
        let priority = Priority::Nominal;
        let source: u8 = 117;
        let subject_id: u16 = 4919;

        // Act
        let target = MessageCanId::new_anonymous(priority, source, subject_id).unwrap();

        // Assert
        assert!(!target.anonymous());
        assert_eq!(target.subject_id(), subject_id);
        assert_eq!(target.priority(), priority);
        assert!(!target.is_service());
        assert_eq!(target.source(), source);
        assert_eq!(target.as_raw(), 0x107D552A);
    }
    */
}
