use std::io;

/// An enum for the error types received by this application
#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    Io(io::Error),
    Json(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}
