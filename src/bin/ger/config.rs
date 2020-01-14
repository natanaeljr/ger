use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use termcolor::StandardStream;
use toml;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum Verbosity {
    Normal,
    Verbose,
    High,
    Debug,
}

pub struct CliConfig {
    pub user_cfg: UserConfig,
    pub stdout: StandardStream,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct UserConfig {
    pub default_remote: Option<String>,
    pub remotes: HashMap<String, Remote>,
}

impl UserConfig {
    /// Read user config from TOML config file
    pub fn from_file(config_file: Option<&str>) -> Result<Self, std::io::Error> {
        let default_config_file =
            format!("{}/.ger.toml", dirs::home_dir().unwrap().to_str().unwrap());
        let config_file = config_file.unwrap_or(default_config_file.as_str());
        let contents = std::fs::read_to_string(config_file)?;
        let config: Self = toml::from_str(contents.as_str()).unwrap();
        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Debug)]
pub struct Remote {
    pub url: String,
    pub port: Option<u16>,
    pub username: String,
    pub http_password: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// TESTS
#[cfg(test)]
mod tests {
    use failure::_core::iter::FromFn;
    use std::collections::HashMap;
    use std::io::Write;

    #[test]
    fn config_file_two_remotes() {
        let expected = super::UserConfig {
            remotes: {
                let mut remotes = HashMap::new();
                remotes.insert(
                    "alpha".to_owned(),
                    super::Remote {
                        url: "https://gerrit-review.googlesource.com".to_owned(),
                        port: None,
                        username: "stickman".to_owned(),
                        http_password: "somelongstring4685+&%".to_owned(),
                    },
                );
                remotes.insert(
                    "beta".to_owned(),
                    super::Remote {
                        url: "http://gerrit-review.example.com".to_owned(),
                        port: Some(8080),
                        username: "wonderwoman".to_owned(),
                        http_password: "+&%anotherlongstring4685".to_owned(),
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
        let actual = super::UserConfig::from_file(tmp_filename).unwrap();

        assert_eq!(expected, actual);
    }
}
