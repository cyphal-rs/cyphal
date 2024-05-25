use crate::{Comment, DsdlError, DsdlResult};

/// Represents a void Primitive Type
#[derive(Debug, Clone, PartialEq)]
pub struct VoidPrimitive {
    bits: u8,
    comment: Option<Comment>,
}

impl VoidPrimitive {
    /// Constructs a new void primitive
    pub fn new(bits: u8, comment: Option<Comment>) -> DsdlResult<Self> {
        if bits > 64 {
            return Err(DsdlError::OutOfRange(
                "The maximum number of bits a void can have is 64".to_string(),
            ));
        }

        Ok(Self { bits, comment })
    }

    /// Returns the number of bits
    pub fn bits(&self) -> u8 {
        self.bits
    }

    /// Returns the comment if it has one
    pub fn comment(&self) -> Option<&Comment> {
        self.comment.as_ref()
    }

    pub(crate) fn parse(_line: &str) -> DsdlResult<Self> {
        Err(DsdlError::NotImplemented)
    }
}
