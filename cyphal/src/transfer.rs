use crate::{Priority, TransferId};

pub enum TransferKind {
    Message,
    Service,
}

pub trait Transfer {
    fn priority(&self) -> Priority;
    fn id(&self) -> TransferId;
    fn kind(&self) -> TransferKind;
    fn payload(&self) -> &[u8];
    fn crc(&self) -> Option<u32>;
}
