use serde_derive::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////
/// The AccountInfo entity contains information about an account.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountInfo {
    /// The numeric ID of the account.
    #[serde(rename = "_account_id")]
    pub account_id: u32,
    /// The full name of the user.
    /// Only set if detailed account information is requested.
    pub name: Option<String>,
    /// The email address the user prefers to be contacted through.
    /// Only set if detailed account information is requested.
    pub email: Option<String>,
    /// The username of the user.
    /// Only set if detailed account information is requested.
    pub username: Option<String>,
}
