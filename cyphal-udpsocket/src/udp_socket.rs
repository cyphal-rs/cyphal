use std::net::{Ipv4Addr, SocketAddrV4};

use async_std::{net::UdpSocket as Socket, task::block_on};
use cyphal_udp::{GroupAddress, Udp, UdpError, UdpResult, MULTIGROUP_ADDRESS_PORT};

/// Represents a UDP Socket
pub struct UdpSocket<const MAX_PAYLOAD_SIZE: usize> {
    socket: Socket,
}

impl<const MAX_PAYLOAD_SIZE: usize> UdpSocket<MAX_PAYLOAD_SIZE> {
    /// Constructs a new UDP Socket Socket
    pub fn new(address: GroupAddress) -> UdpResult<Self> {
        let socketaddr = SocketAddrV4::new(address.into(), MULTIGROUP_ADDRESS_PORT);
        match block_on(Socket::bind(socketaddr)) {
            Ok(socket) => match socket.join_multicast_v4(address.into(), Ipv4Addr::UNSPECIFIED) {
                Ok(_) => Ok(UdpSocket { socket }),
                Err(_) => Err(UdpError::InvalidAddress),
            },
            Err(_) => Err(UdpError::InvalidAddress),
        }
    }
}

impl<const MAX_PAYLOAD_SIZE: usize> Udp<MAX_PAYLOAD_SIZE> for UdpSocket<MAX_PAYLOAD_SIZE> {
    async fn send(&self, address: &str, data: &[u8]) -> UdpResult<usize> {
        if data.len() > MAX_PAYLOAD_SIZE {
            return Err(UdpError::MaxPayloadExceded);
        }

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
