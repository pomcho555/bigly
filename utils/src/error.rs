use thiserror::Error;

/// A type alias that forces the usage of the custom error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Custom error type for handling errors.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),
    
    #[error("Poison error occurred")]
    PoisonError,
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Clap error: {0}")]
    ClapError(String),
    
    #[error("Logger error: {0}")]
    LoggerError(String),
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_err: std::sync::PoisonError<T>) -> Self {
        Error::PoisonError
    }
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Self {
        Error::ClapError(err.to_string())
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(err: log::SetLoggerError) -> Self {
        Error::LoggerError(err.to_string())
    }
}