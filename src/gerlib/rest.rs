use crate::http::HttpRequestHandler;
use crate::GerritConn;
use http::uri::PathAndQuery;

pub struct RestRequestHandler {
    http: HttpRequestHandler,
}

impl RestRequestHandler {
    pub fn new(gerrit: GerritConn) -> Result<Self, failure::Error> {
        Ok(Self {
            http: HttpRequestHandler::new(gerrit)?,
        })
    }

    pub fn request_json(&mut self, path_and_query: PathAndQuery) -> Result<String, failure::Error> {
        let response = self.request(path_and_query)?;
        const MAGIC_PREFIX: &'static str = ")]}'\n";
        if !response.starts_with(MAGIC_PREFIX) {
            return Err(failure::err_msg("Missing MAGIC_PREFIX in JSON response"));
        }
        let json = response[MAGIC_PREFIX.len()..].to_string();
        Ok(json)
    }

    fn request(&mut self, path_and_query: PathAndQuery) -> Result<String, failure::Error> {
        let response = self.http.get(path_and_query.as_str())?;
        Ok(response)
    }
}
