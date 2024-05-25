use crate::{Comment, DsdlError, DsdlResult, Name};

/// Represents a integer Primitive Type
#[derive(Debug, Clone, PartialEq)]
pub struct IntPrimitive {
    bits: u8,
    name: Name,
    value: Option<i64>,
    comment: Option<Comment>,
}

impl IntPrimitive {
    /// Constructs a new int primitive
    pub fn new(
        bits: u8,
        name: Name,
        value: Option<i64>,
        comment: Option<Comment>,
    ) -> DsdlResult<Self> {
        if bits > 64 {
            return Err(DsdlError::OutOfRange(
                "The maximum number of bits an integer can have is 64".to_string(),
            ));
        }

        if let Some(v) = value {
            if v > (2_i64.pow((bits - 1) as u32)) || v < -(2_i64.pow(bits as u32)) {
                return Err(DsdlError::OutOfRange(
                    "The value is outside the permissable range of what the bits can hold"
                        .to_string(),
                ));
            }
        }

        Ok(Self {
            bits,
            name,
            value,
            comment,
        })
    }

    /// Returns the number of bits
    pub fn bits(&self) -> u8 {
        self.bits
    }

    /// Returns the name of the primnitive
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Returns the value if it has one
    pub fn value(&self) -> Option<i64> {
        self.value
    }

    /// Returns the comment if it has one
    pub fn comment(&self) -> Option<&Comment> {
        self.comment.as_ref()
    }

    pub(crate) fn parse(_line: &str) -> DsdlResult<Self> {
        Err(DsdlError::NotImplemented)
    }
}
