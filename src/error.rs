use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to compile {name:?}:\n{message:?}")]
    CompileError { name: String, message: String },
    #[error("Failed to link {name:?}:\n{message:?}")]
    LinkError { name: String, message: String },
    #[error("Failed to get executable path")]
    FailedToGetExePath,
    #[error("Failed to determine shader type for {0}")]
    UnknownShaderType(String),
    #[error("IO error: {0}")]
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}