use crate::changes::TopicInput;
use crate::error::Error;
use crate::http::HttpRequestHandler;
use serde::Serialize;

type Result<T> = std::result::Result<T, crate::error::Error>;

pub struct GerritRestApi {
    http: HttpRequestHandler,
}

impl GerritRestApi {
    pub fn get_topic(&mut self, change_id: &str) -> Result<String> {
        let url = format!("/a/changes/{}/topic", change_id);
        let (code, response) = self.http.get(&url)?;
        RestHandler::expect_response_code(200, code)?;
        RestHandler::strip_json_magic_prefix(response)
    }

    pub fn set_topic(&mut self, change_id: &str, topic: &TopicInput) -> Result<String> {
        let url = format!("/a/changes/{}/topic", change_id);
        let body = serde_json::to_string(&topic)?;
        let (code, response) = self.http.put(&url, Some(body.as_bytes()))?;
        RestHandler::expect_response_code(200, code)?;
        let response = RestHandler::strip_json_magic_prefix(response)?;
        let topic: String = serde_json::from_str(&response)?;
        Ok(topic)
    }

    pub fn delete_topic(&mut self, change_id: &str) -> Result<()> {
        let url = format!("/a/changes/{}/topic", change_id);
        RestHandler::request("DELETE", &url, None, http::StatusCode::NO_CONTENT)
    }
}

pub struct RestHandler {}

impl RestHandler {
    fn request<T: ?Sized>(
        method: &str, url: &str, tx_data: Option<&T>, response_code: &http::StatusCode,
    ) -> Result<()>
    where
        T: Serialize,
    {
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
