//! Open Cyphal FDCAN Rust Implementation
#![no_std]
#![warn(missing_docs)]

mod can;
pub use can::Can;

mod fd_frame;
pub use fd_frame::FdFrame;
