use crate::accounts::*;
use crate::details::Timestamp;
use serde_derive::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

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
    pub labels: Option<BTreeMap<String, LabelInfo>>,
    /// A map of the permitted labels that maps a label name to the list of values that are allowed
    /// for that label. Only set if detailed labels are requested.
    pub permitted_labels: Option<HashMap<String, Vec<String>>>,
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

impl std::fmt::Display for SubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match *self {
            SubmitType::Inherit => "Inherit",
            SubmitType::FastForwardOnly => "Fast-Forward only",
            SubmitType::MergeIfNecessary => "Merge if Necessary",
            SubmitType::AlwaysMerge => "Always Merge",
            SubmitType::CherryPick => "Cherry-Pick",
            SubmitType::RebaseIfNecessary => "Rebase if Necessary",
            SubmitType::RebaseAlways => "Rebase Always",
        })
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// Reviewer State
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewerState {
    /// Users with at least one non-zero vote on the change.
    Reviewer,
    /// Users that were added to the change, but have not voted.
    Cc,
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
pub struct LabelInfo {
    /// Whether the label is optional. Optional means the label may be set,
    /// but it’s neither necessary for submission nor does it block submission if set.
    #[serde(default)]
    pub optional: bool,
    /// One user who approved this label on the change (voted the maximum value) as an AccountInfo.
    pub approved: Option<AccountInfo>,
    /// One user who rejected this label on the change (voted the minimum value) as an AccountInfo.
    pub rejected: Option<AccountInfo>,
    /// One user who recommended this label on the change (voted positively,
    /// but not the maximum value) as an AccountInfo entity.
    pub recommended: Option<AccountInfo>,
    /// One user who disliked this label on the change (voted negatively, but not the minimum value)
    /// as an AccountInfo entity.
    pub disliked: Option<AccountInfo>,
    /// If true, the label blocks submit operation. If not set, the default is false.
    #[serde(default)]
    pub blocking: bool,
    /// List of all approvals for this label as a list of ApprovalInfo entities. Items in this list
    /// may not represent actual votes cast by users; if a user votes on any label, a corresponding
    /// ApprovalInfo will appear in this list for all labels.
    pub all: Option<Vec<ApprovalInfo>>,
    /// The voting value of the user who recommended/disliked this label on the change
    /// if it is not “+1”/“-1”.
    pub value: Option<i32>,
    /// The default voting value for the label. This value may be outside the range specified
    /// in permitted_labels.
    pub default_value: Option<i32>,
    /// A map of all values that are allowed for this label.
    /// The map maps the values (“-2”, “-1”, " `0`", “+1”, “+2”) to the value descriptions.
    pub values: Option<HashMap<String, String>>,
}

/// The ApprovalInfo entity contains information about an approval from a user for a label on a change.
/// ApprovalInfo has the same fields as AccountInfo. In addition to the following fields:
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApprovalInfo {
    /// The account information entity.
    #[serde(flatten)]
    pub account: AccountInfo,
    /// The vote that the user has given for the label. If present and zero, the user is permitted
    /// to vote on the label. If absent, the user is not permitted to vote on that label.
    pub value: Option<i32>,
    /// The VotingRangeInfo the user is authorized to vote on that label. If present, the user is
    /// permitted to vote on the label regarding the range values. If absent, the user is not
    /// permitted to vote on that label.
    pub permitted_voting_range: Option<VotingRangeInfo>,
    /// The time and date describing when the approval was made.
    pub date: Option<Timestamp>,
    /// Value of the tag field from ReviewInput set while posting the review. Votes/comments that
    /// contain tag with 'autogenerated:' prefix can be filtered out in the web UI. NOTE: To apply
    /// different tags on different votes/comments multiple invocations of the REST call are required.
    pub tag: Option<String>,
    /// If true, this vote was made after the change was submitted.
    #[serde(default)]
    pub post_submit: bool,
}

/// The VotingRangeInfo entity describes the continuous voting range from min to max values.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VotingRangeInfo {
    /// The minimum voting value.
    pub min: i32,
    /// The maximum voting value.
    pub max: i32,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ReviewerUpdateInfo {}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeMessageInfo {}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RevisionInfo {
    /// The commit of the patch set as CommitInfo entity.
    pub commit: Option<CommitInfo>,
    /// The files of the patch set as a map that maps the file names to FileInfo entities.
    /// Only set if CURRENT_FILES or ALL_FILES option is requested.
    pub files: Option<BTreeMap<String, FileInfo>>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FileInfo {
    /// The status of the file
    #[serde(default)]
    pub status: FileStatus,
    /// Whether the file is binary.
    #[serde(default)]
    pub binary: bool,
    /// The old file path.
    /// Only set if the file was renamed or copied.
    pub old_path: Option<String>,
    /// Number of inserted lines.
    /// Not set for binary files or if no lines were inserted.
    /// An empty last line is not included in the count and hence this number can differ by one
    /// from details provided in <<#diff-info,DiffInfo>>.
    pub lines_inserted: Option<u32>,
    /// Number of deleted lines.
    /// Not set for binary files or if no lines were deleted.
    /// An empty last line is not included in the count and hence this number can differ by one
    /// from details provided in <<#diff-info,DiffInfo>>.
    pub lines_deleted: Option<u32>,
    /// Number of bytes by which the file size increased/decreased.
    pub size_delta: i32,
    /// File size in bytes.
    pub size: Option<u32>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FileStatus {
    #[serde(rename = "M")]
    Modified,
    #[serde(rename = "A")]
    Added,
    #[serde(rename = "D")]
    Deleted,
    #[serde(rename = "R")]
    Renamed,
    #[serde(rename = "C")]
    Copied,
    #[serde(rename = "W")]
    Rewritten,
}

impl Default for FileStatus {
    fn default() -> Self {
        FileStatus::Modified
    }
}

impl FileStatus {
    pub fn initial(&self) -> char {
        match *self {
            FileStatus::Modified => 'M',
            FileStatus::Added => 'A',
            FileStatus::Deleted => 'D',
            FileStatus::Renamed => 'R',
            FileStatus::Copied => 'C',
            FileStatus::Rewritten => 'W',
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitInfo {
    /// The commit ID. Not set if included in a RevisionInfo entity that is contained in a map
    /// which has the commit ID as key.
    pub commit: Option<String>,
    /// The parent commits of this commit as a list of CommitInfo entities.
    /// In each parentonly the commit and subject fields are populated.
    pub parents: Option<Vec<CommitInfo>>,
    /// The author of the commit as a GitPersonInfo entity.
    pub author: Option<GitPersonInfo>,
    /// The committer of the commit as a GitPersonInfo entity.
    pub committer: Option<GitPersonInfo>,
    /// The subject of the commit (header line of the commit message).
    pub subject: String,
    /// The commit message.
    pub message: Option<String>,
    /// Links to the commit in external sites as a list of WebLinkInfo entities.
    pub web_links: Option<WebLinkInfo>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GitPersonInfo {
    /// The name of the author/committer.
    pub name: String,
    /// The email address of the author/committer.
    pub email: String,
    /// The timestamp of when this identity was constructed.
    pub date: Timestamp,
    /// The timezone offset from UTC of when this identity was constructed.
    pub tz: i32,
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
