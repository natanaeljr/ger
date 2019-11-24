#[derive(Debug)]
pub enum Error {
    Generic(String),
    HttpRequest(curl::Error),
    JsonParse(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Generic(e) => f.write_str(e.as_str()),
            Error::HttpRequest(_) => f.write_str("HTTP Request failed"),
            Error::JsonParse(_) => f.write_str("JSON parsing failed"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Generic(_) => None,
            Error::HttpRequest(ref s) => Some(s),
            Error::JsonParse(ref s) => Some(s),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonParse(err)
    }
}

impl From<curl::Error> for Error {
    fn from(err: curl::Error) -> Self {
        Error::HttpRequest(err)
    }
}
