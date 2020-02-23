//! Error handling

use std::fmt;

/// Error resulting from an API request
#[derive(Debug)]
pub enum Error {
    /// Error originating from reqwest crate
    NetworkError(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::NetworkError(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::NetworkError(reqwest_error) => reqwest_error.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
