use crate::{DsdlError, DsdlResult};

/// Represents a comment
#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    text: String,
}

impl Name {
    /// Constructs a new comment
    pub fn new(text: String) -> DsdlResult<Self> {
        Ok(Self { text })
    }

    /// Return the name in text
    pub fn text(&self) -> &str {
        &self.text
    }

    pub(crate) fn parse(line: &str) -> DsdlResult<(Self, Option<&str>)> {
        let line = line.trim_start();
        if line.is_empty() {
            return Err(DsdlError::Parse("Could not find name".to_string()));
        }

        let result = match line.split_once(' ') {
            None => (Self::new(line.to_string())?, None),
            Some(r) => (Self::new(r.0.to_string())?, Some(r.1)),
        };

        //TODO: make sure it's a valid name
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate::Name;

    #[test]
    fn test_name() {
        let name = "uptime";
        let result = Name::parse(&name);
        assert!(result.is_ok());

        let tuple = result.unwrap();
        let target = tuple.0;
        let extra = tuple.1;
        assert_eq!(target.text(), "uptime");
        assert!(extra.is_none());
    }

    #[test]
    fn test_name_with_comment() {
        let name = " uptime                       # [second]";
        let result = Name::parse(&name);
        assert!(result.is_ok());

        let tuple = result.unwrap();
        let target = tuple.0;
        let extra = tuple.1;
        assert_eq!(target.text(), "uptime");
        assert!(extra.is_some_and(|e| e == "                      # [second]"));
    }

    #[test]
    fn test_name_with_value_and_comment() {
        let name = " MAX_PUBLICATION_PERIOD = 1   # [second]";
        let result = Name::parse(&name);
        assert!(result.is_ok());

        let tuple = result.unwrap();
        let target = tuple.0;
        let extra = tuple.1;
        assert_eq!(target.text(), "MAX_PUBLICATION_PERIOD");
        assert!(extra.is_some_and(|e| e == "= 1   # [second]"));
    }
}
