use cyphal::TransferId;

/// Represents the Transfer ID used by the CAN transport.
#[derive(Debug, Copy, Clone)]
pub struct CanTransferId {
    value: u8,
}

impl CanTransferId {
    /// Constructs a new CAN Transfer ID
    pub fn new() -> Self {
        CanTransferId { value: 0 }
    }
}

impl TransferId<u8> for CanTransferId {
    fn value(&self) -> u8 {
        self.value
    }

    fn next(&self) -> Self {
        if self.value < 31 {
            CanTransferId {
                value: self.value + 1,
            }
        } else {
            CanTransferId { value: 0 }
        }
    }
}

impl Default for CanTransferId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::CanTransferId;
    use cyphal::TransferId;
    use std::vec::Vec;

    #[test]
    fn test_new() {
        let id = CanTransferId::new();

        assert_eq!(id.value, 0);
    }

    #[test]
    fn test_default() {
        let id = CanTransferId::default();

        assert_eq!(id.value, 0);
    }

    #[test]
    fn test_next() {
        let mut id = CanTransferId::default();
        let data: Vec<u8> = (0..255).collect();

        for i in data {
            assert_eq!(id.value, i % 32);
            id = id.next();
        }
    }
}
