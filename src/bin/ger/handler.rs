use crate::config::CliConfig;
use gerlib::rest::rest::RestApiHandler;

pub fn get_remote_restapi_handler(
    config: &CliConfig, remote: Option<&str>,
) -> Result<RestApiHandler, failure::Error> {
    let remote = if let Some(this) = remote {
        this
    } else {
        match config.user.settings.default_remote_verify() {
            Some(default) => default,
            None => return Err(failure::err_msg("no default remote")),
        }
    };

    let _remote = match config.user.settings.remotes.get(remote) {
        Some(r) => r,
        None => return Err(failure::err_msg(format!("no such remote: {}", remote))),
    };

    let handler = RestApiHandler::new()?;
    Ok(handler)
}
