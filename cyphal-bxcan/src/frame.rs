use bxcan::{Data, ExtendedId, Frame as BxcanFrame};
use cyphal_can::{CanError, CanId, CanResult, Frame as CyphalFrame, CLASSIC_PAYLOAD_SIZE};

/// Represents a CAN 2.0 Frame
pub struct Frame {
    can_id: CanId,
    frame: BxcanFrame,
}

impl Frame {
    pub(crate) fn inner_frame(&self) -> &BxcanFrame {
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
                let frame = BxcanFrame::new_data(extended_id, Data::new(data).unwrap());
                Ok(Frame { can_id, frame })
            }
            _ => Err(CanError::Other),
        }
    }

    fn id(&self) -> CanId {
        self.can_id
    }

    fn dlc(&self) -> usize {
        match self.frame.data() {
            Some(data) => data.len(),
            None => 0,
        }
    }

    fn data(&self) -> &[u8] {
        match self.frame.data() {
            Some(data) => data,
            None => &[],
        }
    }
}
