use crate::{Comment, DsdlError, DsdlResult, Expression};

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
    pub fn comment(&self) -> Option<&Comment> {
        self.comment.as_ref()
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
    use crate::ExtentDirective;

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
