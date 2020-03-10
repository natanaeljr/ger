use crate::rest::handler::RestHandler;
use crate::rest::http::HttpRequestHandler;
use ::http::StatusCode;
use url::Url;

pub mod accounts;
pub mod changes;
pub mod details;
pub mod error;
pub mod projects;

mod handler;
mod http;

use crate::rest::changes::ChangesEndpoint;
pub use crate::rest::http::AuthMethod as HttpAuthMethod;

type Result<T> = std::result::Result<T, crate::rest::error::Error>;

pub struct GerritRestApi {
    rest: RestHandler,
}

impl GerritRestApi {
    pub fn new(base_url: Url, username: &str, password: &str) -> Result<Self> {
        let http = HttpRequestHandler::new(base_url, username, password)?;
        let rest = RestHandler::new(http);
        Ok(Self { rest })
    }

    /// Specify the HTTP authentication method.
    pub fn http_auth(&mut self, auth: &HttpAuthMethod) -> Result<&mut Self> {
        self.rest.http_mut().http_auth(auth)?;
        Ok(self)
    }

    /// Enable/Disable SSL verification of both host and peer.
    pub fn ssl_verify(&mut self, enable: bool) -> Result<&mut Self> {
        self.rest.http_mut().ssl_verify(enable)?;
        Ok(self)
    }

    pub fn changes(&mut self) -> ChangesEndpoint {
        ChangesEndpoint::new(&mut self.rest)
    }
}
