use serde_derive::{Deserialize, Serialize};

/// The AccountInfo entity contains information about an account.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountInfo {
    /// The numeric ID of the account.
    #[serde(rename = "_account_id")]
    pub account_id: u32,
    /// The full name of the user.
    /// Only set if detailed account information is requested.
    pub name: Option<String>,
    /// The display name of the user.
    /// Only set if detailed account information is requested.
    pub display_name: Option<String>,
    /// The email address the user prefers to be contacted through.
    /// Only set if detailed account information is requested.
    pub email: Option<String>,
    /// A list of the secondary email addresses of the user.
    /// Only set for account queries when the ALL_EMAILS option or the suggest parameter is set.
    /// Secondary emails are only included if the calling user has the Modify Account, and hence is
    /// allowed to see secondary emails of other users.
    pub secondary_emails: Option<Vec<String>>,
    /// The username of the user.
    /// Only set if detailed account information is requested.
    pub username: Option<String>,
    /// List of AvatarInfo entities that provide information about avatar images of the account.
    pub avatars: Option<Vec<AvatarInfo>>,
    /// Whether the query would deliver more results if not limited.
    /// Only set on the last account that is returned.
    #[serde(default, rename = "_more_accounts")]
    pub more_accounts: bool,
    /// Status message of the account.
    pub status: Option<String>,
    /// Whether the account is inactive.
    #[serde(default)]
    pub inactive: bool,
}

/// The AccountInfo entity contains information about an avatar image of an account.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AvatarInfo {
    /// The URL to the avatar image.
    pub url: String,
    /// The height of the avatar image in pixels.
    pub height: Option<u32>,
    /// The width of the avatar image in pixels.
    pub width: Option<u32>,
}
