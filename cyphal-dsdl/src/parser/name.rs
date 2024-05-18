use crate::{DsdlError, DsdlResult};

pub fn parse_name(line: &str) -> DsdlResult<(&str, Option<&str>)> {
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
