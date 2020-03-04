use curl::easy::Easy as CurlEasy;
use log::{debug, trace};
use std::fmt::Display;
use std::io::Read;
use url::Url;

type Result<T> = std::result::Result<T, Error>;

/// HTTP Request Handler is a wrapper around the libcurl Easy handler
/// to provide common use functions for a REST API Client.
#[derive(Debug)]
pub struct HttpRequestHandler {
    curl: CurlEasy,
    base_url: Url,
}

/// HTTP Request Handler errors.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// CURL operation errors
    Curl(curl::Error),
    /// Wrong URL format
    Url(url::ParseError),
}

/// HTTP Authentication Methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthMethod {
    /// Basic HTTP authentication scheme.
    Basic,
    /// Digest HTTP authentication scheme.
    Digest,
}

/// Common HTTP Headers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Header {
    /// "Content-Type: application/json"
    ContentTypeAppJson,
    /// "Accept: application/json"
    AcceptAppJson,
    /// Any other header
    Custom(String),
}

impl HttpRequestHandler {
    /// Create a new HTTP Request Handler object.
    pub fn new(base_url: Url, username: &str, password: &str) -> Result<Self> {
        trace!("curl version: {}", curl::Version::get().version());
        let mut curl = CurlEasy::new();
        curl.username(username)?;
        curl.password(password)?;
        curl.follow_location(true)?;
        curl.verbose(log::max_level() >= log::LevelFilter::Debug)?;
        Ok(Self { curl, base_url })
    }

    /// Specify the HTTP authentication method.
    pub fn http_auth(mut self, auth: &AuthMethod) -> Result<Self> {
        let mut http_auth = curl::easy::Auth::new();
        match auth {
            AuthMethod::Basic => http_auth.basic(true),
            AuthMethod::Digest => http_auth.digest(true),
        };
        self.curl.http_auth(&http_auth)?;
        Ok(self)
    }

    /// Enable/Disable SSL verification of both host and peer.
    pub fn ssl_verify(mut self, enable: bool) -> Result<Self> {
        self.curl.ssl_verify_host(enable)?;
        self.curl.ssl_verify_peer(enable)?;
        Ok(self)
    }

    /// Set HTTP headers.
    pub fn headers(&mut self, in_headers: &[Header]) -> Result<&mut Self> {
        let mut headers = curl::easy::List::new();
        for header in in_headers {
            headers.append(header.to_string().as_str())?;
        }
        self.curl.http_headers(headers)?;
        Ok(self)
    }

    /// Perform a GET request.
    pub fn get(&mut self, path_and_query: &str) -> Result<(u32, String)> {
        self.curl.get(true)?;
        self.perform_request(path_and_query, None)
    }

    /// Perform a PUT request.
    pub fn put(&mut self, path_and_query: &str, tx_data: Option<&[u8]>) -> Result<(u32, String)> {
        self.curl.put(true)?;
        self.perform_request(path_and_query, tx_data)
    }

    /// Perform a POST request.
    pub fn post(&mut self, path_and_query: &str, tx_data: Option<&[u8]>) -> Result<(u32, String)> {
        self.curl.post(true)?;
        self.perform_request(path_and_query, tx_data)
    }

    /// Perform a DELETE request.
    pub fn delete(&mut self, path_and_query: &str) -> Result<(u32, String)> {
        self.curl.custom_request("DELETE")?;
        self.perform_request(path_and_query, None)
    }

    /// Perform a generic HTTP Request and return the code with received response body.
    fn perform_request(
        &mut self, path_and_query: &str, tx_data: Option<&[u8]>,
    ) -> Result<(u32, String)> {
        let url = self.base_url.join(path_and_query)?;
        self.curl.url(url.as_str())?;
        let rx_data = self.perform_transfer(tx_data)?;
        let code = self.curl.response_code()?;
        let response = String::from_utf8_lossy(rx_data.as_slice()).into_owned();
        Ok((code, response))
    }

    /// Perform CURL transfer and return the response body.
    fn perform_transfer(&mut self, tx_data: Option<&[u8]>) -> Result<Vec<u8>> {
        if let Some(tx_data) = tx_data {
            self.curl.post_field_size(tx_data.len() as u64)?;
        }
        let mut tx_data_mut = tx_data.unwrap_or(b"");
        let mut rx_data: Vec<u8> = Vec::new();
        {
            let mut transfer = self.curl.transfer();
            if tx_data.is_some() {
                transfer.read_function(|into| Ok(tx_data_mut.read(into).unwrap()))?;
            }
            transfer.write_function(|new_data| {
                rx_data.extend_from_slice(new_data);
                Ok(new_data.len())
            })?;
            transfer.debug_function(Self::curl_debug_function)?;
            transfer.perform()?;
        }
        Ok(rx_data)
    }

    /// Debug function for CURL.
    fn curl_debug_function(info_type: curl::easy::InfoType, data: &[u8]) {
        use curl::easy::InfoType;
        match info_type {
            InfoType::Text => debug!("curl:* {}", String::from_utf8_lossy(data).trim_end()),
            InfoType::HeaderIn => debug!("curl:< {}", String::from_utf8_lossy(data).trim_end()),
            InfoType::HeaderOut => debug!("curl:> {}", String::from_utf8_lossy(data).trim_end()),
            InfoType::SslDataIn => trace!("curl: SslDataIn (binary omitted)"),
            InfoType::SslDataOut => trace!("curl: SslDataOut (binary omitted)"),
            _ => debug!("curl: {}", String::from_utf8_lossy(data).trim_end()),
        };
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Header::ContentTypeAppJson => "Content-Type: application/json",
            Header::AcceptAppJson => "Accept: application/json",
            Header::Custom(ref s) => s.as_str(),
        })
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Error::Curl(_) => "LibCURL returned error",
            Error::Url(_) => "Invalid URL",
        })
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Curl(ref e) => Some(e),
            Error::Url(ref e) => Some(e),
        }
    }
}

impl From<curl::Error> for Error {
    fn from(e: curl::Error) -> Self {
        Error::Curl(e)
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::Url(e)
    }
}
