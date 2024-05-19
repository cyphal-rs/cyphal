use crate::{Composite, Directive, Primitive};

/// Represents a DSDL statement
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Represents a comment
    Comment(String),

    /// Represets a composite type
    Composite(Composite),

    /// Represents a primitive type
    Primitive(Primitive),

    /// Represents a directive
    Directive(Directive),

    /// Represents an empty statement
    Empty,
}
