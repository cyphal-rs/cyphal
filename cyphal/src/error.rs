#[derive(Debug)]
pub enum CyphalError {
    #[cfg(feature = "can")]
    CanError(crate::can::CanError),
    NotDefined,
    OutOfRange,
}

pub type CyphalResult<T> = core::result::Result<T, CyphalError>;
