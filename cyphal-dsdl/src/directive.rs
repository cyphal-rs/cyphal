use crate::DsdlResult;

/// Represents a directive
#[derive(Debug, PartialEq)]
pub enum Directive {
    /// Represents the @sealed directive and contains an optional comment
    Assert(AssertDirective),

    /// Represents the @sealed directive and contains an optional comment
    Extent(ExtentDirective),

    /// Represents the @sealed directive and contains an optional comment
    Sealed(Option<String>),
}

/// Represents an Assert Directive
#[derive(Debug, PartialEq)]
pub struct AssertDirective {
    expression: String,
    comment: Option<String>,
}

impl AssertDirective {
    /// Constructs a new Assert Directive
    pub fn new(expression: String, comment: Option<String>) -> DsdlResult<Self> {
        Ok(Self {
            expression,
            comment,
        })
    }

    /// Returns the expression of the directive
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// Returns the comment if it has one
    pub fn comment(&self) -> Option<String> {
        self.comment.clone()
    }
}

/// Represents an Extent Directive
#[derive(Debug, PartialEq)]
pub struct ExtentDirective {
    expression: String,
    comment: Option<String>,
}

impl ExtentDirective {
    /// Constructs a new Extent Directive
    pub fn new(expression: String, comment: Option<String>) -> DsdlResult<Self> {
        Ok(Self {
            expression,
            comment,
        })
    }

    /// Returns the expression of the directive
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// Returns the comment if it has one
    pub fn comment(&self) -> Option<String> {
        self.comment.clone()
    }
}
