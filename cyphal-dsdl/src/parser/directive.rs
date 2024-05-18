use super::parse_comment;
use crate::{Directive, DsdlError, DsdlResult};

pub fn parse_directive(line: &str) -> DsdlResult<Directive> {
    if !line.starts_with('@') {
        return Err(DsdlError::Parse(
            "Directives should start with an '@' symbol".to_string(),
        ));
    }

    if let Some(_s) = line.strip_prefix("@assert") {
        //TODO: implement @assert directive
        Err(DsdlError::NotImplemented)
    } else if let Some(_s) = line.strip_prefix("@deprecated") {
        //TODO: implement @deprecated directive
        Err(DsdlError::NotImplemented)
    } else if let Some(_s) = line.strip_prefix("@print") {
        //TODO: implement @print directive
        Err(DsdlError::NotImplemented)
    } else if let Some(s) = line.strip_prefix("@sealed") {
        let comment = parse_comment(s)?;
        Ok(Directive::Sealed(comment))
    } else if let Some(_s) = line.strip_prefix("@union") {
        //TODO: implement @union directive
        Err(DsdlError::NotImplemented)
    } else {
        Err(DsdlError::OutOfRange("Unrecognized directive".to_string()))
    }
}
