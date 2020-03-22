use crate::error::Error;
use crate::http::{Header, HttpRequestHandler};
use http::StatusCode;
use serde::Serialize;

type Result<T> = std::result::Result<T, crate::error::Error>;

pub struct RestHandler {
    http: HttpRequestHandler,
}

impl RestHandler {
    pub fn new(http: HttpRequestHandler) -> Self {
        Self { http }
    }

    pub fn get_json(&mut self, url: &str, expect_code: StatusCode) -> Result<String> {
        self.http.headers(&[Header::AcceptAppJson])?;
        let (code, response) = self.http.get(url)?;
        Self::expect_response_code(expect_code.as_u16() as u32, code)?;
        let json = Self::strip_json_magic_prefix(response)?;
        Ok(json)
    }

    pub fn put_json<T>(&mut self, url: &str, data: &T, expect_code: StatusCode) -> Result<String>
    where
        T: Serialize + ?Sized,
    {
        self.http
            .headers(&[Header::ContentTypeAppJson, Header::AcceptAppJson])?;
        let data = serde_json::to_string(data)?;
        let (code, response) = self.http.put(url, Some(data.as_bytes()))?;
        Self::expect_response_code(expect_code.as_u16() as u32, code)?;
        let json = Self::strip_json_magic_prefix(response)?;
        Ok(json)
    }

    pub fn post_json<T>(&mut self, url: &str, data: &T, expect_code: StatusCode) -> Result<String>
    where
        T: Serialize + ?Sized,
    {
        self.http
            .headers(&[Header::ContentTypeAppJson, Header::AcceptAppJson])?;
        let data = serde_json::to_string(data)?;
        let (code, response) = self.http.post(url, Some(data.as_bytes()))?;
        Self::expect_response_code(expect_code.as_u16() as u32, code)?;
        let json = Self::strip_json_magic_prefix(response)?;
        Ok(json)
    }

    pub fn delete(&mut self, url: &str, expect_code: StatusCode) -> Result<()> {
        let (code, _) = self.http.delete(&url)?;
        Self::expect_response_code(expect_code.as_u16() as u32, code)
    }

    fn expect_response_code(expected: u32, actual: u32) -> Result<()> {
        if expected != actual {
            Err(Error::UnexpectedHttpResponse(actual))
        } else {
            Ok(())
        }
    }

    fn strip_json_magic_prefix(response: String) -> Result<String> {
        const MAGIC_PREFIX: &'static str = ")]}'\n";
        if !response.starts_with(MAGIC_PREFIX) {
            return Err(Error::NotJsonResponse(response));
        }
        let json = response[MAGIC_PREFIX.len()..].to_string();
        Ok(json)
    }

    pub fn http(self) -> HttpRequestHandler {
        self.http
    }
}
