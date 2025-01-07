use std::io::Error as IoError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum ParseError {
    #[error("Mask was invalid {invalid_set_bytes:2}")]
    InvalidMask { invalid_set_bytes: u32 },
}
#[derive(ThisError, Debug)]
pub enum DisasemblerError {
    #[error("Parse error: {0}")]
    Parse(ParseError),
    #[error("Failed to read file: {0}")]
    FileError(IoError),
}
