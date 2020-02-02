#![allow(dead_code)]

pub mod accounts;
pub mod changes;
pub mod details;
pub mod http;
pub mod rest;

///////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Gerrit {
    pub host: String,
    pub port: Option<u16>,
    pub username: String,
    pub http_password: String,
    pub insecure: bool,
}
