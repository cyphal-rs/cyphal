use crate::CyphalError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Priority {
    Exceptional = 0,
    Immediate = 1,
    Fast = 2,
    High = 3,
    Nominal = 4,
    Low = 5,
    Slow = 6,
    Optional = 7,
}

impl From<Priority> for u8 {
    fn from(priority: Priority) -> Self {
        priority as u8
    }
}

impl TryFrom<u8> for Priority {
    type Error = CyphalError;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
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
