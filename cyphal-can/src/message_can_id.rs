use crate::CanResult;
use cyphal::{NodeId, Priority, SubjectId};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct MessageCanId {
    anonymous: bool,
    priority: Priority,
    source: NodeId,
    subject_id: SubjectId,
}

impl MessageCanId {
    pub fn new(
        priority: Priority,
        source: Option<NodeId>,
        subject_id: SubjectId,
    ) -> CanResult<Self> {
        match source {
            Some(s) => Ok(MessageCanId {
                anonymous: false,
                priority,
                source: s,
                subject_id,
            }),
            None => Ok(MessageCanId {
                anonymous: true,
                priority,
                //TODO: generate a pseudorandom pseudo-ID value for source
                source: 0,
                subject_id,
            }),
        }
    }

    pub fn from_raw(_: u32) -> CanResult<Self> {
        todo!()
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
        // set priority bits 26 to 29
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
        let target = MessageCanId::new(priority, Some(source), subject_id).unwrap();

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
