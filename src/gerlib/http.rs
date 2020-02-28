use super::GerritConn;
use crate::HttpAuthMethod;
use curl::easy::Easy as CurlEasy;
use log::{debug, trace};
use std::io::Read;
use url::Url;

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
        curl.username(gerrit.username.as_str())?;
        curl.password(gerrit.http_password.as_str())?;
        let mut http_auth = curl::easy::Auth::new();
        match gerrit.http_auth {
            HttpAuthMethod::Basic => http_auth.basic(true),
            HttpAuthMethod::Digest => http_auth.digest(true),
        };
        curl.http_auth(&http_auth)?;
        if gerrit.no_ssl_verify {
            curl.ssl_verify_host(false)?;
            curl.ssl_verify_peer(false)?;
        }
        curl.follow_location(true)?;
        curl.verbose(log::max_level() >= log::LevelFilter::Debug)?;

        Ok(Self {
            host: gerrit.host.into_owned(),
            curl,
        })
    }

    /// Make a GET request to URI
    pub fn get(&mut self, uri: &str) -> Result<(u32, String), failure::Error> {
        let url = Url::parse(self.host.as_str())?.join(uri)?;
        debug!("get url: {}", url.as_str());
        self.curl.url(url.as_str())?;
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
        let code = self.curl.response_code()?;
        let response = String::from_utf8_lossy(data.as_slice()).into_owned();
        Ok((code, response))
    }

    pub fn post(&mut self, uri: &str, data: &[u8]) -> Result<(u32, String), failure::Error> {
        let url = Url::parse(self.host.as_str())?.join(uri)?;
        debug!("post url: {}", url.as_str());
        self.curl.url(url.as_str())?;
        self.curl.post(true)?;
        self.curl.post_field_size(data.len() as u64)?;
        let mut headers = curl::easy::List::new();
        headers.append("Content-Type: application/json, charset=UTF-8")?;
        self.curl.http_headers(headers)?;
        let mut output: Vec<u8> = Vec::new();
        {
            let mut data_mut = data;
            let mut transfer = self.curl.transfer();
            transfer.read_function(|into| Ok(data_mut.read(into).unwrap()))?;
            transfer.write_function(|new_data| {
                output.extend_from_slice(new_data);
                Ok(new_data.len())
            })?;
            transfer.debug_function(Self::curl_debug_function)?;
            transfer.perform()?;
        }
        let code = self.curl.response_code()?;
        let response = String::from_utf8_lossy(output.as_slice()).into_owned();
        Ok((code, response))
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
