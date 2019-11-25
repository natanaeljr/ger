use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////
/// The status of the change.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChangeStatus {
    NEW,
    MERGED,
    ABANDONED,
    DRAFT,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// The ChangeInfo entity contains information about a change.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeInfo {
    /// The ID of the change in the format "'<project>~<branch>~<Change-Id>'",
    /// where 'project', 'branch' and 'Change-Id' are URL encoded.
    /// For 'branch' the refs/heads/ prefix is omitted.
    pub id: String,
    /// The name of the project.
    pub project: String,
    /// The name of the target branch. The refs/heads/ prefix is omitted.
    pub branch: String,
    /// The topic to which this change belongs.
    pub topic: Option<String>,
    /// The Change-Id of the change.
    pub change_id: Option<String>,
    /// The subject of the change (header line of the commit message).
    pub subject: String,
    /// The status of the change.
    pub status: ChangeStatus,
    /// The timestamp of when the change was last updated.
    #[serde(with = "super::details::serde_timestamp")]
    pub updated: DateTime<Utc>,
    /// The legacy numeric ID of the change.
    pub _number: u32,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
pub struct ChangeOptions {
    pub queries: Vec<Query>,
    pub additional_opts: Vec<AddiotionalOpt>,
    pub limit: Option<u32>,
    pub start: Option<u32>,
}

impl ChangeOptions {
    pub fn new() -> Self {
        ChangeOptions {
            queries: Vec::new(),
            additional_opts: Vec::new(),
            limit: None,
            start: None,
        }
    }

    pub fn queries(mut self, queries: Vec<Query>) -> Self {
        self.queries = queries;
        self
    }

    pub fn additional_opts(mut self, add_opts: Vec<AddiotionalOpt>) -> Self {
        self.additional_opts = add_opts;
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn start(mut self, start: u32) -> Self {
        self.start = Some(start);
        self
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Query(Vec<QueryOpt>);

///////////////////////////////////////////////////////////////////////////////////////////////////
pub enum QueryOpt {
    Is(ChangeIs),
    Topic(String),
    Branch(String),
    Project(String),
    Owner(Owner),
    Change(String),
    Limit(u32),
    Not,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
pub enum AddiotionalOpt {
    Labels,
    DetailedLabels,
    CurrentRevision,
    AllRevision,
    DownloadCommands,
    Messages,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
pub enum ChangeIs {
    Assigned,
    Unassigned,
    Starred,
    Watched,
    Reviewed,
    Owner,
    Reviewer,
    CC,
    Ignored,
    New,
    Open,
    Pending,
    Closed,
    Merged,
    Abandoned,
    Submittable,
    Mergeable,
    Private,
    WIP,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
pub enum Owner {
    _Self_,
    Other(String),
}
