use thiserror::Error;

/// Represents a DSDL error
#[derive(Error, Debug)]
pub enum DsdlError {
    /// The DSDL statement at line is not valid error
    #[error("The DSDL statement at line `{0}` is not valid: {1}")]
    InvalidStatement(u32, String),

    /// IO error
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),

    /// Not implemented error
    #[error("Not implemented")]
    NotImplemented,

    /// Out of range error
    #[error("Out of range: `{0}`")]
    OutOfRange(String),

    /// Parse error
    #[error("Parse error: `{0}`")]
    Parse(String),

    /// Parse Int
    #[error("ParseIntError error: `{0}`")]
    ParseIntError(#[from] std::num::ParseIntError),
}

/// Represents a DSDL result
pub type DsdlResult<T> = Result<T, DsdlError>;
