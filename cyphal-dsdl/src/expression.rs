use crate::{DsdlError, DsdlResult};

/// Represents and Expression
#[derive(Debug, PartialEq)]
pub struct Expression {
    value: String,
}

impl Expression {
    /// Constructs a new expression
    pub fn new(value: String) -> DsdlResult<Self> {
        Ok(Self { value })
    }

    /// Returns the value of the expression
    pub fn value(&self) -> &str {
        &self.value
    }

    pub(crate) fn parse(line: &str) -> DsdlResult<(Self, Option<String>)> {
        let line = line.trim_start().to_string();
        if line.is_empty() {
            return Err(DsdlError::Parse("No expression is present".to_string()));
        }

        match line.find('#') {
            Some(index) => Ok((
                Self::new(line[..index].trim().to_string())?,
                Some(line[index..].to_string()),
            )),
            None => Ok((Self::new(line)?, None)),
        }
    }
}
