use crate::Frame;
use bxcan::{Can as BxCan, Instance};
use cyphal_can::{Can as CyphalCan, CanError, CLASSIC_PAYLOAD_SIZE};

/// /// Represents a CAN 2.0 BXCAN interface
pub struct Can<I: Instance> {
    bxcan: BxCan<I>,
}

impl<I: Instance> CyphalCan<CLASSIC_PAYLOAD_SIZE> for Can<I> {
    type Frame = Frame;

    async fn transmit(&mut self, frame: &Self::Frame) -> cyphal_can::CanResult<()> {
        match self.bxcan.transmit(frame.inner_frame()) {
            Ok(_) => Ok(()),
            Err(_) => Err(CanError::Other),
        }
    }

    async fn receive(&mut self) -> cyphal_can::CanResult<Self::Frame> {
        todo!()
    }
}
