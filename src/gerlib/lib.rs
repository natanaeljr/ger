#![allow(dead_code)]

extern crate strum;
#[macro_use]
extern crate strum_macros;

use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod accounts;
pub mod changes;
pub mod details;
pub mod http;
pub mod projects;
pub mod rest;
pub mod interface;
pub mod error;

#[derive(EnumString, Display, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum HttpAuthMethod {
    Basic,
    Digest,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
pub struct GerritConn<'a> {
    pub host: Cow<'a, String>,
    pub username: Cow<'a, String>,
    pub http_password: Cow<'a, String>,
    pub http_auth: HttpAuthMethod,
    pub no_ssl_verify: bool,
}
