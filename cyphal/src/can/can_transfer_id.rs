#[derive(Debug, Copy, Clone)]
pub struct CanTransferId {
    value: u8,
}

impl CanTransferId {
    pub fn new(value: u8) -> Self {
        CanTransferId { value }
    }
}

impl crate::TransferId<u8> for CanTransferId {
    fn value(&self) -> u8 {
        self.value
    }

    fn next(&self) -> Self {
        if self.value > 31 {
            Self::new(self.value + 1)
        } else {
            Self::new(0)
        }
    }
}
