use crate::{DsdlError, DsdlResult};

pub fn parse_comment(line: &str) -> DsdlResult<Option<String>> {
    let line = line.trim_start();
    if line.is_empty() {
        return Ok(None);
    }

    if let Some(s) = line.strip_prefix('#') {
        Ok(Some(s.to_string()))
    } else {
        Err(DsdlError::Parse(
            "Found somthing that isn't a comment".to_string(),
        ))
    }
}
