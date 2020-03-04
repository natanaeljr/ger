use http::uri::PathAndQuery;
use std::fmt::Write;
use crate::rest::http::HttpRequestHandler;

pub struct RestApiHandler {
    http: HttpRequestHandler,
}

impl RestApiHandler {
    pub fn new() -> Result<Self, failure::Error> {
        Ok(Self {
            http: HttpRequestHandler::new("".parse()?, "", "")?,
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
                write!(err_str, "\nResponse: {}", response)?;
            }
            return Err(failure::err_msg(err_str));
        }
        Ok(response)
    }

    pub fn post_json(
        &mut self, path_and_query: PathAndQuery, rc: u32, data: &[u8], verbose: bool,
    ) -> Result<String, failure::Error> {
        let response = self.post(path_and_query, rc, data, verbose)?;
        Self::json(response.as_str())
    }

    fn post(
        &mut self, path_and_query: PathAndQuery, rc: u32, data: &[u8], verbose: bool,
    ) -> Result<String, failure::Error> {
        let (code, response) = self.http.post(path_and_query.as_str(), Some(data))?;
        if code != rc {
            let mut err_str = String::new();
            write!(err_str, "HTTP request failed: code {}", code)?;
            if verbose {
                write!(err_str, "\nResponse: {}", response)?;
            }
            return Err(failure::err_msg(err_str));
        }
        Ok(response)
    }

    pub fn put_json(
        &mut self, path_and_query: PathAndQuery, rc: u32, data: &[u8], verbose: bool,
    ) -> Result<String, failure::Error> {
        let response = self.put(path_and_query, rc, data, verbose)?;
        Self::json(response.as_str())
    }

    fn put(
        &mut self, path_and_query: PathAndQuery, rc: u32, data: &[u8], verbose: bool,
    ) -> Result<String, failure::Error> {
        let (code, response) = self.http.put(path_and_query.as_str(), Some(data))?;
        if code != rc {
            let mut err_str = String::new();
            write!(err_str, "HTTP request failed: code {}", code)?;
            if verbose {
                write!(err_str, "\nResponse: {}", response)?;
            }
            return Err(failure::err_msg(err_str));
        }
        Ok(response)
    }

    pub fn delete(
        &mut self, path_and_query: PathAndQuery, verbose: bool,
    ) -> Result<String, failure::Error> {
        let (code, response) = self.http.delete(path_and_query.as_str())?;
        if code != 204 {
            let mut err_str = String::new();
            write!(err_str, "HTTP request failed: code {}", code)?;
            if verbose {
                write!(err_str, "\nResponse: {}", response)?;
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
