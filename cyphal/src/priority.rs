use core::fmt::{write, Display};

use crate::{CyphalError, CyphalResult};

/// The priority level of a transmission
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Priority {
    /// Exceptional is the highest priority level and should only be sent when a total system failure has occurred.
    Exceptional = 0,

    /// Immediate is a high priority message.
    Immediate = 1,

    /// Fast is high priority messages but have looser latency requirements than `Immediate` messages.
    Fast = 2,

    /// High priority messages are more important than `Nominal` messages but have looser latency requirements than `Fast` messages.
    High = 3,

    ///  This is what all messages should use by default. Specifically the heartbeat messages should use this priority.
    Nominal = 4,

    /// Low priority messages are expected to be sent on a bus under all conditions but cannot prevent the delivery of nominal messages.
    Low = 5,

    /// Slow messages are low priority messages that have no time sensitivity at all.
    Slow = 6,

    ///  These messages might never be sent (theoretically) for some possible system states.
    Optional = 7,
}

impl From<Priority> for u8 {
    fn from(priority: Priority) -> Self {
        priority as u8
    }
}

impl TryFrom<u8> for Priority {
    type Error = CyphalError;

    fn try_from(value: u8) -> CyphalResult<Self> {
        match value {
            0 => Ok(Priority::Exceptional),
            1 => Ok(Priority::Immediate),
            2 => Ok(Priority::Fast),
            3 => Ok(Priority::High),
            4 => Ok(Priority::Nominal),
            5 => Ok(Priority::Low),
            6 => Ok(Priority::Slow),
            7 => Ok(Priority::Optional),
            _ => Err(CyphalError::OutOfRange),
        }
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Priority::Exceptional => write!(f, "Exceptional"),
            Priority::Immediate => write!(f, "Immediate"),
            Priority::Fast => write!(f, "Fast"),
            Priority::High => write!(f, "High"),
            Priority::Nominal => write!(f, "Nominal"),
            Priority::Low => write!(f, "Low"),
            Priority::Slow => write!(f, "Slow"),
            Priority::Optional => write!(f, "Optional"),
        }
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use std::collections::HashMap;

    use super::Priority;

    fn create_priorities_hashmap() -> HashMap<Priority, u8> {
        // Arrange
        let mut values: HashMap<Priority, u8> = HashMap::new();
        values.insert(Priority::Exceptional, 0);
        values.insert(Priority::Immediate, 1);
        values.insert(Priority::Fast, 2);
        values.insert(Priority::High, 3);
        values.insert(Priority::Nominal, 4);
        values.insert(Priority::Low, 5);
        values.insert(Priority::Slow, 6);
        values.insert(Priority::Optional, 7);

        values
    }

    #[test]
    fn u8_from_priority() {
        // Arrange
        let values = create_priorities_hashmap();

        for (priority, value) in values.iter() {
            // Act
            let target = u8::from(*priority);

            // Assert
            assert_eq!(target, *value);
        }
    }

    #[test]
    fn priority_from_u8_valid() {
        // Arrange
        let values = create_priorities_hashmap();

        for (priority, value) in values.iter() {
            // Act
            let target = Priority::try_from(*value);

            // Assert
            assert!(target.is_ok());
            assert_eq!(target.unwrap(), *priority);
        }
    }
}
