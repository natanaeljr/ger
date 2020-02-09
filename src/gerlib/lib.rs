#![allow(dead_code)]

use std::borrow::Cow;

pub mod accounts;
pub mod changes;
pub mod details;
pub mod http;
pub mod projects;
pub mod rest;

///////////////////////////////////////////////////////////////////////////////////////////////////
pub struct GerritConn<'a> {
    pub host: Cow<'a, String>,
    pub username: Cow<'a, String>,
    pub http_password: Cow<'a, String>,
    pub no_ssl_verify: bool,
}
