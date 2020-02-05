use crate::config::CliConfig;
use gerlib::rest::RestRequestHandler;
use std::borrow::Cow;

pub fn get_remote_restapi_handler(
    config: &CliConfig, remote: Option<&str>,
) -> Result<RestRequestHandler, failure::Error> {
    let remote = if let Some(this) = remote {
        this
    } else {
        match config.user.settings.default_remote_verify() {
            Some(default) => default,
            None => return Err(failure::err_msg("no default remote")),
        }
    };

    let remote = match config.user.settings.remotes.get(remote) {
        Some(r) => r,
        None => return Err(failure::err_msg(format!("no such remote: {}", remote))),
    };

    let gerrit = gerlib::GerritConn {
        host: Cow::Borrowed(&remote.url),
        username: Cow::Borrowed(&remote.username),
        http_password: Cow::Borrowed(&remote.http_password),
        no_ssl_verify: remote.no_ssl_verify,
    };

    let handler = RestRequestHandler::new(gerrit)?;
    Ok(handler)
}
