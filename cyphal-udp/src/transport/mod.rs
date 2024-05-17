use crate::{Udp, UdpNodeId, UdpServiceId, UdpSubjectId};
use cyphal::{CyphalResult, Message, Request, Transport};

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

    async fn publish<M>(&mut self, _message: &M) -> CyphalResult<()>
    where
        M: Message<Self::SubjectId, Self::NodeId>,
    {
        todo!()
    }

    async fn invoque<R>(&mut self, _request: &R) -> CyphalResult<R::Response>
    where
        R: Request<Self::ServiceId, Self::NodeId>,
    {
        todo!()
    }

    async fn listen<R>(&mut self, _router: R) -> CyphalResult<()>
    where
        R: cyphal::Router<Self::SubjectId, Self::ServiceId, Self::NodeId>,
    {
        todo!()
    }
}
