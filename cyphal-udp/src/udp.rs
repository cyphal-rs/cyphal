use crate::UdpResult;

/// Trait representing a UDP interface
pub trait Udp<const MAX_PAYLOAD_SIZE: usize> {
    /// Sends data on the socket to the given address.
    ///
    /// On success, returns the number of bytes written.
    async fn send(&self, address: &str, buf: &[u8]) -> UdpResult<usize>;

    /// Receives data on the socket.
    ///
    /// On success, returns the bytes received.
    async fn receive(&self, data: &mut [u8]) -> UdpResult<usize>;
}
