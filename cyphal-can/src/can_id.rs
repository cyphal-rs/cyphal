use crate::{MessageCanId, ServiceCanId};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CanId {
    Message(MessageCanId),

    Service(ServiceCanId),
}
