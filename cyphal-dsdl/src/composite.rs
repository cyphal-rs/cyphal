use crate::DsdlResult;

/// Represents a primitive type
#[derive(Debug, PartialEq)]
pub struct Composite {
    namespace: Vec<String>,
    ctype: String,
    major: u8,
    minor: u8,
    name: String,
    comment: Option<String>,
}

impl Composite {
    /// Constructs a new composite type
    pub fn new(
        namespace: Vec<String>,
        ctype: String,
        major: u8,
        minor: u8,
        name: String,
        comment: Option<String>,
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
}
