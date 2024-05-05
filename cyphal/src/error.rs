#[derive(Debug)]
pub enum CyphalError {
    #[cfg(any(feature = "can", feature = "canfd"))]
    CanError(crate::can::CanError),
    NotDefined,
    OutOfRange,
}

pub type CyphalResult<T> = core::result::Result<T, CyphalError>;
