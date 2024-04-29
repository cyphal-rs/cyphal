use crate::Priority;

pub trait CanId {
    fn as_raw(&self) -> u32;
    fn priority(&self) -> Priority;
    fn is_service(&self) -> bool;
    fn source(&self) -> u8;
}
