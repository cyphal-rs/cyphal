/// Represents a UDP Error
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UdpError {
    /// Invalid Address
    InvalidAddress,

    /// Connection
    Connection,
}

/// The result of a Cyphal UDP operation.  On failure, a `UdpError` will be included.
pub type UdpResult<T> = Result<T, UdpError>;
