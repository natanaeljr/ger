use crate::config::CliConfig;
use crate::util;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::io::Write;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("change")
        .about("Lists information about changes.")
        .arg(
            Arg::with_name("CHANGE")
                .required(false)
                .multiple(true)
                .help(
                    "Specify changes to look for.\n\
                     Can be either a legacy numerical id (e.g. 15813),\
                     full or abbreviated Change-Id (e.g. Ic0ff33)\
                     or commit SHA-1 (e.g. d81b32ef).",
                ),
        )
        .arg(
            Arg::with_name("max-count")
                .short("n")
                .takes_value(true)
                .value_name("NUMBER")
                .default_value("20")
                .validator(util::validate::is_u32)
                .help("Limit the number of changes to output."),
        )
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
}

pub fn exec(config: &mut CliConfig, _args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let remote = match config.user_cfg.settings.default_remote_verify() {
        Some(r) => config.user_cfg.settings.remotes.get(r).unwrap(),
        None => return Err(failure::err_msg("no remote specified")),
    };

    let mut http_handler = gerlib::http::HttpRequestHandler::new(gerlib::Gerrit {
        host: remote.url.clone(),
        username: remote.username.as_ref().unwrap().clone(),
        http_password: remote.http_password.as_ref().unwrap().clone(),
        insecure: remote.insecure,
    })?;

    let data = http_handler.get("a/changes/?n=2")?;
    writeln!(config.stdout, "response: {}", data)?;

    Ok(())
}
