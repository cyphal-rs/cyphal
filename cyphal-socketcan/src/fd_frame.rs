use cyphal_can::{CanError, CanId, CanResult, Frame as CyphalFrame, FD_PAYLOAD_SIZE};
use socketcan::{CanFdFrame, EmbeddedFrame, ExtendedId, Frame};

/// Represents a CAN FD Frame
pub struct FdFrame {
    frame: CanFdFrame,
}

impl FdFrame {
    pub(crate) fn inner_frame(&self) -> &CanFdFrame {
        &self.frame
    }
}

impl CyphalFrame<FD_PAYLOAD_SIZE> for FdFrame {
    fn new(id: impl Into<CanId>, data: &[u8]) -> CanResult<Self> {
        let can_id: CanId = id.into();
        let extended_id = ExtendedId::new(can_id.as_raw()).unwrap();
        match data.len() {
            n if n <= FD_PAYLOAD_SIZE => {
                let mut bytes: [u8; FD_PAYLOAD_SIZE] = [0; FD_PAYLOAD_SIZE];
                bytes[..n].copy_from_slice(data);
                let frame = CanFdFrame::new(extended_id, data).unwrap();
                Ok(FdFrame { frame })
            }
            _ => Err(CanError::Other),
        }
    }

    fn id(&self) -> CanId {
        CanId::new(self.frame.raw_id()).unwrap()
    }

    fn dlc(&self) -> usize {
        self.frame.dlc()
    }

    fn data(&self) -> &[u8] {
        self.frame.data()
    }
}
