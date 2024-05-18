use crate::{BoolPrimitive, DsdlError, DsdlResult, Primitive, UintPrimitive};

use super::parse_comment;

pub fn parse_primitive(line: &str) -> DsdlResult<Primitive> {
    if line.starts_with("int") {
        Err(DsdlError::NotImplemented)
    } else if let Some(s) = line.strip_prefix("uint") {
        let r = parse_bits(s)?;
        let bits = r.0;
        let r = if let Some(s) = r.1 {
            parse_name(s)?
        } else {
            return Err(DsdlError::Parse("Primitive is missing a name".to_string()));
        };
        let name = r.0.to_string();

        match r.1 {
            None => {
                let primitive = UintPrimitive::new(bits, name, None, None)?;
                Ok(Primitive::Uint(primitive))
            }
            Some(s) => {
                let r = parse_uint_value(s)?;
                let value = r.0;
                let comment = match r.1 {
                    Some(s) => parse_comment(s)?,
                    None => None,
                };
                let primitive = UintPrimitive::new(bits, name, value, comment)?;
                Ok(Primitive::Uint(primitive))
            }
        }
    } else if line.starts_with("float") {
        Err(DsdlError::NotImplemented)
    } else if let Some(s) = line.strip_prefix("bool") {
        let r = parse_name(s)?;
        let name = r.0.to_string();
        match r.1 {
            None => {
                let primitive = BoolPrimitive::new(name, None, None)?;
                Ok(Primitive::Bolean(primitive))
            }
            Some(s) => {
                let r = parse_bool_value(s)?;
                let value = r.0;
                let comment = match r.1 {
                    Some(s) => parse_comment(s)?,
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

fn parse_name(line: &str) -> DsdlResult<(&str, Option<&str>)> {
    let line = line.trim_start();
    if line.is_empty() {
        return Err(DsdlError::Parse("Could not find name".to_string()));
    }

    let result = match line.split_once(' ') {
        None => (line, None),
        Some(r) => (r.0, Some(r.1)),
    };

    //TODO: make sure it's a valid name
    Ok(result)
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
