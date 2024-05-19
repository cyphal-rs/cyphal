use crate::{DsdlError, DsdlResult};

/// Represents a comment
#[derive(Debug, PartialEq)]
pub struct Comment {
    text: String,
}

impl Comment {
    /// Constructs a new comment
    pub fn new(text: String) -> DsdlResult<Self> {
        Ok(Self { text })
    }

    pub(crate) fn parse(line: &str) -> DsdlResult<Option<Self>> {
        let line = line.trim_start();
        if line.is_empty() {
            return Ok(None);
        }

        if let Some(s) = line.strip_prefix('#') {
            Ok(Some(Self::new(s.to_string())?))
        } else {
            Err(DsdlError::Parse(
                "Found somthing that isn't a comment".to_string(),
            ))
        }
    }
}
