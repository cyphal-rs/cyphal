use core::fmt::{Display, Formatter, Result as FmtResult};

/// Represents a UDP Error
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UdpError {
    /// Invalid Address
    InvalidAddress,

    /// Connection
    Connection,

    /// The transmission data is above the maximum allowed size
    MaxPayloadExceded,
}

impl Display for UdpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::InvalidAddress => write!(f, "The address is invalid"),
            Self::Connection => write!(f, "A connection failure has occured"),
            Self::MaxPayloadExceded => {
                write!(f, "The transmission data is above the maximum allowed size")
            }
        }
    }
}

/// The result of a Cyphal UDP operation.  On failure, a `UdpError` will be included.
pub type UdpResult<T> = Result<T, UdpError>;
