//! Open Cyphal FDCAN Rust Implementation
#![no_std]
#![allow(async_fn_in_trait)]
#![warn(missing_docs)]

mod can;
pub use can::Can;

mod fd_frame;
pub use fd_frame::FdFrame;
