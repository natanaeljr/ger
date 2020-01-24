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
    pub user_cfg: UserConfig,
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
    pub remotes: BTreeMap<String, Remote>,
}

impl UserConfig {
    /// Read user config from TOML config file
    pub fn from_file(config_file: Option<&str>) -> Result<Self, std::io::Error> {
        let default_config_file =
            format!("{}/.ger.toml", dirs::home_dir().unwrap().to_str().unwrap());
        let config_file = config_file.unwrap_or(default_config_file.as_str());
        let contents = std::fs::read_to_string(config_file)?;
        let settings: UserSettings = toml::from_str(contents.as_str()).unwrap();
        Ok(UserConfig {
            filepath: config_file.into(),
            settings,
        })
    }

    /// Write user config to filepath
    pub fn store(&self) -> Result<(), std::io::Error> {
        let toml = toml::to_string_pretty(&self.settings).unwrap();
        std::fs::write(&self.filepath, toml)
    }
}

impl UserSettings {
    /// Get default remote or figure out one
    pub fn default_remote(&self) -> Option<&str> {
        if let Some(default) = &self.default_remote {
            Some(default.as_str())
        } else if self.remotes.len() == 1 {
            Some(self.remotes.keys().next().unwrap().as_str())
        } else {
            None
        }
    }

    /// Set default remote value
    pub fn set_default_remote(&mut self, default: Option<String>) {
        self.default_remote = default;
    }
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Debug)]
pub struct Remote {
    pub url: String,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub http_password: Option<String>,
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
                    super::Remote {
                        url: "https://gerrit-review.googlesource.com".to_owned(),
                        port: None,
                        username: Some("stickman".to_owned()),
                        http_password: Some("somelongstring4685+&%".to_owned()),
                    },
                );
                remotes.insert(
                    "beta".to_owned(),
                    super::Remote {
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
