use core::fmt::{Display, Formatter, Result as FmtResult};

/// Cyphal Error
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CyphalError {
    /// An error caused by the underlying transport
    Transport,

    /// The value is out of the permissible range
    OutOfRange,
}

impl Display for CyphalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Transport => write!(f, "An error has occured in the underlying transport"),
            Self::OutOfRange => write!(f, "The value is outside the permissable range"),
        }
    }
}

/// The result of a Cyphal operation.  On failure, a `CyphalError` will be included.
pub type CyphalResult<T> = Result<T, CyphalError>;
