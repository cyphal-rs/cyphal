use crate::FdFrame;
use cyphal_can::{Can as CyphalCan, CanError, FD_PAYLOAD_SIZE};
use fdcan::{FdCan, Instance, Transmit};

/// /// Represents a CAN 2.0 BXCAN interface
pub struct Can<I: Instance, M: Transmit> {
    fdcan: FdCan<I, M>,
}

impl<I: Instance, M: Transmit> Can<I, M> {}

impl<I: Instance, M: Transmit> CyphalCan<FD_PAYLOAD_SIZE> for Can<I, M> {
    type Frame = FdFrame;

    async fn transmit(&mut self, frame: &Self::Frame) -> cyphal_can::CanResult<()> {
        match self.fdcan.transmit(frame.header(), frame.data()) {
            Ok(_) => Ok(()),
            Err(_) => Err(CanError::Other),
        }
    }

    async fn receive(&mut self) -> cyphal_can::CanResult<Self::Frame> {
        todo!()
    }
}
