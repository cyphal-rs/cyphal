//! Open Cyphal BXCAN Rust Implementation
#![no_std]
#![allow(async_fn_in_trait)]
#![warn(missing_docs)]

mod can;
pub use can::Can;

mod frame;
pub use frame::Frame;
