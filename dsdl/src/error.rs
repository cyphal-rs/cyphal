#[derive(Debug, PartialEq)]
pub enum DsdlError {
    NotDefined,
    FileName(String),
}

pub type Result<T> = core::result::Result<T, DsdlError>;
