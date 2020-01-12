#[derive(Debug)]
pub enum Error {
    NetworkError(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::NetworkError(error)
    }
}
