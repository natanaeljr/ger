use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

/// The status of the change.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChangeStatus {
    NEW,
    MERGED,
    ABANDONED,
    DRAFT,
}

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
