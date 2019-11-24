pub mod accounts;
pub mod changes;
pub mod details;
pub mod error;

use error::Error;

type Result<T> = std::result::Result<T, Error>;

///////////////////////////////////////////////////////////////////////////////////////////////////
/// Gerrit Interface
#[derive(Default, Debug)]
pub struct Gerrit {
    host: String,
    username: String,
    password: String,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
impl Gerrit {
    /// Creates a new Gerrit object which is the core for interfacing with a Gerrit server.
    pub fn new<S: Into<String>>(host: S) -> Self {
        Gerrit {
            host: host.into(),
            ..Default::default()
        }
    }

    /// Configures the username to pass authentication for this connection.
    pub fn username<S: Into<String>>(mut self, username: S) -> Self {
        self.username = username.into();
        self
    }

    /// Configures the password to pass authentication for this connection.
    pub fn password<S: Into<String>>(mut self, password: S) -> Self {
        self.password = password.into();
        self
    }

    /// Get list of changes from Gerrit server
    pub fn get_changes(&self, max_count: u32) -> Result<Vec<changes::ChangeInfo>> {
        let json: String = self.request_json(format!("changes/?n={}", max_count))?;
        let changes: Vec<changes::ChangeInfo> = serde_json::from_str(json.as_str())?;
        Ok(changes)
    }

    // Perform a request to the REST API through libcurl
    pub fn request_json<S: AsRef<str>>(&self, query_str: S) -> Result<String> {
        let mut easy = curl::easy::Easy::new();
        easy.url(
            format!(
                "{}{}",
                "https://gerrit-review.googlesource.com/a/",
                query_str.as_ref()
            )
            .as_str(),
        )?;
        easy.username(self.username.as_str())?;
        easy.password(self.password.as_str())?;
        let mut headers = curl::easy::List::new();
        headers.append("Accept: application/json")?;
        easy.http_headers(headers)?;
        let mut data: Vec<u8> = Vec::new();
        {
            let mut transfer = easy.transfer();
            transfer.write_function(|input| {
                data.extend_from_slice(input);
                Ok(input.len())
            })?;
            transfer.perform()?
        }
        let response = String::from_utf8_lossy(data.as_slice()).to_string();
        let res_code = easy.response_code()?;
        if res_code != 200 {
            return Err(Error::Generic(response));
        }
        const MAGIC_PREFIX: &'static str = ")]}'\n";
        if !response.starts_with(MAGIC_PREFIX) {
            return Err(Error::Generic(format!(
                "Response is not JSON!\n{}",
                response
            )));
        }
        let json: &str = &response[MAGIC_PREFIX.len()..];
        Ok(json.to_string())
    }
}
