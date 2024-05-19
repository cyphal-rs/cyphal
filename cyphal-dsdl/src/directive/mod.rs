mod assert_directive;
pub use assert_directive::AssertDirective;

mod extent_directive;
pub use extent_directive::ExtentDirective;

use crate::{Comment, DsdlError, DsdlResult};

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

#[cfg(test)]
mod test {
    use crate::Directive;

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
}
