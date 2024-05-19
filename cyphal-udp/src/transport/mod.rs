use crate::Udp;
use cyphal::{
    CyphalError, CyphalResult, Message, NodeId, Request, Router, ServiceId, SubjectId, Transport,
};

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
    /// Maximim Subject ID
    const MAX_SUBJECT_ID: SubjectId = 8191;

    /// Maximim Subject ID
    const MAX_SERVICE_ID: ServiceId = 16383;

    /// Maximim Subject ID
    const MAX_NODE_ID: NodeId = 65534;

    async fn publish<M>(&mut self, message: &M) -> CyphalResult<()>
    where
        M: Message,
    {
        if message.subject() > Self::MAX_SUBJECT_ID
            || message.source().is_some_and(|id| id > Self::MAX_NODE_ID)
        {
            return Err(CyphalError::OutOfRange);
        }

        todo!()
    }

    async fn invoque<R>(&mut self, request: &R) -> CyphalResult<R::Response>
    where
        R: Request,
    {
        if request.service() > Self::MAX_SERVICE_ID
            || request.source() > Self::MAX_NODE_ID
            || request.destination() > Self::MAX_NODE_ID
        {
            return Err(CyphalError::OutOfRange);
        }
        todo!()
    }

    async fn listen<R>(&mut self, _router: R) -> CyphalResult<()>
    where
        R: Router,
    {
        todo!()
    }
}
