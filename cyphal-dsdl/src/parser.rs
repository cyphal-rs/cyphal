use crate::{DsdlResult, File};
use std::{collections::HashMap, fs, path::Path};

/// Represents a DSDL parser
#[derive(Debug)]
pub struct Parser {
    files: HashMap<String, File>,
}

impl Parser {
    /// Constructs a new DSDL Parser
    pub fn new() -> DsdlResult<Self> {
        Ok(Self {
            files: HashMap::new(),
        })
    }

    /// Reads a DSDL file
    pub fn parse_dsdl(&mut self, path: &Path) -> DsdlResult<&File> {
        let path = fs::canonicalize(path)?;
        let file = File::parse(&path)?;
        let path = file.path().to_string();
        self.files.insert(path.clone(), file);

        Ok(self.files.get(&path).unwrap())
    }
}

#[cfg(test)]
mod test {
    use crate::Parser;
    use std::path::PathBuf;

    const UAVCAN: &str = "tests/assets/public_regulated_data_types/uavcan";

    #[test]
    #[ignore = "not implemented"]
    fn test_7509_heartbeat_1_0() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(UAVCAN);
        path.push("node/7509.Heartbeat.1.0.dsdl");

        let mut parser = Parser::new().expect("Could not construct parser");
        let result = parser.parse_dsdl(&path);
        assert!(result.is_ok());

        let file = result.unwrap();
        assert_eq!(file.name(), "Heartbeat");
        assert_eq!(file.major(), 1);
        assert_eq!(file.minor(), 0);
        assert_eq!(file.port(), Some(&7509));
        assert_eq!(file.path(), path.to_str().unwrap());
        assert_eq!(file.statements().len(), 37);
    }

    #[test]
    fn test_health_1_0() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(UAVCAN);
        path.push("node/Health.1.0.dsdl");

        let mut parser = Parser::new().expect("Could not construct parser");
        let result = parser.parse_dsdl(&path);
        assert!(result.is_ok());

        let file = result.unwrap();
        assert_eq!(file.name(), "Health");
        assert_eq!(file.major(), 1);
        assert_eq!(file.minor(), 0);
        assert_eq!(file.port(), None);
        assert_eq!(file.path(), path.to_str().unwrap());
        assert_eq!(file.statements().len(), 22);
    }

    #[test]
    fn test_mode_1_0() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(UAVCAN);
        path.push("node/Mode.1.0.dsdl");

        let mut parser = Parser::new().expect("Could not construct parser");
        let result = parser.parse_dsdl(&path);
        assert!(result.is_ok());

        let file = result.unwrap();
        assert_eq!(file.name(), "Mode");
        assert_eq!(file.major(), 1);
        assert_eq!(file.minor(), 0);
        assert_eq!(file.port(), None);
        assert_eq!(file.path(), path.to_str().unwrap());
        assert_eq!(file.statements().len(), 18);
    }

    #[test]
    fn test_version_1_0() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(UAVCAN);
        path.push("node/Version.1.0.dsdl");

        let mut parser = Parser::new().expect("Could not construct parser");
        let result = parser.parse_dsdl(&path);
        assert!(result.is_ok());

        let file = result.unwrap();
        assert_eq!(file.name(), "Version");
        assert_eq!(file.major(), 1);
        assert_eq!(file.minor(), 0);
        assert_eq!(file.port(), None);
        assert_eq!(file.path(), path.to_str().unwrap());
        assert_eq!(file.statements().len(), 7);
    }
}
