use async_std::{net::UdpSocket as Socket, task::block_on};
use cyphal_udp::{Udp, UdpError, UdpResult};

/// Represents a UDP Socket
pub struct UdpSocket {
    socket: Socket,
}

impl UdpSocket {
    /// Constructs a new UDP Socket Socket
    pub fn new(address: &str) -> UdpResult<Self> {
        match block_on(Socket::bind(address)) {
            Ok(socket) => Ok(UdpSocket { socket }),
            Err(_) => Err(UdpError::InvalidAddress),
        }
    }
}

impl Udp for UdpSocket {
    async fn send(&self, address: &str, data: &[u8]) -> UdpResult<usize> {
        match self.socket.send_to(data, address).await {
            Ok(bytes) => Ok(bytes),
            Err(_) => Err(UdpError::Connection),
        }
    }

    async fn receive(&self, data: &mut [u8]) -> UdpResult<usize> {
        match self.socket.recv(data).await {
            Ok(bytes) => Ok(bytes),
            Err(_) => Err(UdpError::Connection),
        }
    }
}
