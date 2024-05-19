use super::parse_bits;
use crate::{Comment, DsdlError, DsdlResult, Name};

/// Represents a unsigned integer Primitive Type
#[derive(Debug, Clone, PartialEq)]
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

    pub(crate) fn parse(line: &str) -> DsdlResult<Self> {
        if let Some(s) = line.strip_prefix("uint") {
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
                None => UintPrimitive::new(bits, name, None, None),
                Some(s) => {
                    let result = parse_uint_value(s)?;
                    let value = result.0;
                    let comment = match result.1 {
                        Some(s) => Comment::parse(s)?,
                        None => None,
                    };
                    UintPrimitive::new(bits, name, value, comment)
                }
            }
        } else {
            Err(DsdlError::Parse(
                "The unint primitive prefix was not found".to_string(),
            ))
        }
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
