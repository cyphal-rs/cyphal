use crate::{Composite, Directive, DsdlError, DsdlResult, Primitive, Statement};
use std::{
    fs::File,
    io::{BufRead, BufReader},
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

            // remove new lines
            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }

            // ignore leading empty spaces
            let line = line.trim_start().to_string();

            if line.is_empty() {
                statements.push(Statement::Empty)
            } else if let Some(s) = line.strip_prefix('#') {
                statements.push(Statement::Comment(s.to_string()))
            } else if line.starts_with("bool")
                || line.starts_with("float")
                || line.starts_with("int")
                || line.starts_with("uint")
                || line.starts_with("void")
            {
                match Primitive::parse(&line) {
                    Ok(p) => statements.push(Statement::Primitive(p)),
                    Err(e) => {
                        return Err(DsdlError::InvalidStatement(
                            line_number,
                            format!("Could not parse primitive: {}", e),
                        ))
                    }
                }
            } else if line.starts_with('@') {
                match Directive::parse(&line) {
                    Ok(d) => statements.push(Statement::Directive(d)),
                    Err(e) => {
                        return Err(DsdlError::InvalidStatement(
                            line_number,
                            format!("Could not parse directive: {}", e),
                        ))
                    }
                }
            } else {
                match Composite::parse(&line) {
                    Ok(c) => statements.push(Statement::Composite(c)),
                    Err(e) => {
                        return Err(DsdlError::InvalidStatement(
                            line_number,
                            format!("Could not parse composite type: {}", e),
                        ))
                    }
                }
            }

            line_number += 1;
        }

        Ok(statements)
    }
}

#[cfg(test)]
mod test {
    use crate::{DsdlResult, Parser, Statement};
    use std::{fs::File, io::BufReader, path::PathBuf};

    #[test]
    #[ignore = "not implemented"]
    fn test_7509_heartbeat_1_0() {
        let dsdl = "tests/assets/public_regulated_data_types/uavcan/node/7509.Heartbeat.1.0.dsdl";

        let result = parse_dsdl(dsdl);
        assert!(result.is_ok());

        let statements = result.unwrap();
        assert_eq!(statements.len(), 37);
    }

    #[test]
    fn test_health_1_0() {
        let dsdl = "tests/assets/public_regulated_data_types/uavcan/node/Health.1.0.dsdl";

        let result = parse_dsdl(dsdl);
        assert!(result.is_ok());

        let statements = result.unwrap();
        assert_eq!(statements.len(), 22);
    }

    #[test]
    fn test_mode_1_0() {
        let dsdl = "tests/assets/public_regulated_data_types/uavcan/node/Mode.1.0.dsdl";
        let result = parse_dsdl(dsdl);

        assert!(result.is_ok());

        let statements = result.unwrap();
        assert_eq!(statements.len(), 18);
    }

    #[test]
    fn test_version_1_0() {
        let dsdl = "tests/assets/public_regulated_data_types/uavcan/node/Version.1.0.dsdl";
        let result = parse_dsdl(dsdl);

        assert!(result.is_ok());

        let statements = result.unwrap();
        assert_eq!(statements.len(), 7);
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
