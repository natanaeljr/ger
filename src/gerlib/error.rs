#[derive(Display, Debug)]
pub enum Error {
    ///
    WrongHttpResponseCode(u32),
    /// Response is not JSON, maybe missed the magic prefix
    NotJsonResponse(String),
}

impl std::error::Error for Error {}
