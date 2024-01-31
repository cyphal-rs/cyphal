use crate::{NodeId, Priority, TransferId};

pub enum TransferKind {
    Message,
    Service,
}

pub trait Transfer {
    fn priority(&self) -> Priority;
    fn id(&self) -> TransferId;
    fn source(&self) -> NodeId;
    fn kind(&self) -> TransferKind;
}
