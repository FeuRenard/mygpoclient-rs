//! Error handling

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
