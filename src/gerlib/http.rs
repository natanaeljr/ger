use super::GerritConn;
use curl::easy::Easy as CurlEasy;
use log::{debug, trace};

/// Handler for make request to Gerrit REST API
pub struct HttpRequestHandler {
    host: String,
    curl: CurlEasy,
}

impl HttpRequestHandler {
    /// Create a new RequestHandler
    pub fn new(gerrit: GerritConn) -> Result<Self, failure::Error> {
        trace!("curl version: {}", curl::Version::get().version());
        let mut curl = CurlEasy::new();
        curl.url(gerrit.host.as_str())?;
        curl.http_auth(curl::easy::Auth::new().basic(true).digest(true))?;
        curl.username(gerrit.username.as_str())?;
        curl.password(gerrit.http_password.as_str())?;
        if gerrit.no_ssl_verify {
            curl.ssl_verify_host(false)?;
            curl.ssl_verify_peer(false)?;
        }
        curl.follow_location(true)?;

        Ok(Self {
            host: gerrit.host.into_owned(),
            curl,
        })
    }

    /// Make a GET request to URI
    pub fn get(&mut self, uri: &str) -> Result<String, failure::Error> {
        let url = format!("{}/{}", self.host, uri);
        debug!("get url: {}", url);
        self.curl.url(url.as_str())?;
        self.curl
            .verbose(log::max_level() >= log::LevelFilter::Debug)?;
        let mut data: Vec<u8> = Vec::new();
        {
            let mut transfer = self.curl.transfer();
            transfer.write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })?;
            transfer.debug_function(Self::curl_debug_function)?;
            transfer.perform()?;
        }
        Ok(String::from_utf8_lossy(data.as_slice()).into_owned())
    }

    /// Debug function for CURL
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
