use crate::{Comment, DsdlError, DsdlResult, Expression};

/// Represents a directive
#[derive(Debug, PartialEq)]
pub enum Directive {
    /// Represents the @sealed directive and contains an optional comment
    Assert(AssertDirective),

    /// Represents the @sealed directive and contains an optional comment
    Extent(ExtentDirective),

    /// Represents the @sealed directive and contains an optional comment
    Sealed(Option<Comment>),
}

impl Directive {
    pub(crate) fn parse(line: &str) -> DsdlResult<Directive> {
        if !line.starts_with('@') {
            return Err(DsdlError::Parse(
                "Directives should start with an '@' symbol".to_string(),
            ));
        }

        if let Some(line) = line.strip_prefix("@assert") {
            let result = Expression::parse(line)?;
            let expression = result.0;
            let comment = match result.1 {
                Some(s) => Comment::parse(&s)?,
                None => None,
            };
            let directive = AssertDirective::new(expression, comment)?;

            Ok(Directive::Assert(directive))
        } else if let Some(_line) = line.strip_prefix("@deprecated") {
            //TODO: implement @deprecated directive
            Err(DsdlError::NotImplemented)
        } else if let Some(line) = line.strip_prefix("@extent") {
            let result = Expression::parse(line)?;
            let expression = result.0;
            let comment = match result.1 {
                Some(s) => Comment::parse(&s)?,
                None => None,
            };
            let directive = ExtentDirective::new(expression, comment)?;

            Ok(Directive::Extent(directive))
        } else if let Some(_line) = line.strip_prefix("@print") {
            //TODO: implement @print directive
            Err(DsdlError::NotImplemented)
        } else if let Some(line) = line.strip_prefix("@sealed") {
            let comment = Comment::parse(line)?;
            Ok(Directive::Sealed(comment))
        } else if let Some(_line) = line.strip_prefix("@union") {
            //TODO: implement @union directive
            Err(DsdlError::NotImplemented)
        } else {
            Err(DsdlError::OutOfRange("Unrecognized directive".to_string()))
        }
    }
}

/// Represents an Assert Directive
#[derive(Debug, PartialEq)]
pub struct AssertDirective {
    expression: Expression,
    comment: Option<Comment>,
}

impl AssertDirective {
    /// Constructs a new Assert Directive
    pub fn new(expression: Expression, comment: Option<Comment>) -> DsdlResult<Self> {
        Ok(Self {
            expression,
            comment,
        })
    }

    /// Returns the expression of the directive
    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    /// Returns the comment if it has one
    pub fn comment(&self) -> &Option<Comment> {
        &self.comment
    }
}

/// Represents an Extent Directive
#[derive(Debug, PartialEq)]
pub struct ExtentDirective {
    expression: Expression,
    comment: Option<Comment>,
}

impl ExtentDirective {
    /// Constructs a new Extent Directive
    pub fn new(expression: Expression, comment: Option<Comment>) -> DsdlResult<Self> {
        Ok(Self {
            expression,
            comment,
        })
    }

    /// Returns the expression of the directive
    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    /// Returns the comment if it has one
    pub fn comment(&self) -> &Option<Comment> {
        &self.comment
    }
}
