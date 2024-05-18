use super::{parse_comment, parse_expression};
use crate::{AssertDirective, Directive, DsdlError, DsdlResult, ExtentDirective};

pub fn parse_directive(line: &str) -> DsdlResult<Directive> {
    if !line.starts_with('@') {
        return Err(DsdlError::Parse(
            "Directives should start with an '@' symbol".to_string(),
        ));
    }

    if let Some(line) = line.strip_prefix("@assert") {
        let result = parse_expression(line)?;
        let expression = result.0;
        let comment = match result.1 {
            Some(s) => parse_comment(&s)?,
            None => None,
        };
        let directive = AssertDirective::new(expression, comment)?;

        Ok(Directive::Assert(directive))
    } else if let Some(_line) = line.strip_prefix("@deprecated") {
        //TODO: implement @deprecated directive
        Err(DsdlError::NotImplemented)
    } else if let Some(line) = line.strip_prefix("@extent") {
        let result = parse_expression(line)?;
        let expression = result.0;
        let comment = match result.1 {
            Some(s) => parse_comment(&s)?,
            None => None,
        };
        let directive = ExtentDirective::new(expression, comment)?;

        Ok(Directive::Extent(directive))
    } else if let Some(_line) = line.strip_prefix("@print") {
        //TODO: implement @print directive
        Err(DsdlError::NotImplemented)
    } else if let Some(line) = line.strip_prefix("@sealed") {
        let comment = parse_comment(line)?;
        Ok(Directive::Sealed(comment))
    } else if let Some(_line) = line.strip_prefix("@union") {
        //TODO: implement @union directive
        Err(DsdlError::NotImplemented)
    } else {
        Err(DsdlError::OutOfRange("Unrecognized directive".to_string()))
    }
}
