use crate::{Comment, DsdlError, DsdlResult, Name};

/// Represents a boolean Primitive Type
#[derive(Debug, Clone, PartialEq)]
pub struct BoolPrimitive {
    name: Name,
    value: Option<bool>,
    comment: Option<Comment>,
}

impl BoolPrimitive {
    /// Constructs a new int primitive
    pub fn new(name: Name, value: Option<bool>, comment: Option<Comment>) -> DsdlResult<Self> {
        Ok(Self {
            name,
            value,
            comment,
        })
    }

    /// Returns the name of the primnitive
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Returns the value if it has one
    pub fn value(&self) -> &Option<bool> {
        &self.value
    }

    /// Returns the comment if it has one
    pub fn comment(&self) -> &Option<Comment> {
        &self.comment
    }

    pub(crate) fn parse(line: &str) -> DsdlResult<Self> {
        if let Some(line) = line.strip_prefix("bool") {
            let result = Name::parse(line)?;
            let name = result.0;
            match result.1 {
                None => BoolPrimitive::new(name, None, None),
                Some(s) => {
                    let result = parse_bool_value(s)?;
                    let value = result.0;
                    let comment = match result.1 {
                        Some(s) => Comment::parse(s)?,
                        None => None,
                    };
                    BoolPrimitive::new(name, value, comment)
                }
            }
        } else {
            Err(DsdlError::Parse(
                "The bool primitive prefix was not found".to_string(),
            ))
        }
    }
}

fn parse_bool_value(line: &str) -> DsdlResult<(Option<bool>, Option<&str>)> {
    let mut line = line.trim_start();
    if line.is_empty() {
        return Ok((None, None));
    }

    if line.starts_with('=') {
        line = line[1..].trim_start();
        let value = if line.starts_with("true") {
            line = &line[4..];
            Some(true)
        } else if line.starts_with("false") {
            line = &line[5..];
            Some(false)
        } else {
            return Err(DsdlError::Parse("Could not find a bool value".to_string()));
        };

        let line = if line.is_empty() { None } else { Some(line) };

        Ok((value, line))
    } else {
        Ok((None, Some(line)))
    }
}
