use serde_derive::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////
/// AccountInfo
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountInfo {
    pub name: Option<String>,
}
