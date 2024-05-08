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
        if self.value > 31 {
            CanTransferId {
                value: self.value + 1,
            }
        } else {
            CanTransferId { value: 0 }
        }
    }
}
