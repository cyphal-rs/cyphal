#[cfg(feature = "can")]
mod can_socket;
#[cfg(feature = "can")]
pub use can_socket::CanSocket;

#[cfg(feature = "canfd")]
mod can_fd_socket;
#[cfg(feature = "canfd")]
pub use can_fd_socket::CanFdSocket;
