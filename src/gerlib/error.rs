use crate::http;
use failure::_core::fmt::Formatter;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    /// Unexpected HTTP response status code
    UnexpectedHttpResponse(u32),
    /// Response is not JSON
    NotJsonResponse(String),
    /// Failed to deserialize JSON response
    InvalidJsonResponse(serde_json::Error),
    /// The HTTP handler returned error
    HttpHandler(http::Error),
    /// Failed to generate query parameters
    WrongQuery(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Error::UnexpectedHttpResponse(code) => {
                write!(f, "Unexpected HTTP response code: {}", code)
            }
            Error::NotJsonResponse(_) => f.write_str("Unexpected non-JSON response"),
            Error::InvalidJsonResponse(_) => f.write_str("Failed to parse JSON response"),
            Error::HttpHandler(_) => f.write_str("Low-level HTTP Handler failure"),
            Error::WrongQuery(_) => f.write_str("Failed to generate query"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::UnexpectedHttpResponse(_) => None,
            Error::NotJsonResponse(_) => None,
            Error::InvalidJsonResponse(ref e) => Some(e),
            Error::HttpHandler(ref e) => Some(e),
            Error::WrongQuery(_) => None,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::InvalidJsonResponse(e)
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        Error::HttpHandler(e)
    }
}

impl From<serde_url_params::Error> for Error {
    fn from(e: serde_url_params::Error) -> Self {
        Error::WrongQuery(e.to_string())
    }
}
