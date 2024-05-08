use cyphal_can::{CanError, CanId, CanResult, Frame as CyphalFrame, FD_PAYLOAD_SIZE};
use fdcan::{
    frame::{FrameFormat, TxFrameHeader},
    id::ExtendedId,
};

/// Represents a CAN 2.0 Frame
pub struct FdFrame {
    can_id: CanId,
    header: TxFrameHeader,
    data: [u8; FD_PAYLOAD_SIZE],
}

impl FdFrame {
    pub(crate) fn header(&self) -> TxFrameHeader {
        self.header
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.data
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
                let header = TxFrameHeader {
                    len: n as u8,
                    id: extended_id.into(),
                    frame_format: FrameFormat::Fdcan,
                    bit_rate_switching: false,
                    marker: None,
                };
                Ok(FdFrame {
                    can_id,
                    header,
                    data: bytes,
                })
            }
            _ => Err(CanError::Other),
        }
    }

    fn id(&self) -> CanId {
        self.can_id
    }

    fn dlc(&self) -> usize {
        self.header.len as usize
    }

    fn data(&self) -> &[u8] {
        &self.data
    }
}
