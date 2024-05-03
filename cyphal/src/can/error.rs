use embedded_can::{Error as EmbeddedError, ErrorKind};

#[derive(Debug)]
pub enum CanError {
    #[cfg(feature = "socketcan")]
    Socketcan(),
    InvalidFrame,
}

impl EmbeddedError for CanError {
    fn kind(&self) -> ErrorKind {
        todo!()
    }
}

pub type CanResult<T> = core::result::Result<T, CanError>;
