use crate::Udp;
use cyphal::{CyphalResult, Transport};

/// Represents a UDP Transport
pub struct UdpTransport<const MAX_PAYLOAD_SIZE: usize, U: Udp<MAX_PAYLOAD_SIZE>> {
    #[allow(dead_code)]
    udp: U,
}

impl<const MAX_PAYLOAD_SIZE: usize, U: Udp<MAX_PAYLOAD_SIZE>> UdpTransport<MAX_PAYLOAD_SIZE, U> {
    /// Constructs a new UDP transport
    pub fn new(udp: U) -> CyphalResult<Self> {
        Ok(UdpTransport { udp })
    }
}

impl<const MAX_PAYLOAD_SIZE: usize, U: Udp<MAX_PAYLOAD_SIZE>> Transport
    for UdpTransport<MAX_PAYLOAD_SIZE, U>
{
    async fn publish<const N: usize, M: cyphal::Message<N>>(
        &mut self,
        _message: &M,
    ) -> cyphal::CyphalResult<()> {
        todo!()
    }

    async fn invoque<const N: usize, const M: usize, R: cyphal::Request<N, M>>(
        &mut self,
        _request: &R,
    ) -> cyphal::CyphalResult<R::Response> {
        todo!()
    }
}
