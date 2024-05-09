use crate::{NodeId, Priority, SubjectId};

/// Trait representing a message
pub trait Message<const N: usize>: Sized {
    /// Type representing the payload of the message
    type Payload: Sized;

    /// The Priority of the message
    fn priority(&self) -> Priority;

    /// Returns the Node ID of the sender.  Anonymous messages can be sent using `None`
    fn source(&self) -> Option<NodeId>;

    /// Returns the Subject ID of the message
    fn subject(&self) -> SubjectId;

    /// Return the message payload
    fn payload(&self) -> &[u8];
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::{
        test::{TestMessage, TEST_MESSAGE_SIZE},
        Message as _, NodeId, Priority, SubjectId,
    };
    use std::vec::Vec;

    #[test]
    fn test_new() {
        let priority = Priority::Optional;
        let subject_id: SubjectId = 12;
        let source: Option<NodeId> = Some(56);
        let data: Vec<u8> = (0..(TEST_MESSAGE_SIZE as u8)).collect();
        let data: [u8; TEST_MESSAGE_SIZE] = data.try_into().unwrap();

        let message = TestMessage::new(priority, subject_id, source, data).unwrap();

        assert_eq!(message.priority(), priority);
        assert_eq!(message.subject(), subject_id);
        assert_eq!(message.source(), source);

        let payload = message.payload();
        assert_eq!(payload.len(), TEST_MESSAGE_SIZE);

        for (i, v) in data.iter().enumerate() {
            assert_eq!(payload[i], *v);
        }
    }
}
