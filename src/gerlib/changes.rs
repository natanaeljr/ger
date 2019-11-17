use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChangeStatus {
    NEW,
    MERGED,
    ABANDONED,
    DRAFT,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
/// The ChangeInfo entity contains information about a change.
pub struct ChangeInfo {
    pub id: String,
    pub project: String,
    pub branch: String,
    pub topic: Option<String>,
    pub change_id: Option<String>,
    pub subject: String,
    pub status: ChangeStatus,
    pub _number: u32,
}
