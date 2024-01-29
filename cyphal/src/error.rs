#[derive(Debug)]
pub enum CyphalError {
    NotDefined,
    OutOfRange,
}

pub type Result<T> = core::result::Result<T, CyphalError>;
