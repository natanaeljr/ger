use crate::config::CliConfig;
use gerlib::rest::GerritRestApi;

pub fn get_remote_restapi_handler(
    config: &CliConfig, remote: Option<&str>,
) -> Result<GerritRestApi, failure::Error> {
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

    let handler = GerritRestApi::new(remote.url.parse()?, &remote.username, &remote.http_password)?
        .http_auth(&remote.http_auth.into())?
        .ssl_verify(!remote.no_ssl_verify)?;
    Ok(handler)
}
