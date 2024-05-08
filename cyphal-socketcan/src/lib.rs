//! Open Cyphal SocketCAN Rust Implementation
#![warn(missing_docs)]

mod can_socket;
pub use can_socket::CanSocket;

mod can_fd_socket;
pub use can_fd_socket::CanFdSocket;

mod frame;
pub use frame::Frame;

mod fd_frame;
pub use fd_frame::FdFrame;

#[cfg(test)]
pub(crate) mod test;
