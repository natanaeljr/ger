use crate::http::HttpRequestHandler;
use crate::GerritConn;
use http::uri::PathAndQuery;
use std::fmt::Write;

pub struct RestApiHandler {
    http: HttpRequestHandler,
}

impl RestApiHandler {
    pub fn new(gerrit: GerritConn) -> Result<Self, failure::Error> {
        Ok(Self {
            http: HttpRequestHandler::new(gerrit)?,
        })
    }

    pub fn get_json(
        &mut self, path_and_query: PathAndQuery, verbose: bool,
    ) -> Result<String, failure::Error> {
        let response = self.get(path_and_query, verbose)?;
        Self::json(response.as_str())
    }

    fn get(
        &mut self, path_and_query: PathAndQuery, verbose: bool,
    ) -> Result<String, failure::Error> {
        let (code, response) = self.http.get(path_and_query.as_str())?;
        if code != 200 {
            let mut err_str = String::new();
            write!(err_str, "HTTP request failed: code {}", code)?;
            if verbose {
                write!(err_str, ", response:\n{}", response)?;
            }
            return Err(failure::err_msg(err_str));
        }
        Ok(response)
    }

    pub fn post_json(
        &mut self, path_and_query: PathAndQuery, data: &[u8], verbose: bool,
    ) -> Result<String, failure::Error> {
        let response = self.post(path_and_query, data, verbose)?;
        Self::json(response.as_str())
    }

    fn post(
        &mut self, path_and_query: PathAndQuery, data: &[u8], verbose: bool,
    ) -> Result<String, failure::Error> {
        let (code, response) = self.http.post(path_and_query.as_str(), data)?;
        if code != 201 {
            let mut err_str = String::new();
            write!(err_str, "HTTP request failed: code {}", code)?;
            if verbose {
                write!(err_str, ", response:\n{}", response)?;
            }
            return Err(failure::err_msg(err_str));
        }
        Ok(response)
    }

    fn json(response: &str) -> Result<String, failure::Error> {
        const MAGIC_PREFIX: &'static str = ")]}'\n";
        if !response.starts_with(MAGIC_PREFIX) {
            return Err(failure::err_msg(
                "Unexpected JSON response: missing magic prefix",
            ));
        }
        let json = response[MAGIC_PREFIX.len()..].to_string();
        Ok(json)
    }
}
