use super::accounts;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////
/// The status of the change.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChangeStatus {
    New,
    Merged,
    Abandoned,
    Draft,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Timestamp(#[serde(with = "super::details::serde_timestamp")] pub DateTime<Utc>);

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubmitType {
    Inherit,
    FastForwardOnly,
    MergeIfNecessary,
    AlwaysMerge,
    CherryPick,
    RebaseIfNecessary,
    RebaseAlways,
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
    /// The assignee of the change as an AccountInfo entity.
    pub assignee: Option<accounts::AccountInfo>,
    /// List of hashtags that are set on the change (only populated when NoteDb is enabled).
    pub hashtags: Option<Vec<String>>,
    /// The Change-Id of the change.
    pub change_id: String,
    /// The subject of the change (header line of the commit message).
    pub subject: String,
    /// The status of the change.
    pub status: ChangeStatus,
    /// The timestamp of when the change was created.
    pub created: Timestamp,
    /// The timestamp of when the change was last updated.
    pub updated: Timestamp,
    /// The timestamp of when the change was submitted.
    pub submitted: Option<Timestamp>,
    /// The user who submitted the change, as an AccountInfo entity.
    pub submitter: Option<accounts::AccountInfo>,
    /// Whether the calling user has starred this change with the default label.
    #[serde(default)]
    pub starred: bool,
    /// A list of star labels that are applied by the calling user to this change.
    /// The labels are lexicographically sorted.
    pub stars: Option<Vec<String>>,
    /// Whether the change was reviewed by the calling user. Only set if reviewed is requested.
    #[serde(default)]
    pub reviewed: bool,
    /// The submit type of the change. Not set for merged changes.
    pub submit_type: Option<SubmitType>,
    /// Whether the change is mergeable. Not set for merged changes, if the change has not yet
    /// been tested, or if the skip_mergeable option is set or when
    /// change.api.excludeMergeableInChangeInfo is set.
    pub mergeable: Option<bool>,
    /// The legacy numeric ID of the change.
    pub _number: u32,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
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

    pub fn to_query_string(&self) -> String {
        use std::fmt::Write;
        let mut result = String::new();
        for query in &self.queries {
            let sym = if result.is_empty() { '?' } else { '&' };
            let q_str = query.to_query_string();
            if !q_str.is_empty() {
                write!(result, "{}q={}", sym, query.to_query_string()).unwrap();
            }
        }
        let sym = if result.is_empty() { '?' } else { '&' };
        if let Some(limit) = self.limit {
            write!(result, "{}n={}", sym, limit).unwrap();
        }
        result
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Query(pub Vec<QueryOpt>);

impl Query {
    pub fn to_query_string(&self) -> String {
        use std::fmt::Write;
        let mut result = String::new();
        for opt in self.0.iter() {
            let add = if result.is_empty() { "" } else { "+" };
            write!(result, "{}{}", add, opt).unwrap();
        }
        result
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum QueryOpt {
    Is(ChangeIs),
    Topic(String),
    Branch(String),
    Project(String),
    Owner(Owner),
    Change(String),
    Limit(u32),
    Not(Box<QueryOpt>),
}

impl std::fmt::Display for QueryOpt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            QueryOpt::Is(i) => write!(f, "is:{}", i),
            QueryOpt::Topic(s) => write!(f, "topic:{}", s),
            QueryOpt::Branch(s) => write!(f, "branch:{}", s),
            QueryOpt::Project(s) => write!(f, "project:{}", s),
            QueryOpt::Owner(o) => write!(f, "owner:{}", o),
            QueryOpt::Change(s) => write!(f, "change:{}", s),
            QueryOpt::Limit(u) => write!(f, "limit:{}", u),
            QueryOpt::Not(q) => write!(f, "-{}", q),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum AddiotionalOpt {
    Labels,
    DetailedLabels,
    CurrentRevision,
    AllRevision,
    DownloadCommands,
    Messages,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
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

impl std::fmt::Display for ChangeIs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).to_lowercase().as_str())
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum Owner {
    _Self_,
    Other(String),
}

impl std::fmt::Display for Owner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Owner::_Self_ => f.write_str("self"),
            Owner::Other(s) => f.write_str(s.as_str()),
        }
    }
}
