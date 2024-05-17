use crate::{DsdlError, DsdlResult};

/// Represents the primitive's type
#[derive(Debug, PartialEq)]
pub enum Primitive {
    /// bool
    Bolean(BoolPrimitive),

    /// int
    Int(IntPrimitive),

    /// unsigned int
    Uint(UintPrimitive),

    /// floating point
    Float(FloatPrimitive),
}

/// Represents a bolean Primitive Type
#[derive(Debug, PartialEq)]
pub struct BoolPrimitive {
    name: String,
    value: Option<bool>,
    comment: Option<String>,
}

impl BoolPrimitive {
    /// Constructs a new int primitive
    pub fn new(name: String, value: Option<bool>, comment: Option<String>) -> DsdlResult<Self> {
        Ok(Self {
            name,
            value,
            comment,
        })
    }

    /// Returns the name of the primnitive
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the value if it has one
    pub fn value(&self) -> Option<bool> {
        self.value
    }

    /// Returns the comment if it has one
    pub fn comment(&self) -> Option<String> {
        self.comment.clone()
    }
}

/// Represents a integer Primitive Type
#[derive(Debug, PartialEq)]
pub struct IntPrimitive {
    bits: u8,
    name: String,
    value: Option<i64>,
    comment: Option<String>,
}

impl IntPrimitive {
    /// Constructs a new int primitive
    pub fn new(
        bits: u8,
        name: String,
        value: Option<i64>,
        comment: Option<String>,
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
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the value if it has one
    pub fn value(&self) -> Option<i64> {
        self.value
    }

    /// Returns the comment if it has one
    pub fn comment(&self) -> Option<String> {
        self.comment.clone()
    }
}

/// Represents a unsigned integer Primitive Type
#[derive(Debug, PartialEq)]
pub struct UintPrimitive {
    bits: u8,
    name: String,
    value: Option<u64>,
    comment: Option<String>,
}

impl UintPrimitive {
    /// Constructs a new int primitive
    pub fn new(
        bits: u8,
        name: String,
        value: Option<u64>,
        comment: Option<String>,
    ) -> DsdlResult<Self> {
        if bits > 64 {
            return Err(DsdlError::OutOfRange(
                "The maximum number of bits an unsigned integer can have is 64".to_string(),
            ));
        }

        if let Some(v) = value {
            if v > 2_u64.pow(bits as u32) {
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
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the value if it has one
    pub fn value(&self) -> Option<u64> {
        self.value
    }

    /// Returns the comment if it has one
    pub fn comment(&self) -> Option<String> {
        self.comment.clone()
    }
}

/// Represents a floating point Primitive Type
#[derive(Debug, PartialEq)]
pub struct FloatPrimitive {
    bits: u8,
    name: String,
    value: Option<f64>,
    comment: Option<String>,
}

impl FloatPrimitive {
    /// Constructs a new int primitive
    pub fn new(
        bits: u8,
        name: String,
        value: Option<f64>,
        comment: Option<String>,
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
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the value if it has one
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Returns the comment if it has one
    pub fn comment(&self) -> Option<String> {
        self.comment.clone()
    }
}
