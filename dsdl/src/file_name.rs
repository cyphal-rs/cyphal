use std::path::Path;

use crate::{DsdlError, Result};

const DSDL_EXTENTION: &str = "dsdl";
const ERROR_DIECTORY: &str = "Expecting a DSDL file but received a directory instead";
const ERROR_EXTENTION: &str = "A DSDL file must have a dsdl extention";
const ERROR_FORMAT: &str = "DSDL file name format is not valid";
const ERROR_PORT_ID: &str = "Could not parse the DSDL file name fixed Port-ID";
const ERROR_MAJOR_VERSION: &str = "Could not parse the DSDL file name major version number";
const ERROR_MINOR_VERSION: &str = "Could not parse the DSDL file name minor version number";

pub struct FileName {
    port_id: u32,
    short_name: String,
    major_version: u32,
    minor_version: u32,
}

impl FileName {
    pub fn new(path: &Path) -> Result<Self> {
        // Make sure it's not a directory
        if path.is_dir() {
            return Err(DsdlError::FileName(ERROR_DIECTORY.to_string()));
        }

        // Make sure it has a dsdl extention
        match path.extension() {
            None => return Err(DsdlError::FileName(ERROR_EXTENTION.to_string())),
            Some(ext) => {
                if ext != DSDL_EXTENTION {
                    return Err(DsdlError::FileName(ERROR_EXTENTION.to_string()));
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
            _ => return Err(DsdlError::FileName(ERROR_FORMAT.to_string())),
        };

        let mut position = 0;

        // get port number
        let mut port_id: u32 = 0;
        if has_port {
            match pieces[position].parse::<u32>() {
                Ok(value) => {
                    port_id = value;
                    position += 1;
                }
                Err(_) => return Err(DsdlError::FileName(ERROR_PORT_ID.to_string())),
            }
        }

        // get short name
        let short_name = pieces[position].to_string();
        position += 1;

        // get major version
        let major_version: u32;
        match pieces[position].parse::<u32>() {
            Ok(value) => {
                major_version = value;
                position += 1;
            }
            Err(_) => return Err(DsdlError::FileName(ERROR_MAJOR_VERSION.to_string())),
        }

        // get minor version
        let minor_version: u32 = match pieces[position].parse::<u32>() {
            Ok(value) => value,
            Err(_) => return Err(DsdlError::FileName(ERROR_MINOR_VERSION.to_string())),
        };

        Ok(FileName {
            port_id,
            short_name,
            major_version,
            minor_version,
        })
    }

    pub fn port_id(&self) -> Result<u32> {
        match self.port_id {
            0 => Err(DsdlError::NotDefined),
            _ => Ok(self.port_id),
        }
    }

    pub fn short_name(&self) -> String {
        self.short_name.clone()
    }

    pub fn minor_version(&self) -> u32 {
        self.minor_version
    }

    pub fn major_version(&self) -> u32 {
        self.major_version
    }
}

impl From<FileName> for String {
    fn from(file_name: FileName) -> Self {
        match file_name.port_id {
            0 => format!(
                "{}.{}.{}.{}",
                file_name.short_name,
                file_name.major_version,
                file_name.minor_version,
                DSDL_EXTENTION
            ),
            _ => format!(
                "{}.{}.{}.{}.{}",
                file_name.port_id,
                file_name.short_name,
                file_name.major_version,
                file_name.minor_version,
                DSDL_EXTENTION
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::{
        file_name::{
            ERROR_DIECTORY, ERROR_EXTENTION, ERROR_FORMAT, ERROR_MAJOR_VERSION,
            ERROR_MINOR_VERSION, ERROR_PORT_ID,
        },
        DsdlError, FileName,
    };

    #[test]
    fn test_directory() {
        // Arrange
        let target = Path::new("/");

        // Act
        let result = FileName::new(target);

        // Assert
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err, DsdlError::FileName(ERROR_DIECTORY.to_string()));
    }

    #[test]
    fn test_invalid_extention() {
        // Arrange
        let target = Path::new("432.GetInfo.1.0.bad");

        // Act
        let result = FileName::new(target);

        // Assert
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err, DsdlError::FileName(ERROR_EXTENTION.to_string()));
    }

    #[test]
    fn test_components() {
        // Arrange
        let targets = [
            Path::new("432.GetInfo.0.1.0.dsdl"),
            Path::new("GetInfo.1.dsdl"),
        ];

        for target in targets.iter() {
            // Act
            let result = FileName::new(target);

            // Assert
            assert!(result.is_err());
            let err = result.err().unwrap();
            assert_eq!(err, DsdlError::FileName(ERROR_FORMAT.to_string()));
        }
    }

    #[test]
    fn test_invalid_port() {
        // Arrange
        let target = Path::new("4O2.GetInfo.1.0.dsdl");

        // Act
        let result = FileName::new(target);

        // Assert
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err, DsdlError::FileName(ERROR_PORT_ID.to_string()));
    }

    #[test]
    fn test_invalid_major_version() {
        // Arrange
        let target = Path::new("432.GetInfo.a.0.dsdl");

        // Act
        let result = FileName::new(target);

        // Assert
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err, DsdlError::FileName(ERROR_MAJOR_VERSION.to_string()));
    }

    #[test]
    fn test_invalid_minor_version() {
        // Arrange
        let target = Path::new("432.GetInfo.1.a.dsdl");

        // Act
        let result = FileName::new(target);

        // Assert
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err, DsdlError::FileName(ERROR_MINOR_VERSION.to_string()));
    }
}
