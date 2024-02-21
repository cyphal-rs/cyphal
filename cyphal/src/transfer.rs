use crate::{Priority, TransferId};

pub enum TransferKind {
    Message,
    Service,
}

pub trait Transfer {
    fn priority(&self) -> Priority;
    fn id(&self) -> TransferId;
    fn kind(&self) -> TransferKind;
}
