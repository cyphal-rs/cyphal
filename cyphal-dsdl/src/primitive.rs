use crate::{Comment, DsdlError, DsdlResult, Name};

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

impl Primitive {
    pub(crate) fn parse(line: &str) -> DsdlResult<Primitive> {
        if line.starts_with("int") {
            Err(DsdlError::NotImplemented)
        } else if let Some(s) = line.strip_prefix("uint") {
            let result = parse_bits(s)?;
            let bits = result.0;
            let result = if let Some(s) = result.1 {
                Name::parse(s)?
            } else {
                return Err(DsdlError::Parse(
                    "Primitive type is missing a name".to_string(),
                ));
            };
            let name = result.0;

            match result.1 {
                None => {
                    let primitive = UintPrimitive::new(bits, name, None, None)?;
                    Ok(Primitive::Uint(primitive))
                }
                Some(s) => {
                    let result = parse_uint_value(s)?;
                    let value = result.0;
                    let comment = match result.1 {
                        Some(s) => Comment::parse(s)?,
                        None => None,
                    };
                    let primitive = UintPrimitive::new(bits, name, value, comment)?;
                    Ok(Primitive::Uint(primitive))
                }
            }
        } else if line.starts_with("float") {
            Err(DsdlError::NotImplemented)
        } else if let Some(s) = line.strip_prefix("bool") {
            let result = Name::parse(s)?;
            let name = result.0;
            match result.1 {
                None => {
                    let primitive = BoolPrimitive::new(name, None, None)?;
                    Ok(Primitive::Bolean(primitive))
                }
                Some(s) => {
                    let result = parse_bool_value(s)?;
                    let value = result.0;
                    let comment = match result.1 {
                        Some(s) => Comment::parse(s)?,
                        None => None,
                    };
                    let primitive = BoolPrimitive::new(name, value, comment)?;
                    Ok(Primitive::Bolean(primitive))
                }
            }
        } else {
            Err(DsdlError::OutOfRange("Unrecognized primitive".to_string()))
        }
    }
}

fn parse_bits(line: &str) -> DsdlResult<(u8, Option<&str>)> {
    let mut chars = line.chars();
    let mut v: Vec<char> = Vec::new();
    let mut bits_length = 0;
    if let Some(c) = chars.next() {
        if c.is_numeric() {
            v.push(c);
            bits_length += 1;
        } else {
            return Err(DsdlError::Parse(
                "Found non numeric value after the '=' sign".to_string(),
            ));
        }
    } else {
        return Err(DsdlError::Parse("Bit length not found".to_string()));
    }

    if let Some(c) = chars.next() {
        if c.is_numeric() {
            v.push(c);
            bits_length += 1;
            if let Some(c) = chars.next() {
                if c.is_numeric() {
                    return Err(DsdlError::Parse(
                        "Found three digit bit value after the '=' sign".to_string(),
                    ));
                } else if c != ' ' {
                    return Err(DsdlError::Parse(
                        "Found non numeric value after the '=' sign".to_string(),
                    ));
                }
            }
        } else if c != ' ' {
            return Err(DsdlError::Parse(
                "Found non numeric value after the '=' sign".to_string(),
            ));
        }
    }

    let value: String = v.into_iter().collect();
    let value = value.parse::<u8>().unwrap();

    let line = &line[bits_length..];
    let line = if line.is_empty() { None } else { Some(line) };

    Ok((value, line))
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

fn parse_uint_value(line: &str) -> DsdlResult<(Option<u64>, Option<&str>)> {
    let mut line = line.trim_start();
    if line.is_empty() {
        return Ok((None, None));
    }

    if line.starts_with('=') {
        line = line[1..].trim_start();
        let chars = line.chars();
        let mut v: Vec<char> = Vec::new();
        let mut bits_length = 0;
        for c in chars {
            if c.is_numeric() {
                v.push(c);
                bits_length += 1;
            } else if c != ' ' {
                bits_length += 1;
                break;
            }
        }

        let value: String = v.into_iter().collect();
        let value = value.parse::<u64>()?;

        let line = &line[bits_length..];
        let line = if line.is_empty() { None } else { Some(line) };

        Ok((Some(value), line))
    } else {
        Ok((None, Some(line)))
    }
}

/// Represents a bolean Primitive Type
#[derive(Debug, PartialEq)]
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
    pub fn value(&self) -> Option<bool> {
        self.value
    }

    /// Returns the comment if it has one
    pub fn comment(&self) -> &Option<Comment> {
        &self.comment
    }
}

/// Represents a integer Primitive Type
#[derive(Debug, PartialEq)]
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
    pub fn comment(&self) -> &Option<Comment> {
        &self.comment
    }
}

/// Represents a unsigned integer Primitive Type
#[derive(Debug, PartialEq)]
pub struct UintPrimitive {
    bits: u8,
    name: Name,
    value: Option<u64>,
    comment: Option<Comment>,
}

impl UintPrimitive {
    /// Constructs a new int primitive
    pub fn new(
        bits: u8,
        name: Name,
        value: Option<u64>,
        comment: Option<Comment>,
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
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Returns the value if it has one
    pub fn value(&self) -> Option<u64> {
        self.value
    }

    /// Returns the comment if it has one
    pub fn comment(&self) -> &Option<Comment> {
        &self.comment
    }
}

/// Represents a floating point Primitive Type
#[derive(Debug, PartialEq)]
pub struct FloatPrimitive {
    bits: u8,
    name: Name,
    value: Option<f64>,
    comment: Option<String>,
}

impl FloatPrimitive {
    /// Constructs a new int primitive
    pub fn new(
        bits: u8,
        name: Name,
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
    pub fn name(&self) -> &Name {
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
