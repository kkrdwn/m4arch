use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum M4ArchError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid value")]
    InvalidValue,

    #[error("Permission denied (are you root?)")]
    PermissionDenied,
}

pub type Result<T> = std::result::Result<T, M4ArchError>;
