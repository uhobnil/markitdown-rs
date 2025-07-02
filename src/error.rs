use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum MarkitdownError {
    Io(io::Error),
    Zip(String),
    Conversion(String),
    InvalidFile(String),
    ParseError(String),
    NetworkError(String),
}

impl fmt::Display for MarkitdownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarkitdownError::Io(err) => write!(
                f,
                "IO error: {} - Please check file permissions and path",
                err
            ),
            MarkitdownError::Zip(msg) => write!(
                f,
                "ZIP archive error: {} - The archive may be corrupted or unsupported",
                msg
            ),
            MarkitdownError::Conversion(msg) => write!(
                f,
                "Conversion error: {} - The file format may not be supported",
                msg
            ),
            MarkitdownError::InvalidFile(msg) => write!(
                f,
                "Invalid file: {} - Please verify the file exists and is accessible",
                msg
            ),
            MarkitdownError::ParseError(msg) => write!(
                f,
                "Parse error: {} - The document structure may be invalid",
                msg
            ),
            MarkitdownError::NetworkError(msg) => write!(
                f,
                "Network error: {} - Please check your internet connection",
                msg
            ),
        }
    }
}

impl Error for MarkitdownError {}

impl From<io::Error> for MarkitdownError {
    fn from(error: io::Error) -> Self {
        MarkitdownError::Io(error)
    }
}

impl From<zip::result::ZipError> for MarkitdownError {
    fn from(error: zip::result::ZipError) -> Self {
        MarkitdownError::Zip(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, MarkitdownError>;
