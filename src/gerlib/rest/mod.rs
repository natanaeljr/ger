use crate::rest::changes::{AdditionalOpt, ChangeInfo, ChangeInput, QueryParams, TopicInput};
use crate::rest::handler::RestHandler;
use crate::rest::http::HttpRequestHandler;
use ::http::StatusCode;
use url::Url;

pub use crate::rest::http::AuthMethod as HttpAuthMethod;

pub mod accounts;
pub mod changes;
pub mod details;
pub mod error;
pub mod projects;

mod handler;
mod http;

type Result<T> = std::result::Result<T, crate::rest::error::Error>;

/// Gerrit REST API over HTTP.
///
/// The API is suitable for automated tools to build upon, as well as supporting some ad-hoc scripting use cases.
pub struct GerritRestApi {
    rest: RestHandler,
}

impl GerritRestApi {
    /// Create a new GerritRestApi with the host url, username and HTTP password.
    ///
    /// Additional configuration is available through specific methods below.
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

    /// Create a new change.
    ///
    /// The change input ChangeInput entity must be provided.
    ///
    /// To create a change the calling user must be allowed to upload to code review.
    ///
    /// As response a ChangeInfo entity is returned that describes the resulting change.
    pub fn create_change(&mut self, change: &ChangeInput) -> Result<ChangeInfo> {
        let json = self.rest.post_json("/a/changes", change, StatusCode::OK)?;
        let change_info: ChangeInfo = serde_json::from_str(&json)?;
        Ok(change_info)
    }

    /// Queries changes visible to the caller.
    ///
    /// The query string must be provided by the q parameter. The n parameter can be used to limit
    /// the returned results. The no-limit parameter can be used remove the default limit on queries
    /// and return all results. This might not be supported by all index backends.
    ///
    /// As result a list of ChangeInfo entries is returned. The change output is sorted by the last
    /// update time, most recently updated to oldest updated.
    ///
    /// If the number of changes matching the query exceeds either the internal limit or
    /// a supplied n query parameter, the last change object has a _more_changes: true JSON field set.
    /// The S or start query parameter can be supplied to skip a number of changes from the list.
    /// Clients are allowed to specify more than one query by setting the q parameter multiple times.
    /// In this case the result is an array of arrays, one per query in the same order the queries were given in.
    pub fn query_changes(&mut self, query: &QueryParams) -> Result<Vec<Vec<ChangeInfo>>> {
        let params = serde_url_params::to_string(query)?;
        let url = format!(
            "/a/changes/{}{}",
            if params.is_empty() { "" } else { "?" },
            params
        );
        let json = self.rest.get_json(&url, StatusCode::OK)?;
        let changes = if query.search_queries.is_some() && query.search_queries.unwrap().len() > 1 {
            serde_json::from_str::<Vec<Vec<ChangeInfo>>>(&json)?
        } else {
            vec![serde_json::from_str::<Vec<ChangeInfo>>(&json)?]
        };
        Ok(changes)
    }

    /// Retrieves a change.
    ///
    /// Additional fields can be obtained by adding o parameters, each option requires more database
    /// lookups and slows down the query response time to the client so they are generally disabled
    /// by default. Fields are described in Query Changes.
    ///
    /// As response a ChangeInfo entity is returned that describes the change.
    pub fn get_change(
        &mut self, change_id: &str, additional_opts: Option<Vec<AdditionalOpt>>,
    ) -> Result<ChangeInfo> {
        let query = QueryParams {
            search_queries: None,
            additional_opts,
            limit: None,
            start: None,
        };
        let params = serde_url_params::to_string(&query)?;
        let url = format!(
            "/a/changes/{}/{}{}",
            change_id,
            if params.is_empty() { "" } else { "?" },
            params
        );
        let json = self.rest.get_json(&url, StatusCode::OK)?;
        let change_info: ChangeInfo = serde_json::from_str(&json)?;
        Ok(change_info)
    }

    /// Retrieves a change with labels, detailed labels, detailed accounts, reviewer updates, and messages.
    ///
    /// Additional fields can be obtained by adding o parameters, each option requires more database
    /// lookups and slows down the query response time to the client so they are generally disabled
    /// by default. Fields are described in Query Changes.
    ///
    /// As response a ChangeInfo entity is returned that describes the change.
    /// This response will contain all votes for each label and include one combined vote.
    /// The combined label vote is calculated in the following order (from highest to lowest):
    /// REJECTED > APPROVED > DISLIKED > RECOMMENDED.
    pub fn get_change_detail(
        &mut self, change_id: &str, additional_opts: Option<Vec<AdditionalOpt>>,
    ) -> Result<ChangeInfo> {
        let query = QueryParams {
            search_queries: None,
            additional_opts,
            limit: None,
            start: None,
        };
        let params = serde_url_params::to_string(&query)?;
        let url = format!(
            "/a/changes/{}/detail/{}{}",
            change_id,
            if params.is_empty() { "" } else { "?" },
            params
        );
        let json = self.rest.get_json(&url, StatusCode::OK)?;
        let change_info: ChangeInfo = serde_json::from_str(&json)?;
        Ok(change_info)
    }

    /// Retrieves the topic of a change.
    ///
    /// If the change does not have a topic an empty string is returned.
    pub fn get_topic(&mut self, change_id: &str) -> Result<String> {
        let json = &self.rest.get_json(
            format!("/a/changes/{}/topic", change_id).as_str(),
            StatusCode::OK,
        )?;
        let topic: String = serde_json::from_str(&json)?;
        Ok(topic)
    }

    /// Sets the topic of a change.
    ///
    /// The new topic must be provided in the request body inside a TopicInput entity.
    /// Any leading or trailing whitespace in the topic name will be removed.
    ///
    /// As response the new topic is returned.
    pub fn set_topic(&mut self, change_id: &str, topic: &TopicInput) -> Result<String> {
        let json = &self.rest.put_json(
            format!("/a/changes/{}/topic", change_id).as_str(),
            topic,
            StatusCode::CREATED,
        )?;
        let topic: String = serde_json::from_str(&json)?;
        Ok(topic)
    }

    /// Deletes the topic of a change.
    pub fn delete_topic(&mut self, change_id: &str) -> Result<()> {
        self.rest.delete(
            format!("/a/changes/{}/topic", change_id).as_str(),
            StatusCode::NO_CONTENT,
        )
    }
}
