use crate::{Directive, Primitive};

/// Represents a DSDL statement
#[derive(Debug, PartialEq)]
pub enum Statement {
    /// Represents a comment
    Comment(String),

    /// Represents a primitive
    Primitive(Primitive),

    /// Represents a directive
    Directive(Directive),

    /// Represents an empty statement
    Empty,
}
