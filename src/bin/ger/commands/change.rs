use crate::config::{CliConfig, Verbosity};
use crate::handler::get_remote_restapi_handler;
use crate::util;
use clap::{App, Arg, ArgMatches, SubCommand};
use gerlib::changes::ChangeInfo;
use http::uri::PathAndQuery;
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
        .arg(
            Arg::with_name("remote")
                .long("remote")
                .short("r")
                .takes_value(true)
                .value_name("NAME")
                .help("Specify an alternative remote to use."),
        )
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
}

pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    let verbose: Verbosity = args.occurrences_of("verbose").into();
    let remote = args.value_of("remote");

    let mut rest = get_remote_restapi_handler(config, remote)?;
    let uri: PathAndQuery = "/a/changes/?q=is:open&n=10".parse()?;
    let json = rest.request_json(uri, verbose >= Verbosity::Debug)?;
    let changes: Vec<ChangeInfo> = serde_json::from_str(json.as_str())?;
    if changes.is_empty() {
        writeln!(config.stdout, "No changes.")?;
        return Ok(());
    }
    for change in changes {
        writeln!(config.stdout, "{} - {}", change._number, change.subject)?;
    }

    Ok(())
}
