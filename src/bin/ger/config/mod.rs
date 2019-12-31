use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use toml;

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Debug)]
pub struct Config {
    pub remotes: HashMap<String, Remote>,
    pub default_remote: Option<String>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Debug)]
pub struct Remote {
    pub url: String,
    pub port: Option<u16>,
    pub username: String,
    pub http_password: String,
}

impl Config {
    pub fn from_file(filepath: &str) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(filepath)?;
        let config: Self = toml::from_str(contents.as_str()).unwrap();
        Ok(config)
    }
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
        let expected = super::Config {
            remotes: {
                let mut remotes = HashMap::new();
                remotes.insert(
                    "alpha".to_owned(),
                    super::Remote {
                        url: "https://gerrit-review.googlesource.com".to_owned(),
                        username: "stickman".to_owned(),
                        http_password: "somelongstring4685+&%".to_owned(),
                    },
                );
                remotes.insert(
                    "beta".to_owned(),
                    super::Remote {
                        url: "http://gerrit-review.example.com".to_owned(),
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
default_remote = \"alpha\"

[remotes.alpha]
url = \"https://gerrit-review.googlesource.com\"
username = \"stickman\"
http_password = \"somelongstring4685+&%\"

[remotes.beta]
url = \"http://gerrit-review.example.com\"
http_password = \"+&%anotherlongstring4685\"
username = \"wonderwoman\"
        ",
            )
            .unwrap();
        let tmp_filename = tmp_file.path().to_str().unwrap();
        let actual = super::Config::from_file(tmp_filename).unwrap();

        assert_eq!(expected, actual);
    }
}
