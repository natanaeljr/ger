use std::io::Write;

use clap::{App, Arg, ArgMatches, SubCommand};
use http::uri::PathAndQuery;
use log::info;
use termcolor::{ColorSpec, WriteColor};

use gerlib::changes::{AdditionalOpt, ChangeInfo, QueryParams, QueryStr};

use crate::config::{CliConfig, Verbosity};
use crate::handler::get_remote_restapi_handler;

use super::list;

/// Build the CLI
pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("dashboard")
        .visible_alias("db")
        .about(
            "User's change dashboard.\n\
             It is the default subcommand when none specified.",
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

/// Execute the command
pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    let verbose: Verbosity = args.occurrences_of("verbose").into();
    let remote = args.value_of("remote");

    let mut rest = get_remote_restapi_handler(config, remote)?;
    let query_param = QueryParams {
        search_queries: Some(vec![
            QueryStr::Raw("is:open+owner:self".into()),
            QueryStr::Raw("is:open+reviewer:self+-owner:self".into()),
            QueryStr::Raw("is:closed+(owner:self+OR+reviewer:self)+limit:10".into()),
        ]),
        additional_opts: Some(vec![
            AdditionalOpt::DetailedAccounts,
            AdditionalOpt::CurrentRevision,
        ]),
        limit: None,
        start: None,
    };
    let changes_vec: Vec<Vec<ChangeInfo>> = rest.query_changes(&query_param)?;

    config
        .stdout
        .set_color(ColorSpec::new().set_italic(true).set_bold(true))?;
    writeln!(config.stdout, "* Outgoing reviews:")?;
    config.stdout.reset()?;
    list::list(config, &changes_vec[0])?;

    config
        .stdout
        .set_color(ColorSpec::new().set_italic(true).set_bold(true))?;
    writeln!(config.stdout, "\n* Incoming reviews:")?;
    config.stdout.reset()?;
    list::list(config, &changes_vec[1])?;

    config
        .stdout
        .set_color(ColorSpec::new().set_italic(true).set_bold(true))?;
    writeln!(config.stdout, "\n* Recently closed:")?;
    config.stdout.reset()?;
    list::list(config, &changes_vec[2])?;

    Ok(())
}
