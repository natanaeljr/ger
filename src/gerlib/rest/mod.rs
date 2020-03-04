use crate::rest::changes::TopicInput;
use crate::rest::handler::RestHandler;
use crate::rest::http::HttpRequestHandler;
use ::http::StatusCode;
use url::Url;

pub mod accounts;
pub mod changes;
pub mod details;
pub mod error;
pub mod projects;
pub mod rest;

mod handler;
mod http;

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

    pub fn get_topic(&mut self, change_id: &str) -> Result<String> {
        let topic: String = serde_json::from_str(&self.rest.get_json(
            format!("/a/changes/{}/topic", change_id).as_str(),
            StatusCode::OK,
        )?)?;
        Ok(topic)
    }

    pub fn set_topic(&mut self, change_id: &str, topic: &TopicInput) -> Result<String> {
        let topic: String = serde_json::from_str(&self.rest.put_json(
            format!("/a/changes/{}/topic", change_id).as_str(),
            topic,
            StatusCode::CREATED,
        )?)?;
        Ok(topic)
    }

    pub fn delete_topic(&mut self, change_id: &str) -> Result<()> {
        self.rest.delete(
            format!("/a/changes/{}/topic", change_id).as_str(),
            StatusCode::NO_CONTENT,
        )
    }
}
