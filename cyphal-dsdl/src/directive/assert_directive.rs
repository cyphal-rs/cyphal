use crate::{Comment, DsdlError, DsdlResult, Expression};

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

#[cfg(test)]
mod test {
    use crate::AssertDirective;

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
}
