use crate::{Comment, DsdlError, DsdlResult, Name};

/// Represents a primitive type
#[derive(Debug, PartialEq)]
pub struct Composite {
    namespace: Vec<String>,
    ctype: String,
    major: u8,
    minor: u8,
    name: Name,
    comment: Option<Comment>,
}

impl Composite {
    /// Constructs a new composite type
    pub fn new(
        namespace: Vec<String>,
        ctype: String,
        major: u8,
        minor: u8,
        name: Name,
        comment: Option<Comment>,
    ) -> DsdlResult<Self> {
        Ok(Self {
            namespace,
            ctype,
            major,
            minor,
            name,
            comment,
        })
    }

    pub(crate) fn parse(line: &str) -> DsdlResult<Composite> {
        let result = match line.split_once(' ') {
            None => {
                return Err(DsdlError::Parse(
                    "Expected a name after the composite type declaration".to_string(),
                ))
            }
            Some(r) => r,
        };

        let mut namespace: Vec<String> = result.0.split('.').map(|s| s.to_string()).collect();
        let mut parts = namespace.split_off(namespace.len() - 3);
        let minor = parts.pop().unwrap().parse::<u8>()?;
        let major = parts.pop().unwrap().parse::<u8>()?;
        let ctype = parts.pop().unwrap().to_string();

        let result = Name::parse(result.1)?;
        let name = result.0;

        let comment = match result.1 {
            Some(s) => Comment::parse(s)?,
            None => None,
        };

        Composite::new(namespace, ctype, major, minor, name, comment)
    }
}
