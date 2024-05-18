mod comment;
use comment::parse_comment;

mod directive;
use directive::parse_directive;

mod primitive;
use primitive::parse_primitive;

#[cfg(test)]
mod test;

use crate::{DsdlError, DsdlResult, Statement};
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
                statements.push(Statement::Comment(s.to_string()))
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
