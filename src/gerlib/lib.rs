#![allow(dead_code)]

pub mod accounts;
pub mod changes;
pub mod details;
pub mod http;

///////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Gerrit {
    pub host: String,
    pub username: String,
    pub http_password: String,
    pub insecure: bool,
}
