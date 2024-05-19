#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]
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

mod file;
pub use file::File;

mod name;
pub use name::Name;

mod parser;
pub use parser::Parser;

mod primitive;
pub use primitive::{
    BoolPrimitive, FloatPrimitive, IntPrimitive, Primitive, UintPrimitive, VoidPrimitive,
};

mod statement;
pub use statement::Statement;
