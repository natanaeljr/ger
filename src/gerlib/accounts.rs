use serde_derive::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////
/// JSON Entities
////////////////////////////////////////////////////////////////////////////////////////////////////

/// The AccountInfo entity contains information about an account.
#[derive(Debug, Serialize, Deserialize)]
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

/// The AccountInput entity contains information for the creation of a new account.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInput {
    /// The user name. If provided, must match the user name from the URL.
    pub username: Option<String>,
    /// The full name of the user.
    pub name: Option<String>,
    /// The display name of the user.
    pub display_name: Option<String>,
    /// The email address of the user.
    pub email: Option<String>,
    /// The public SSH key of the user.
    pub ssh_key: Option<String>,
    /// The HTTP password of the user.
    pub http_password: Option<String>,
    /// A list of group IDs that identify the groups to which the user should be added.
    pub groups: Option<Vec<u32>>,
}

/// The AccountInfo entity contains information about an avatar image of an account.
#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarInfo {
    /// The URL to the avatar image.
    pub url: String,
    /// The height of the avatar image in pixels.
    pub height: Option<u32>,
    /// The width of the avatar image in pixels.
    pub width: Option<u32>,
}

/// The GpgKeyInfo entity contains information about a GPG public key.
#[derive(Debug, Serialize, Deserialize)]
pub struct GpgKeyInfo {
    /// The 8-char hex GPG key ID.
    /// Not set in map context
    pub id: Option<String>,
    /// The 40-char (plus spaces) hex GPG key fingerprint.
    /// Not set for deleted keys
    pub fingerprint: Option<String>,
    /// OpenPGP User IDs associated with the public key.
    /// Not set for deleted keys
    pub user_ids: Option<String>,
    /// ASCII armored public key material.
    /// Not set for deleted keys
    pub key: Option<String>,
    /// The result of server-side checks on the key; one of BAD, OK, or TRUSTED.
    /// Not set for deleted keys
    pub status: Option<KeyStatus>,
    /// A list of human-readable problem strings found in the course of checking whether the key is
    /// valid and trusted.
    /// Not set for deleted keys
    pub problems: Option<Vec<String>>,
}

/// Key check status.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KeyStatus {
    /// If a key is OK, inspecting only that key found no problems,
    Ok,
    /// BAD keys have serious problems and should not be used, but the system does not fully trust the key’s origin.
    Bad,
    /// A TRUSTED key is valid, and the system knows enough about the key and its origin to trust it.
    Trusted,
}
