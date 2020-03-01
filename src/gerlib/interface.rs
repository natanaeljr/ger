use crate::changes::TopicInput;
use crate::error::Error;
use crate::http::HttpRequestHandler;
use log::error;

type Result<T> = std::result::Result<T, crate::error::Error>;

struct GerritRestApi {
    http_handler: HttpRequestHandler,
}

impl GerritRestApi {
    pub fn get_topic(&mut self, change_id: &str) -> Result<String> {
        let url = format!("/a/changes/{}/topic", change_id);
        let (code, response) = self.http_handler.get(&url)?;
        Self::expect_http_status_code(200, code)?;
        Self::json_strip_magic_prefix(response)
    }
    pub fn set_topic(&self, change_id: &str, topic: TopicInput) {}
    pub fn delete_topic(&self, change_id: &str) {}

    fn expect_http_status_code(expected: u32, actual: u32) -> Result<()> {
        if expected != actual {
            Err(Error::WrongHttpResponseCode(actual))
        } else {
            Ok(())
        }
    }

    fn json_strip_magic_prefix(content: String) -> Result<String> {
        const MAGIC_PREFIX: &'static str = ")]}'\n";
        if !content.starts_with(MAGIC_PREFIX) {
            return Err(Error::NotJsonResponse(content));
        }
        let json = content[MAGIC_PREFIX.len()..].to_string();
        Ok(json)
    }
}
