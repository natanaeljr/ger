#![allow(dead_code)]

use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub mod accounts;
pub mod changes;
pub mod details;
pub mod http;
pub mod projects;
pub mod rest;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HttpAuthMethod {
    Basic,
    Digest,
}

impl Display for HttpAuthMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            HttpAuthMethod::Basic => "basic",
            HttpAuthMethod::Digest => "digest",
        })
    }
}

impl FromStr for HttpAuthMethod {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "basic" => Ok(HttpAuthMethod::Basic),
            "digest" => Ok(HttpAuthMethod::Digest),
            &_ => Err(failure::err_msg("invalid http auth method")),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
pub struct GerritConn<'a> {
    pub host: Cow<'a, String>,
    pub username: Cow<'a, String>,
    pub http_password: Cow<'a, String>,
    pub http_auth: HttpAuthMethod,
    pub no_ssl_verify: bool,
}
