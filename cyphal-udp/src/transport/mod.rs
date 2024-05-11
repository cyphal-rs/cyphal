use crate::Udp;
use cyphal::{CyphalResult, Transport};

/// Represents a UDP Transport
pub struct UdpTransport<U: Udp> {
    #[allow(dead_code)]
    udp: U,
}

impl<U: Udp> UdpTransport<U> {
    /// Constructs a new UDP transport
    pub fn new(udp: U) -> CyphalResult<Self> {
        Ok(UdpTransport { udp })
    }
}

impl<U: Udp> Transport for UdpTransport<U> {
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
