#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![allow(async_fn_in_trait)]

mod comment;
pub use comment::Comment;

mod composite;
pub use composite::Composite;

mod directive;
pub use directive::{AssertDirective, Directive, ExtentDirective};

mod error;
pub use error::{DsdlError, DsdlResult};

mod expression;
pub use expression::Expression;

mod name;
pub use name::Name;

mod parser;
pub use parser::Parser;

mod primitive;
pub use primitive::{BoolPrimitive, FloatPrimitive, IntPrimitive, Primitive, UintPrimitive};

mod statement;
pub use statement::Statement;
