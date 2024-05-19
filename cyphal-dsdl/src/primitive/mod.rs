mod bool_primitive;
pub use bool_primitive::BoolPrimitive;

mod float_primitive;
pub use float_primitive::FloatPrimitive;

mod int_primitive;
pub use int_primitive::IntPrimitive;

mod uint_primitive;
pub use uint_primitive::UintPrimitive;

mod void_primitive;
pub use void_primitive::VoidPrimitive;

use crate::{DsdlError, DsdlResult};

/// Represents the primitive's type
#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    /// bool
    Bool(BoolPrimitive),

    /// int
    Int(IntPrimitive),

    /// unsigned int
    Uint(UintPrimitive),

    /// floating point
    Float(FloatPrimitive),

    /// void (padding field)
    Void(VoidPrimitive),
}

impl Primitive {
    pub(crate) fn parse(line: &str) -> DsdlResult<Primitive> {
        if line.starts_with("int") {
            let primitive = IntPrimitive::parse(line)?;
            Ok(Primitive::Int(primitive))
        } else if line.starts_with("uint") {
            let primitive = UintPrimitive::parse(line)?;
            Ok(Primitive::Uint(primitive))
        } else if line.starts_with("float") {
            Err(DsdlError::NotImplemented)
        } else if line.starts_with("bool") {
            let primitive = BoolPrimitive::parse(line)?;
            Ok(Primitive::Bool(primitive))
        } else if line.starts_with("void") {
            let primitive = VoidPrimitive::parse(line)?;
            Ok(Primitive::Void(primitive))
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

#[cfg(test)]
mod test {
    use crate::Primitive;

    #[test]
    #[ignore = "not implemented"]
    fn test_int_enum() {
        let int8 = "int8 name";
        let result = Primitive::parse(&int8);
        assert!(result.is_ok());

        let primitive = result.unwrap();
        assert!(matches!(primitive, Primitive::Int { .. }));
    }

    #[test]
    fn test_uint_enum() {
        let uint8 = "uint8 name";
        let result = Primitive::parse(&uint8);
        assert!(result.is_ok());

        let primitive = result.unwrap();
        assert!(matches!(primitive, Primitive::Uint { .. }));
    }

    #[test]
    #[ignore = "not implemented"]
    fn test_float_enum() {
        let float = "float16 name";
        let result = Primitive::parse(&float);
        assert!(result.is_ok());

        let primitive = result.unwrap();
        assert!(matches!(primitive, Primitive::Float { .. }));
    }
}
