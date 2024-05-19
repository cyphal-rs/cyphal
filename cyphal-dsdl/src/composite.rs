use crate::{Comment, DsdlError, DsdlResult, Name};

/// Represents a primitive type
#[derive(Debug, Clone, PartialEq)]
pub struct Composite {
    namespace: Vec<String>,
    ctype: String,
    major: u8,
    minor: u8,
    name: Name,
    comment: Option<Comment>,
}

impl Composite {
    /// Constructs a new composite type
    pub fn new(
        namespace: Vec<String>,
        ctype: String,
        major: u8,
        minor: u8,
        name: Name,
        comment: Option<Comment>,
    ) -> DsdlResult<Self> {
        Ok(Self {
            namespace,
            ctype,
            major,
            minor,
            name,
            comment,
        })
    }

    /// Returns the namespace
    pub fn namespace(&self) -> &[String] {
        &self.namespace
    }

    /// Returns the type
    pub fn ctype(&self) -> &str {
        &self.ctype
    }

    /// Returns the major version number
    pub fn major(&self) -> u8 {
        self.major
    }

    /// Returns the minor version number
    pub fn minor(&self) -> u8 {
        self.minor
    }

    /// Returns the name
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Returns an optional comment
    pub fn comment(&self) -> &Option<Comment> {
        &self.comment
    }

    pub(crate) fn parse(line: &str) -> DsdlResult<Composite> {
        let result = match line.split_once(' ') {
            None => {
                return Err(DsdlError::Parse(
                    "Expected a name after the composite type declaration".to_string(),
                ))
            }
            Some(r) => r,
        };

        let mut namespace: Vec<String> = result.0.split('.').map(|s| s.to_string()).collect();
        if namespace.len() < 3 {
            return Err(DsdlError::Parse(
                "Composite type declaration is expected to be in the {TYPE}.{MAJOR}.{MINOR} format"
                    .to_string(),
            ));
        }
        let mut parts = namespace.split_off(namespace.len() - 3);
        let minor = parts.pop().unwrap().parse::<u8>()?;
        let major = parts.pop().unwrap().parse::<u8>()?;
        let ctype = parts.pop().unwrap().to_string();

        let result = Name::parse(result.1)?;
        let name = result.0;

        let comment = match result.1 {
            Some(s) => Comment::parse(s)?,
            None => None,
        };

        Composite::new(namespace, ctype, major, minor, name, comment)
    }
}

#[cfg(test)]
mod test {
    use crate::Composite;

    #[test]
    fn test_composite() {
        let ctype = "Mode.1.0 mode";

        let result = Composite::parse(&ctype);
        assert!(result.is_ok());

        let target = result.unwrap();
        assert_eq!(target.ctype(), "Mode");
        assert_eq!(target.namespace().len(), 0);
        assert_eq!(target.major(), 1);
        assert_eq!(target.minor(), 0);
        assert_eq!(target.name().text(), "mode");
        assert!(target.comment().is_none());
    }

    #[test]
    fn test_composite_with_namespace() {
        let ctype = "uavcan.node.Heartbeat.1.2 heartbeat   # some comment ";

        let result = Composite::parse(&ctype);
        assert!(result.is_ok());

        let target = result.unwrap();
        assert_eq!(target.ctype(), "Heartbeat");
        assert_eq!(target.namespace().len(), 2);
        assert_eq!(target.namespace()[0], "uavcan");
        assert_eq!(target.namespace()[1], "node");
        assert_eq!(target.major(), 1);
        assert_eq!(target.minor(), 2);
        assert_eq!(target.name().text(), "heartbeat");
        assert!(target
            .comment()
            .clone()
            .is_some_and(|c| c.text() == " some comment"));
    }

    #[test]
    fn test_composite_missing_name() {
        let ctype = "uavcan.node.Heartbeat.1.2 # some comment";

        let result = Composite::parse(&ctype);
        assert!(result.is_err());
    }

    #[test]
    fn test_composite_missing_version() {
        let ctype = "Heartbeat.1 # some comment";

        let result = Composite::parse(&ctype);
        assert!(result.is_err());
    }

    #[test]
    fn test_compositewith_namespace_missing_version() {
        let ctype = "uavcan.node.Heartbeat # some comment";

        let result = Composite::parse(&ctype);
        assert!(result.is_err());
    }
}
