use crate::{Composite, Directive, DsdlError, DsdlResult, Primitive, Statement};
use std::{
    io::{BufRead as _, BufReader},
    path::Path,
};

const DSDL_EXTENTION: &str = "dsdl";
const ERROR_DIECTORY: &str = "Expecting a DSDL file but received a directory instead";
const ERROR_EXTENTION: &str = "A DSDL file must have a dsdl extention";
const ERROR_FORMAT: &str = "DSDL file name format is not valid";
const ERROR_PORT_ID: &str = "Could not parse the DSDL file name fixed Port-ID";
const ERROR_MAJOR_VERSION: &str = "Could not parse the DSDL file name major version number";
const ERROR_MINOR_VERSION: &str = "Could not parse the DSDL file name minor version number";

/// Represents a file
#[derive(Debug)]
pub struct File {
    port: Option<u16>,
    name: String,
    major: u8,
    minor: u8,
    path: String,
    statements: Vec<Statement>,
}

impl File {
    /// Returns the port id if one exists
    pub fn port(&self) -> Option<&u16> {
        self.port.as_ref()
    }

    /// Returns the file name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the major version
    pub fn major(&self) -> u8 {
        self.major
    }

    /// Returns the minor version
    pub fn minor(&self) -> u8 {
        self.minor
    }

    /// Returns the path
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the DSDL statements
    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }

    pub(crate) fn parse(path: &Path) -> DsdlResult<Self> {
        // Make sure it's not a directory
        if path.is_dir() {
            return Err(DsdlError::File(ERROR_DIECTORY.to_string()));
        }

        // Make sure it has a dsdl extention
        match path.extension() {
            None => return Err(DsdlError::File(ERROR_EXTENTION.to_string())),
            Some(ext) => {
                if ext != DSDL_EXTENTION {
                    return Err(DsdlError::File(ERROR_EXTENTION.to_string()));
                }
            }
        }

        // split the file name into the various components
        let pieces: Vec<&str> = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split('.')
            .collect();

        // mnake sure the number of components is correct
        let has_port = match pieces.len() {
            5 => true,
            4 => false,
            _ => return Err(DsdlError::File(ERROR_FORMAT.to_string())),
        };

        let mut position = 0;

        // get port number
        let port_id = if has_port {
            match pieces[position].parse::<u16>() {
                Ok(value) => {
                    position += 1;
                    Some(value)
                }
                Err(_) => return Err(DsdlError::File(ERROR_PORT_ID.to_string())),
            }
        } else {
            None
        };

        // get short name
        let short_name = pieces[position].to_string();
        position += 1;

        // get major version
        let major = match pieces[position].parse::<u8>() {
            Ok(value) => {
                position += 1;
                value
            }
            Err(_) => return Err(DsdlError::File(ERROR_MAJOR_VERSION.to_string())),
        };

        // get minor version
        let minor = match pieces[position].parse::<u8>() {
            Ok(value) => value,
            Err(_) => return Err(DsdlError::File(ERROR_MINOR_VERSION.to_string())),
        };

        let target = std::fs::File::open(path)?;
        let mut reader = BufReader::new(target);

        let mut statements: Vec<Statement> = Vec::new();
        let mut line_number = 1;

        loop {
            let mut line = String::new();
            let len = reader.read_line(&mut line)?;
            if len == 0 {
                // reached EoF
                break;
            }

            // remove new lines
            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }

            // ignore leading empty spaces
            let line = line.trim_start().to_string();

            if line.is_empty() {
                statements.push(Statement::Empty)
            } else if let Some(s) = line.strip_prefix('#') {
                statements.push(Statement::Comment(s.to_string()))
            } else if line.starts_with("bool")
                || line.starts_with("float")
                || line.starts_with("int")
                || line.starts_with("uint")
                || line.starts_with("void")
            {
                match Primitive::parse(&line) {
                    Ok(p) => statements.push(Statement::Primitive(p)),
                    Err(e) => {
                        return Err(DsdlError::InvalidStatement(
                            line_number,
                            format!("Could not parse primitive: {}", e),
                        ))
                    }
                }
            } else if line.starts_with('@') {
                match Directive::parse(&line) {
                    Ok(d) => statements.push(Statement::Directive(d)),
                    Err(e) => {
                        return Err(DsdlError::InvalidStatement(
                            line_number,
                            format!("Could not parse directive: {}", e),
                        ))
                    }
                }
            } else {
                match Composite::parse(&line) {
                    Ok(c) => statements.push(Statement::Composite(c)),
                    Err(e) => {
                        return Err(DsdlError::InvalidStatement(
                            line_number,
                            format!("Could not parse composite type: {}", e),
                        ))
                    }
                }
            }

            line_number += 1;
        }

        if statements.is_empty() {
            return Err(DsdlError::Parse("File is empty".to_string()));
        }

        Ok(File {
            port: port_id,
            name: short_name,
            major,
            minor,
            path: path.to_str().unwrap().to_string(),
            statements,
        })
    }
}

impl From<File> for String {
    fn from(file: File) -> Self {
        match file.port {
            None => format!(
                "{}.{}.{}.{}",
                file.name, file.major, file.minor, DSDL_EXTENTION
            ),
            Some(p) => format!(
                "{}.{}.{}.{}.{}",
                p, file.name, file.major, file.minor, DSDL_EXTENTION
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{DsdlError, File};
    use std::path::Path;

    #[test]
    fn test_directory() {
        let target = Path::new("/");

        let result = File::parse(target);

        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(matches!(err, DsdlError::File { .. }));
    }

    #[test]
    fn test_invalid_extention() {
        let target = Path::new("432.GetInfo.1.0.bad");

        let result = File::parse(target);

        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(matches!(err, DsdlError::File { .. }));
    }

    #[test]
    fn test_components() {
        let targets = [
            Path::new("432.GetInfo.0.1.0.dsdl"),
            Path::new("GetInfo.1.dsdl"),
        ];

        for target in targets.iter() {
            let result = File::parse(target);

            assert!(result.is_err());
            let err = result.err().unwrap();
            assert!(matches!(err, DsdlError::File { .. }));
        }
    }

    #[test]
    fn test_invalid_port() {
        let target = Path::new("4O2.GetInfo.1.0.dsdl");

        let result = File::parse(target);

        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(matches!(err, DsdlError::File { .. }));
    }

    #[test]
    fn test_invalid_major_version() {
        let target = Path::new("432.GetInfo.a.0.dsdl");

        let result = File::parse(target);

        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(matches!(err, DsdlError::File { .. }));
    }

    #[test]
    fn test_invalid_minor_version() {
        let target = Path::new("432.GetInfo.1.a.dsdl");

        let result = File::parse(target);

        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(matches!(err, DsdlError::File { .. }));
    }
}
