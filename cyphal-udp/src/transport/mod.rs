use cyphal::Transport;

/// Represents a UDP Transport
pub struct UdpTransport {}

impl Transport for UdpTransport {
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
