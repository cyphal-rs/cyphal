use cyphal::{CyphalError, CyphalResult, TransferId};

/// Represents the Transfer ID used by the UDP transport.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct UdpTransferId {
    value: u64,
}

impl UdpTransferId {
    /// Constructs a new UDP Transfer ID
    pub fn new(value: u64) -> CyphalResult<Self> {
        Ok(UdpTransferId { value })
    }
}

impl TransferId for UdpTransferId {
    type T = u64;

    fn value(&self) -> u64 {
        self.value
    }

    fn next(&self) -> Self {
        UdpTransferId {
            value: self.value + 1,
        }
    }
}

impl TryFrom<u64> for UdpTransferId {
    type Error = CyphalError;

    fn try_from(value: u64) -> CyphalResult<Self> {
        Ok(Self { value })
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::UdpTransferId;
    use cyphal::TransferId;
    use std::vec::Vec;

    #[test]
    fn test_new() {
        let value: u64 = 5;
        let id = UdpTransferId::new(value).unwrap();

        assert_eq!(id.value, value);
    }

    #[test]
    fn test_default() {
        let id = UdpTransferId::default();

        assert_eq!(id.value, 0);
    }

    #[test]
    fn test_next() {
        let mut id = UdpTransferId::default();
        let data: Vec<u64> = (0..255).collect();

        for i in data {
            assert_eq!(id.value, i);
            id = id.next();
        }
    }
}
