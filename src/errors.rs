use thiserror::Error;

#[derive(Error, Debug)]
pub enum JvmError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<&str> for JvmError {
    fn from(error: &str) -> Self {
        JvmError::Unknown(error.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for JvmError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        JvmError::Unknown(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, JvmError>;