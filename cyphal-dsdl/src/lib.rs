#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![allow(async_fn_in_trait)]

mod directive;
pub use directive::Directive;

mod error;
pub use error::{DsdlError, DsdlResult};

mod parser;
pub use parser::Parser;

mod primitive;
pub use primitive::{BoolPrimitive, FloatPrimitive, IntPrimitive, Primitive, UintPrimitive};

mod statement;
pub use statement::Statement;