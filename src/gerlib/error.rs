use crate::http;

#[derive(Display, Debug)]
pub enum Error {
    /// Unexpected HTTP response status code
    WrongHttpResponseCode(u32),
    /// Response is not JSON
    NotJsonResponse(String),
    /// Failed to deserialize JSON response
    InvalidJsonResponse(serde_json::Error),
    /// The HTTP handler returned error
    HttpHandler(http::Error),
    /// Failed to generate query parameters
    WrongQuery(String),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::WrongHttpResponseCode(_) => None,
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
