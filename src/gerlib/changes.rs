use crate::accounts::*;
use crate::details::Timestamp;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

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

impl std::fmt::Display for ChangeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).to_uppercase().as_str())
    }
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
    pub assignee: Option<AccountInfo>,
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
    pub submitter: Option<AccountInfo>,
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
    /// Whether the change has been approved by the project submit rules. Only set if requested.
    pub submittable: Option<bool>,
    /// Number of inserted lines.
    pub insertions: u32,
    /// Number of deleted lines.
    pub deletions: u32,
    /// Total number of inline comments across all patch sets.
    /// Not set if the current change index doesn’t have the data.
    pub total_comment_count: Option<u32>,
    /// Number of unresolved inline comment threads across all patch sets.
    /// Not set if the current change index doesn’t have the data.
    pub unresolved_comment_count: Option<u32>,
    /// The legacy numeric ID of the change.
    #[serde(rename = "_number")]
    pub number: u32,
    /// The owner of the change as an AccountInfo entity.
    pub owner: AccountInfo,
    /// Actions the caller might be able to perform on this revision.
    /// The information is a map of view name to ActionInfo entities.
    pub actions: Option<HashMap<String, ActionInfo>>,
    /// List of the requirements to be met before this change can be submitted.
    pub requirements: Option<Vec<Requirement>>,
    /// The labels of the change as a map that maps the label names to LabelInfo entries.
    /// Only set if labels or detailed labels are requested.
    pub labels: Option<HashMap<String, LabelInfo>>,
    /// A map of the permitted labels that maps a label name to the list of values that are allowed
    /// for that label. Only set if detailed labels are requested.
    pub permitted_labels: Option<HashMap<String, LabelInfo>>,
    /// The reviewers that can be removed by the calling user as a list of AccountInfo entities.
    /// Only set if detailed labels are requested.
    pub removable_reviewers: Option<Vec<AccountInfo>>,
    /// The reviewers as a map that maps a reviewer state to a list of AccountInfo entities.
    pub reviewers: Option<HashMap<ReviewerState, Vec<AccountInfo>>>,
    /// Updates to reviewers that have been made while the change was in the WIP state.
    /// Only present on WIP changes and only if there are pending reviewer updates to report.
    /// These are reviewers who have not yet been notified about being added to or removed from the change.
    /// Only set if detailed labels are requested.
    pub pending_reviewers: Option<HashMap<ReviewerState, Vec<AccountInfo>>>,
    /// Updates to reviewers set for the change as ReviewerUpdateInfo entities.
    /// Only set if reviewer updates are requested and if NoteDb is enabled.
    pub reviewer_updates: Option<Vec<ReviewerUpdateInfo>>,
    /// Messages associated with the change as a list of ChangeMessageInfo entities.
    /// Only set if messages are requested.
    pub messages: Option<Vec<ChangeMessageInfo>>,
    /// The commit ID of the current patch set of this change.
    /// Only set if the current revision is requested or if all revisions are requested.
    pub current_revision: Option<String>,
    /// All patch sets of this change as a map that maps the commit ID of the patch set
    /// to a RevisionInfo entity. Only set if the current revision is requested (in which case
    /// it will only contain a key for the current revision) or if all revisions are requested.
    pub revisions: Option<HashMap<String, RevisionInfo>>,
    /// A list of TrackingIdInfo entities describing references to external tracking systems.
    /// Only set if tracking ids are requested.
    pub tracking_ids: Option<Vec<TrackingIdInfo>>,
    /// Whether the query would deliver more results if not limited.
    /// Only set on the last change that is returned.
    #[serde(default, rename = "_more_changes")]
    pub more_changes: bool,
    /// A list of ProblemInfo entities describing potential problems with this change.
    /// Only set if CHECK is set.
    pub problems: Option<Vec<ProblemInfo>>,
    /// When present, change is marked as private.
    #[serde(default)]
    pub is_private: bool,
    /// When present, change is marked as Work In Progress.
    #[serde(default)]
    pub work_in_progress: bool,
    /// When present, change has been marked Ready at some point in time.
    #[serde(default)]
    pub has_review_started: bool,
    /// The numeric Change-Id of the change that this change reverts.
    pub revert_of: Option<u32>,
    /// ID of the submission of this change. Only set if the status is MERGED.
    pub submission_id: Option<String>,
}

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
/// Reviewer State
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewerState {
    /// Users with at least one non-zero vote on the change.
    Reviewer,
    /// Users that were added to the change, but have not voted.
    CC,
    /// Users that were previously reviewers on the change, but have been removed.
    Removed,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionInfo {}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Requirement {}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LabelInfo {}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ReviewerUpdateInfo {}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeMessageInfo {}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RevisionInfo {
    pub commit: Option<CommitInfo>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitInfo {
    pub message: String,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackingIdInfo {}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProblemInfo {}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebLinkInfo {}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct ChangeOptions {
    pub queries: Vec<Query>,
    pub additional_opts: Vec<AdditionalOpt>,
    pub limit: Option<u32>,
    pub start: Option<u32>,
}

impl ChangeOptions {
    pub fn to_query_string(&self) -> String {
        use std::fmt::Write;
        let mut result = String::new();
        for query in &self.queries {
            let sym = if result.is_empty() { '?' } else { '&' };
            let q_str = query.to_query_string();
            if !q_str.is_empty() {
                write!(result, "{}q={}", sym, q_str).unwrap();
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
pub struct Query(pub QueryOpt);

impl Query {
    pub fn to_query_string(&self) -> String {
        format!("{}", self.0)
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
pub enum AdditionalOpt {
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
    Draft,
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
