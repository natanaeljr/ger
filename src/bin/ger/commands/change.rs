use crate::config::{CliConfig, Verbosity};
use crate::util;
use clap::{App, Arg, ArgMatches, SubCommand};
use gerlib::changes::ChangeInfo;
use gerlib::rest::RestRequestHandler;
use http::uri::PathAndQuery;
use std::borrow::Cow;
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

pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    let verbose: Verbosity = args.occurrences_of("verbose").into();

    let remote = match config.user_cfg.settings.default_remote_verify() {
        Some(default) => config.user_cfg.settings.remotes.get(default).unwrap(),
        None => return Err(failure::err_msg("no default remote")),
    };

    let gerrit = gerlib::GerritConn {
        host: Cow::Borrowed(&remote.url),
        username: Cow::Borrowed(&remote.username),
        http_password: Cow::Borrowed(&remote.http_password),
        no_ssl_verify: remote.no_ssl_verify,
    };

    let mut rest = RestRequestHandler::new(gerrit)?;
    let uri: PathAndQuery = "/a/changes/?q=owner:self+is:open".parse()?;

    let json = rest.request_json(uri, verbose >= Verbosity::Debug)?;

    let changes: Vec<ChangeInfo> = serde_json::from_str(json.as_str())?;
    for change in changes {
        writeln!(config.stdout, "{} - {}", change._number, change.subject)?;
    }

    Ok(())
}
