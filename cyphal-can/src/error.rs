use core::fmt::{Display, Formatter, Result as FmtResult};

/// Represents a CAN Error
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CanError {
    /// Invalid CAN ID
    InvalidId,

    /// Overrun error
    Overrun,

    // MAC sublayer errors
    /// A bit error is detected at that bit time when the bit value that is
    /// monitored differs from the bit value sent.
    Bit,

    /// A stuff error is detected at the bit time of the sixth consecutive
    /// equal bit level in a frame field that shall be coded by the method
    /// of bit stuffing.
    Stuff,

    /// Calculated CRC sequence does not equal the received one.
    Crc,

    /// A form error shall be detected when a fixed-form bit field contains
    /// one or more illegal bits.
    Form,

    /// An ACK  error shall be detected by a transmitter whenever it does not
    /// monitor a dominant bit during the ACK slot.
    Acknowledge,

    /// A different error occurred. The original error may contain more information.
    Other,
}

impl Display for CanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::InvalidId => write!(f, "Invalid CAN ID"),
            Self::Overrun => write!(f, "The peripheral receive buffer was overrun"),
            Self::Bit => write!(
                f,
                "Bit value that is monitored differs from the bit value sent"
            ),
            Self::Stuff => write!(f, "Sixth consecutive equal bits detected"),
            Self::Crc => write!(f, "Calculated CRC sequence does not equal the received one"),
            Self::Form => write!(
                f,
                "A fixed-form bit field contains one or more illegal bits"
            ),
            Self::Acknowledge => write!(f, "Transmitted frame was not acknowledged"),
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// The result of a Cyphal CAN operation.  On failure, a `CANError` will be included.
pub type CanResult<T> = Result<T, CanError>;
