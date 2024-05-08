use crate::Frame;
use cyphal_can::{Can as CyphalCan, CanError};
use fdcan::{FdCan, Instance};

/// /// Represents a CAN 2.0 BXCAN interface
pub struct Can<I: Instance> {
    bxcan: FdCan<I, MODE>,
}

impl<I: Instance> CyphalCan<8> for Can<I> {
    type Frame = Frame;

    fn transmit(&mut self, frame: &Self::Frame) -> cyphal_can::CanResult<()> {
        match self.bxcan.transmit(frame.inner_frame()) {
            Ok(_) => Ok(()),
            Err(_) => Err(CanError::Other),
        }
    }

    fn receive(&mut self) -> cyphal_can::CanResult<Self::Frame> {
        todo!()
    }
}
