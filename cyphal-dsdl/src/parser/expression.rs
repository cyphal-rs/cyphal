use crate::{DsdlError, DsdlResult};

pub fn parse_expression(line: &str) -> DsdlResult<(String, Option<String>)> {
    let line = line.trim_start().to_string();
    if line.is_empty() {
        return Err(DsdlError::Parse("No expression is present".to_string()));
    }

    match line.find('#') {
        Some(index) => Ok((
            line[..index].trim().to_string(),
            Some(line[index..].to_string()),
        )),
        None => Ok((line, None)),
    }
}
