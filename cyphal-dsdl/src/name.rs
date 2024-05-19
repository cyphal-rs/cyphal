use crate::{DsdlError, DsdlResult};

/// Represents a comment
#[derive(Debug, PartialEq)]
pub struct Name {
    text: String,
}

impl Name {
    /// Constructs a new comment
    pub fn new(text: String) -> DsdlResult<Self> {
        Ok(Self { text })
    }

    pub(crate) fn parse(line: &str) -> DsdlResult<(Self, Option<&str>)> {
        let line = line.trim_start();
        if line.is_empty() {
            return Err(DsdlError::Parse("Could not find name".to_string()));
        }

        let result = match line.split_once(' ') {
            None => (Self::new(line.to_string())?, None),
            Some(r) => (Self::new(r.0.to_string())?, Some(r.1)),
        };

        //TODO: make sure it's a valid name
        Ok(result)
    }
}
