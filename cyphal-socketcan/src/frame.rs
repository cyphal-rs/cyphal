use cyphal_can::{CanError, CanId, CanResult, Frame as CyphalFrame, CLASSIC_PAYLOAD_SIZE};
use socketcan::{CanDataFrame, EmbeddedFrame, ExtendedId, Frame as SocketcanFrame};

/// Represents a CAN 2.0 Frame
pub struct Frame {
    frame: CanDataFrame,
}

impl Frame {
    pub(crate) fn inner_frame(&self) -> &CanDataFrame {
        &self.frame
    }
}

impl CyphalFrame<CLASSIC_PAYLOAD_SIZE> for Frame {
    fn new(id: impl Into<CanId>, data: &[u8]) -> CanResult<Self> {
        let can_id: CanId = id.into();
        let extended_id = ExtendedId::new(can_id.as_raw()).unwrap();
        match data.len() {
            n if n <= CLASSIC_PAYLOAD_SIZE => {
                let mut bytes: [u8; CLASSIC_PAYLOAD_SIZE] = [0; CLASSIC_PAYLOAD_SIZE];
                bytes[..n].copy_from_slice(data);
                let frame = CanDataFrame::new(extended_id, data).unwrap();
                Ok(Frame { frame })
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
