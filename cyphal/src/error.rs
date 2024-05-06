#[derive(Debug)]
pub enum CyphalError {
    Transport,
    NotDefined,
    OutOfRange,
}

pub type CyphalResult<T> = core::result::Result<T, CyphalError>;
