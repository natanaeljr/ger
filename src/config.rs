use crate::util;
use failure::ResultExt;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use termcolor::StandardStream;
use toml;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Verbosity {
    Normal,
    Verbose,
    High,
    Debug,
}

impl From<u64> for Verbosity {
    fn from(val: u64) -> Self {
        match val {
            0 => Verbosity::Normal,
            1 => Verbosity::Verbose,
            2 => Verbosity::High,
            _ => Verbosity::Debug,
        }
    }
}

pub struct CliConfig {
    pub user: UserConfig,
    pub stdout: StandardStream,
}

pub struct UserConfig {
    pub filepath: PathBuf,
    pub settings: UserSettings,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct UserSettings {
    default_remote: Option<String>,
    pub remotes: BTreeMap<String, RemoteOpts>,
}

impl UserConfig {
    /// Read user config from TOML config file
    pub fn from_file(config_file: Option<String>) -> Result<Self, failure::Error> {
        let default_config_file =
            format!("{}/.ger.toml", dirs::home_dir().unwrap().to_str().unwrap());
        let config_file = config_file.unwrap_or(default_config_file);
        let contents = std::fs::read_to_string(&config_file)?;
        let settings: UserSettings = toml::from_str(contents.as_str())
            .with_context(|_| format!("failed to parse config file: {}", config_file))?;
        Ok(UserConfig {
            filepath: config_file.into(),
            settings,
        })
    }

    /// Write user config to filepath
    pub fn store(&self) -> Result<(), failure::Error> {
        let toml = toml::to_string_pretty(&self.settings)?;
        std::fs::write(&self.filepath, toml)?;
        Ok(())
    }
}

impl UserSettings {
    /// Get default remote from config or figure out one
    pub fn default_remote(&self) -> Option<&str> {
        if let Some(default) = &self.default_remote {
            Some(default.as_str())
        } else if self.remotes.len() == 1 {
            Some(self.remotes.keys().next().unwrap().as_str())
        } else {
            None
        }
    }

    /// Get default remote from config or figure out one,
    /// plus verify that the remote is exists in the map
    pub fn default_remote_verify(&self) -> Option<&str> {
        self.default_remote().and_then(|default| {
            if self.remotes.contains_key(default) {
                Some(default)
            } else {
                None
            }
        })
    }

    /// Set default remote value
    pub fn set_default_remote(&mut self, default: Option<String>) {
        self.default_remote = default;
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(default, deny_unknown_fields)]
pub struct RemoteOpts {
    pub url: String,
    pub username: String,
    pub http_password: String,
    pub http_auth: HttpAuthMethod,
    #[serde(skip_serializing_if = "util::is_false")]
    pub no_ssl_verify: bool,
}

impl Default for RemoteOpts {
    fn default() -> Self {
        Self {
            url: Default::default(),
            username: Default::default(),
            http_password: Default::default(),
            http_auth: HttpAuthMethod::Basic,
            no_ssl_verify: false,
        }
    }
}

/// HTTP Authentication Methods.
#[derive(EnumString, Display, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum HttpAuthMethod {
    /// Basic HTTP authentication scheme.
    Basic,
    /// Digest HTTP authentication scheme.
    Digest,
}

impl From<gerlib::HttpAuthMethod> for HttpAuthMethod {
    fn from(auth: gerlib::HttpAuthMethod) -> Self {
        match auth {
            gerlib::HttpAuthMethod::Basic => HttpAuthMethod::Basic,
            gerlib::HttpAuthMethod::Digest => HttpAuthMethod::Digest,
        }
    }
}

impl Into<gerlib::HttpAuthMethod> for HttpAuthMethod {
    fn into(self) -> gerlib::HttpAuthMethod {
        match self {
            HttpAuthMethod::Basic => gerlib::HttpAuthMethod::Basic,
            HttpAuthMethod::Digest => gerlib::HttpAuthMethod::Digest,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// TESTS
#[cfg(test)]
mod tests {
    use failure::_core::iter::FromFn;
    use std::collections::{BTreeMap, HashMap};
    use std::io::Write;

    #[test]
    fn config_file_two_remotes() {
        let expected = super::UserConfig {
            remotes: {
                let mut remotes = BTreeMap::new();
                remotes.insert(
                    "alpha".to_owned(),
                    super::RemoteOpts {
                        url: "https://gerrit-review.googlesource.com".to_owned(),
                        port: None,
                        username: Some("stickman".to_owned()),
                        http_password: Some("somelongstring4685+&%".to_owned()),
                    },
                );
                remotes.insert(
                    "beta".to_owned(),
                    super::RemoteOpts {
                        url: "http://gerrit-review.example.com".to_owned(),
                        port: Some(8080),
                        username: Some("wonderwoman".to_owned()),
                        http_password: Some("+&%anotherlongstring4685".to_owned()),
                    },
                );
                remotes
            },
            default_remote: Some("beta".to_owned()),
        };

        let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
        tmp_file
            .write_all(
                b"\
default_remote = \"beta\"

[remotes.alpha]
url = \"https://gerrit-review.googlesource.com\"
username = \"stickman\"
http_password = \"somelongstring4685+&%\"

[remotes.beta]
url = \"http://gerrit-review.example.com\"
port = 8080
http_password = \"+&%anotherlongstring4685\"
username = \"wonderwoman\"
        ",
            )
            .unwrap();
        let tmp_filename = tmp_file.path().to_str().unwrap();
        let actual = super::UserConfig::from_file(Some(tmp_filename)).unwrap();

        assert_eq!(expected, actual);
    }
}
