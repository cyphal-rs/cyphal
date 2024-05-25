use crate::{Comment, DsdlError, DsdlResult, Name};

/// Represents a floating point Primitive Type
#[derive(Debug, Clone, PartialEq)]
pub struct FloatPrimitive {
    bits: u8,
    name: Name,
    value: Option<f64>,
    comment: Option<Comment>,
}

impl FloatPrimitive {
    /// Constructs a new int primitive
    pub fn new(
        bits: u8,
        name: Name,
        value: Option<f64>,
        comment: Option<Comment>,
    ) -> DsdlResult<Self> {
        if bits != 16 && bits != 32 && bits != 64 {
            return Err(DsdlError::OutOfRange(
                "A floating point number must have 16, 32 or 64 bits".to_string(),
            ));
        }

        if let Some(v) = value {
            if bits == 32 && (v > f32::MAX as f64 || v < f32::MIN as f64) {
                return Err(DsdlError::OutOfRange(
                    "The value is outside the permissable range of what the bits can hold"
                        .to_string(),
                ));
            } else if bits == 16 {
                //TODO: check f16 -> https://github.com/rust-lang/rust/issues/116909
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
    pub fn value(&self) -> Option<&f64> {
        self.value.as_ref()
    }

    /// Returns the comment if it has one
    pub fn comment(&self) -> Option<&Comment> {
        self.comment.as_ref()
    }
}
