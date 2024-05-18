use super::{parse_comment, parse_name};
use crate::{Composite, DsdlError, DsdlResult};

pub fn parse_composite(line: &str) -> DsdlResult<Composite> {
    let result = match line.split_once(' ') {
        None => {
            return Err(DsdlError::Parse(
                "Expected a name after the composite type declaration".to_string(),
            ))
        }
        Some(r) => r,
    };

    let mut namespace: Vec<String> = result.0.split('.').map(|s| s.to_string()).collect();
    let mut parts = namespace.split_off(namespace.len() - 3);
    let minor = parts.pop().unwrap().parse::<u8>()?;
    let major = parts.pop().unwrap().parse::<u8>()?;
    let ctype = parts.pop().unwrap().to_string();

    let result = parse_name(result.1)?;
    let name = result.0.to_string();

    let comment = match result.1 {
        Some(s) => parse_comment(s)?,
        None => None,
    };

    Composite::new(namespace, ctype, major, minor, name, comment)
}
