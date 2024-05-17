use crate::{BoolPrimitive, Directive, DsdlError, DsdlResult, Primitive, Statement, UintPrimitive};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

/// Represents a DSDL parser
pub struct Parser {}

impl Parser {
    /// Constructs a new DSDL Parser
    pub fn new() -> DsdlResult<Self> {
        Ok(Self {})
    }

    /// Reads a DSDL file
    pub fn read_dsdl(&self, reader: &mut BufReader<File>) -> DsdlResult<Vec<Statement>> {
        let mut statements: Vec<Statement> = Vec::new();
        let mut line_number = 1;

        loop {
            let mut line = String::new();
            let len = reader.read_line(&mut line)?;
            if len == 0 {
                // reached EoF
                break;
            }

            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }
            let line = line.trim().to_string();

            if line.is_empty() {
                statements.push(Statement::Empty)
            } else if let Some(s) = line.strip_prefix('#') {
                statements.push(Statement::Comment(s.trim().to_string()))
            } else if line.starts_with("int")
                || line.starts_with("uint")
                || line.starts_with("float")
                || line.starts_with("bool")
            {
                match parse_primitive(&line) {
                    Ok(p) => statements.push(Statement::Primitive(p)),
                    Err(e) => {
                        return Err(DsdlError::InvalidStatement(line_number, format!("{}", e)))
                    }
                }
            } else if line.starts_with('@') {
                match parse_directive(&line) {
                    Ok(d) => statements.push(Statement::Directive(d)),
                    Err(e) => {
                        return Err(DsdlError::InvalidStatement(line_number, format!("{}", e)))
                    }
                }
            } else {
                return Err(DsdlError::InvalidStatement(
                    line_number,
                    "Could not understand the statement".to_string(),
                ));
            }

            line_number += 1;
        }

        Ok(statements)
    }
}

fn parse_directive(line: &str) -> DsdlResult<Directive> {
    if !line.starts_with('@') {
        return Err(DsdlError::Parse(
            "Directives should start with an '@' symbol".to_string(),
        ));
    }

    let line = &line[1..];
    if let Some(s) = line.strip_prefix("sealed") {
        let comment = get_comment(s)?;
        Ok(Directive::Sealed(comment))
    } else {
        Err(DsdlError::NotImplemented)
    }
}

fn parse_primitive(line: &str) -> DsdlResult<Primitive> {
    if line.starts_with("int") {
        Err(DsdlError::NotImplemented)
    } else if let Some(s) = line.strip_prefix("uint") {
        let r = get_bits(s)?;
        let bits = r.0;
        let r = if let Some(s) = r.1 {
            get_name(s)?
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
                let r = get_uint_value(s)?;
                let value = r.0;
                let comment = match r.1 {
                    Some(s) => get_comment(s)?,
                    None => None,
                };
                let primitive = UintPrimitive::new(bits, name, value, comment)?;
                Ok(Primitive::Uint(primitive))
            }
        }
    } else if line.starts_with("float") {
        Err(DsdlError::NotImplemented)
    } else if let Some(s) = line.strip_prefix("bool") {
        let r = get_name(s)?;
        let name = r.0.to_string();
        match r.1 {
            None => {
                let primitive = BoolPrimitive::new(name, None, None)?;
                Ok(Primitive::Bolean(primitive))
            }
            Some(s) => {
                let r = get_bool_value(s)?;
                let value = r.0;
                let comment = match r.1 {
                    Some(s) => get_comment(s)?,
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

fn get_name(line: &str) -> DsdlResult<(&str, Option<&str>)> {
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

fn get_bool_value(line: &str) -> DsdlResult<(Option<bool>, Option<&str>)> {
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

fn get_uint_value(line: &str) -> DsdlResult<(Option<u64>, Option<&str>)> {
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

fn get_bits(line: &str) -> DsdlResult<(u8, Option<&str>)> {
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

fn get_comment(line: &str) -> DsdlResult<Option<String>> {
    let mut line = line.trim_start();
    if line.is_empty() {
        return Ok(None);
    }

    if line.starts_with('#') {
        line = line[1..].trim();
        Ok(Some(line.to_string()))
    } else {
        Err(DsdlError::Parse(
            "Found somthing that isn't a comment".to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::BufReader, path::PathBuf};

    use crate::{Directive, DsdlResult, Parser, Primitive, Statement, UintPrimitive};

    #[test]
    #[ignore = "not implemented"]
    fn test_7509_heartbeat_1_0() {
        let dsdl = "assets/7509.Heartbeat.1.0.dsdl";
        let result = parse_dsdl(dsdl);

        assert!(result.is_ok())
    }

    #[test]
    fn test_health_1_0() {
        let dsdl = "assets/Health.1.0.dsdl";
        let result = parse_dsdl(dsdl);

        assert!(result.is_ok());
        let statements = result.unwrap();

        assert_eq!(statements.len(), 22);
        assert_eq!(statements[0], Statement::Comment("Abstract component health information. If the node performs multiple activities (provides multiple network services),".to_string()));
        assert_eq!(statements[1], Statement::Comment("its health status should reflect the status of the worst-performing activity (network service).".to_string()));
        assert_eq!(statements[2], Statement::Comment("Follows:".to_string()));
        assert_eq!(
            statements[3],
            Statement::Comment("https://www.law.cornell.edu/cfr/text/14/23.1322".to_string())
        );
        assert_eq!(statements[4], Statement::Comment("https://www.faa.gov/documentLibrary/media/Advisory_Circular/AC_25.1322-1.pdf section 6".to_string()));
        assert_eq!(statements[5], Statement::Empty);
        assert_eq!(
            statements[6],
            Statement::Primitive(Primitive::Uint(
                UintPrimitive::new(2, "value".to_string(), None, None).unwrap()
            )),
        );
        assert_eq!(statements[7], Statement::Empty);
        assert_eq!(
            statements[8],
            Statement::Primitive(Primitive::Uint(
                UintPrimitive::new(2, "NOMINAL".to_string(), Some(0), None).unwrap()
            )),
        );
        assert_eq!(
            statements[9],
            Statement::Comment("The component is functioning properly (nominal).".to_string())
        );
        assert_eq!(statements[10], Statement::Empty);
        assert_eq!(
            statements[11],
            Statement::Primitive(Primitive::Uint(
                UintPrimitive::new(2, "ADVISORY".to_string(), Some(1), None).unwrap()
            )),
        );
        assert_eq!(statements[12], Statement::Comment("A critical parameter went out of range or the component encountered a minor failure that does not prevent".to_string()));
        assert_eq!(
            statements[13],
            Statement::Comment(
                "the subsystem from performing any of its real-time functions.".to_string()
            )
        );
        assert_eq!(statements[14], Statement::Empty);
        assert_eq!(
            statements[15],
            Statement::Primitive(Primitive::Uint(
                UintPrimitive::new(2, "CAUTION".to_string(), Some(2), None).unwrap()
            )),
        );
        assert_eq!(statements[16], Statement::Comment("The component encountered a major failure and is performing in a degraded mode or outside of its designed limitations.".to_string()));
        assert_eq!(statements[17], Statement::Empty);
        assert_eq!(
            statements[18],
            Statement::Primitive(Primitive::Uint(
                UintPrimitive::new(2, "WARNING".to_string(), Some(3), None).unwrap()
            )),
        );
        assert_eq!(statements[19], Statement::Comment("The component suffered a fatal malfunction and is unable to perform its intended function.".to_string()));
        assert_eq!(statements[20], Statement::Empty);
        assert_eq!(
            statements[21],
            Statement::Directive(Directive::Sealed(None))
        );
    }

    fn parse_dsdl(dsdl: &str) -> DsdlResult<Vec<Statement>> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(dsdl);

        let file = File::open(path).expect("Could not open file");
        let mut reader = BufReader::new(file);

        let parser = Parser::new().expect("Could not construct parser");
        parser.read_dsdl(&mut reader)
    }
}
