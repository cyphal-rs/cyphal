use crate::{Comment, DsdlError, DsdlResult, Expression};

/// Represents a directive
#[derive(Debug, Clone, PartialEq)]
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

        if line.starts_with("@assert") {
            let directive = AssertDirective::parse(line)?;
            Ok(Directive::Assert(directive))
        } else if line.starts_with("@deprecated") {
            //TODO: implement @deprecated directive
            Err(DsdlError::NotImplemented)
        } else if line.starts_with("@extent") {
            let directive = ExtentDirective::parse(line)?;
            Ok(Directive::Extent(directive))
        } else if line.starts_with("@print") {
            //TODO: implement @print directive
            Err(DsdlError::NotImplemented)
        } else if let Some(line) = line.strip_prefix("@sealed") {
            let comment = Comment::parse(line)?;
            Ok(Directive::Sealed(comment))
        } else if line.starts_with("@union") {
            //TODO: implement @union directive
            Err(DsdlError::NotImplemented)
        } else {
            Err(DsdlError::OutOfRange("Unrecognized directive".to_string()))
        }
    }
}

/// Represents an Assert Directive
#[derive(Debug, Clone, PartialEq)]
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

    pub(crate) fn parse(line: &str) -> DsdlResult<Self> {
        if let Some(line) = line.strip_prefix("@assert") {
            let result = Expression::parse(line)?;
            let expression = result.0;
            let comment = match result.1 {
                Some(s) => Comment::parse(&s)?,
                None => None,
            };
            Ok(AssertDirective::new(expression, comment)?)
        } else {
            Err(DsdlError::OutOfRange("Unrecognized directive".to_string()))
        }
    }
}

/// Represents an Extent Directive
#[derive(Debug, Clone, PartialEq)]
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

    pub(crate) fn parse(line: &str) -> DsdlResult<Self> {
        if let Some(line) = line.strip_prefix("@extent") {
            let result = Expression::parse(line)?;
            let expression = result.0;
            let comment = match result.1 {
                Some(s) => Comment::parse(&s)?,
                None => None,
            };

            ExtentDirective::new(expression, comment)
        } else {
            Err(DsdlError::OutOfRange("Unrecognized directive".to_string()))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{AssertDirective, Directive, ExtentDirective};

    #[test]
    fn test_assert_enum() {
        let assert = "@assert _offset_ % 8 == {0}";
        let result = Directive::parse(&assert);
        assert!(result.is_ok());

        let directive = result.unwrap();
        assert!(matches!(directive, Directive::Assert { .. }));
    }

    #[test]
    fn test_assert_enum_with_comment() {
        let assert = "@assert _offset_ == {56}  # Fits into a single-frame Classic CAN transfer";
        let result = Directive::parse(&assert);
        assert!(result.is_ok());

        let directive = result.unwrap();
        assert!(matches!(directive, Directive::Assert { .. }));
    }

    #[test]
    fn test_extent_enum() {
        let extent = "@extent 12 * 8";
        let result = Directive::parse(&extent);
        assert!(result.is_ok());

        let directive = result.unwrap();
        assert!(matches!(directive, Directive::Extent { .. }));
    }

    #[test]
    fn test_assert() {
        let assert = "@assert _offset_ % 8 == {0}";
        let result = AssertDirective::parse(&assert);
        assert!(result.is_ok());

        let target = result.unwrap();
        assert_eq!(target.expression.value(), "_offset_ % 8 == {0}");
        assert!(target.comment().is_none())
    }

    #[test]
    fn test_assert_with_comment() {
        let assert = "@assert _offset_ == {56}  # Fits into a single-frame Classic CAN transfer";
        let result = AssertDirective::parse(&assert);
        assert!(result.is_ok());

        let target = result.unwrap();
        assert_eq!(target.expression.value(), "_offset_ == {56}");
        assert!(target
            .comment()
            .clone()
            .is_some_and(|c| c.text() == " Fits into a single-frame Classic CAN transfer"))
    }

    #[test]
    fn test_extent() {
        let extent = "@extent 12 * 8";
        let result = ExtentDirective::parse(&extent);
        assert!(result.is_ok());

        let target = result.unwrap();
        assert_eq!(target.expression.value(), "12 * 8");
        assert!(target.comment().is_none())
    }
}
