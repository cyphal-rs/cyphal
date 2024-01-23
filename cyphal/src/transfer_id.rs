pub struct TransferId(u64);

impl TransferId {
    pub fn increment(self) -> Self {
        TransferId(self.0.wrapping_add(1))
    }
}

impl From<TransferId> for u64 {
    fn from(id: TransferId) -> Self {
        id.0
    }
}

impl From<u64> for TransferId {
    fn from(value: u64) -> Self {
        TransferId(value)
    }
}
