#![no_std]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![allow(async_fn_in_trait)]

mod can;
pub use can::Can;

mod fd_frame;
pub use fd_frame::FdFrame;
