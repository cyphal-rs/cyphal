use crate::{Udp, UdpNodeId, UdpServiceId, UdpSubjectId};
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
    type NodeId = UdpNodeId;
    type ServiceId = UdpServiceId;
    type SubjectId = UdpSubjectId;

    async fn publish<const N: usize, M: cyphal::Message<N, Self::SubjectId, Self::NodeId>>(
        &mut self,
        _message: &M,
    ) -> cyphal::CyphalResult<()> {
        todo!()
    }

    async fn invoque<
        const N: usize,
        const M: usize,
        R: cyphal::Request<N, M, Self::ServiceId, Self::NodeId>,
    >(
        &mut self,
        _request: &R,
    ) -> cyphal::CyphalResult<R::Response> {
        todo!()
    }
}
