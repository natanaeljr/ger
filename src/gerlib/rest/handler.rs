use crate::rest::error::Error;
use crate::rest::http::HttpRequestHandler;
use http::StatusCode;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, crate::rest::error::Error>;

pub struct RestHandler {
    http: HttpRequestHandler,
}

impl RestHandler {
    pub fn new(http: HttpRequestHandler) -> Self {
        Self { http }
    }

    pub fn get_json<'a, J>(&mut self, url: &str, expect_code: StatusCode) -> Result<J>
    where
        J: Deserialize<'a>,
    {
        let (code, response) = self.http.get(url)?;
        Self::expect_response_code(expect_code.as_u16() as u32, code)?;
        let response = Self::strip_json_magic_prefix(response)?;
        let json: J = serde_json::from_str(&response)?;
        Ok(json)
    }

    pub fn put_json<'a, D, J>(&mut self, url: &str, data: &D, expect_code: StatusCode) -> Result<J>
    where
        D: Serialize + ?Sized,
        J: Deserialize<'a>,
    {
        let data = serde_json::to_string(data)?;
        let (code, response) = self.http.put(url, Some(data.as_bytes()))?;
        Self::expect_response_code(expect_code.as_u16() as u32, code)?;
        let response = Self::strip_json_magic_prefix(response)?;
        let json: J = serde_json::from_str(&response)?;
        Ok(json)
    }

    pub fn post_json<'a, D, J>(&mut self, url: &str, data: &D, expect_code: StatusCode) -> Result<J>
    where
        D: Serialize + ?Sized,
        J: Deserialize<'a>,
    {
        let data = serde_json::to_string(data)?;
        let (code, response) = self.http.post(url, Some(data.as_bytes()))?;
        Self::expect_response_code(expect_code.as_u16() as u32, code)?;
        let response = Self::strip_json_magic_prefix(response)?;
        let json: J = serde_json::from_str(&response)?;
        Ok(json)
    }

    pub fn delete(&mut self, url: &str, expect_code: StatusCode) -> Result<()> {
        let (code, _) = self.http.delete(&url)?;
        Self::expect_response_code(expect_code.as_u16() as u32, code)
    }

    fn expect_response_code(expected: u32, actual: u32) -> Result<()> {
        if expected != actual {
            Err(Error::WrongHttpResponseCode(actual))
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
}
