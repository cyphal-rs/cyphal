use crate::{Udp, MAX_NODE_ID, MAX_SERVICE_ID, MAX_SUBJECT_ID};
use cyphal::{CyphalError, CyphalResult, Message, Request, Router, Transport};

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
    async fn publish<M>(&mut self, message: &M) -> CyphalResult<()>
    where
        M: Message,
    {
        if message.subject() > MAX_SUBJECT_ID || message.source().is_some_and(|id| id > MAX_NODE_ID)
        {
            return Err(CyphalError::OutOfRange);
        }

        todo!()
    }

    async fn invoque<R>(&mut self, request: &R) -> CyphalResult<R::Response>
    where
        R: Request,
    {
        if request.service() > MAX_SERVICE_ID
            || request.source() > MAX_NODE_ID
            || request.destination() > MAX_NODE_ID
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
