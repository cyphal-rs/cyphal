/// Cyphal Error
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CyphalError {
    /// An error caused by the underlying transport
    Transport,

    /// The value is out of the permissible range
    OutOfRange,
}

/// The result of a Cyphal operation.  On failure, a `CyphalError` will be included.
pub type CyphalResult<T> = core::result::Result<T, CyphalError>;
