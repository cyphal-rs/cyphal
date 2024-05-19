use crate::{DsdlResult, File, Statement};
use std::{collections::HashMap, path::Path};

/// Represents a DSDL parser
#[derive(Debug)]
pub struct Parser {
    files: HashMap<File, Vec<Statement>>,
}

impl Parser {
    /// Constructs a new DSDL Parser
    pub fn new() -> DsdlResult<Self> {
        Ok(Self {
            files: HashMap::new(),
        })
    }

    /// Reads a DSDL file
    pub fn parse_dsdl(&mut self, path: &Path) -> DsdlResult<Vec<Statement>> {
        let tuple = File::parse(path)?;
        self.files.insert(tuple.0, tuple.1.clone());

        Ok(tuple.1)
    }
}

#[cfg(test)]
mod test {
    use crate::{DsdlResult, Parser, Statement};
    use std::path::PathBuf;

    #[test]
    #[ignore = "not implemented"]
    fn test_7509_heartbeat_1_0() {
        let dsdl = "tests/assets/public_regulated_data_types/uavcan/node/7509.Heartbeat.1.0.dsdl";

        let result = parse_dsdl(dsdl);
        assert!(result.is_ok());

        let statements = result.unwrap();
        assert_eq!(statements.len(), 37);
    }

    #[test]
    fn test_health_1_0() {
        let dsdl = "tests/assets/public_regulated_data_types/uavcan/node/Health.1.0.dsdl";

        let result = parse_dsdl(dsdl);
        assert!(result.is_ok());

        let statements = result.unwrap();
        assert_eq!(statements.len(), 22);
    }

    #[test]
    fn test_mode_1_0() {
        let dsdl = "tests/assets/public_regulated_data_types/uavcan/node/Mode.1.0.dsdl";
        let result = parse_dsdl(dsdl);

        assert!(result.is_ok());

        let statements = result.unwrap();
        assert_eq!(statements.len(), 18);
    }

    #[test]
    fn test_version_1_0() {
        let dsdl = "tests/assets/public_regulated_data_types/uavcan/node/Version.1.0.dsdl";
        let result = parse_dsdl(dsdl);

        assert!(result.is_ok());

        let statements = result.unwrap();
        assert_eq!(statements.len(), 7);
    }

    fn parse_dsdl(dsdl: &str) -> DsdlResult<Vec<Statement>> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(dsdl);

        let mut parser = Parser::new().expect("Could not construct parser");
        parser.parse_dsdl(&path)
    }
}
