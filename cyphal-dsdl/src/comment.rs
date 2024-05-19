use crate::{DsdlError, DsdlResult};

/// Represents a comment
#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    text: String,
}

impl Comment {
    /// Constructs a new comment
    pub fn new(text: String) -> DsdlResult<Self> {
        Ok(Self { text })
    }

    /// Returns the comment's text
    pub fn text(&self) -> &str {
        &self.text
    }

    pub(crate) fn parse(line: &str) -> DsdlResult<Option<Self>> {
        let line = line.trim_start();
        if line.is_empty() {
            return Ok(None);
        }

        if let Some(s) = line.strip_prefix('#') {
            Ok(Some(Self::new(s.trim_end().to_string())?))
        } else {
            Err(DsdlError::Parse(
                "Found somthing that isn't a comment".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Comment;

    #[test]
    fn test_comment() {
        let comment = "# this is a comment";
        let result = Comment::parse(&comment);
        assert!(result.is_ok());

        let option = result.unwrap();
        assert!(option.is_some());

        let target = option.unwrap();
        assert_eq!(target.text(), &comment[1..]);
    }

    #[test]
    fn test_comment_with_indent() {
        let comment = "#   this is a comment with indent";
        let result = Comment::parse(&comment);
        assert!(result.is_ok());

        let option = result.unwrap();
        assert!(option.is_some());

        let target = option.unwrap();
        assert_eq!(target.text(), &comment[1..]);
    }

    #[test]
    fn test_comment_with_trailing_spaces() {
        let comment = "# this is a comment with trailing spaces    ";
        let result = Comment::parse(&comment);
        assert!(result.is_ok());

        let option = result.unwrap();
        assert!(option.is_some());

        let target = option.unwrap();
        assert_eq!(target.text(), &comment[1..comment.len() - 4]);
    }

    #[test]
    fn test_comment_with_leading_spaces() {
        let comment = "  # this is a comment";
        let result = Comment::parse(&comment);
        assert!(result.is_ok());

        let option = result.unwrap();
        assert!(option.is_some());

        let target = option.unwrap();
        assert_eq!(target.text(), &comment[3..]);
    }

    #[test]
    fn test_no_comment() {
        let comment = "  ";
        let result = Comment::parse(&comment);
        assert!(result.is_ok());

        let option = result.unwrap();
        assert!(option.is_none());
    }

    #[test]
    fn test_invalid_comment() {
        let comment = "this is a bad comment";
        let result = Comment::parse(&comment);
        assert!(result.is_err());
    }
}
