use super::list;
use crate::config::{CliConfig, Verbosity};
use crate::handler::get_remote_restapi_handler;
use clap::{App, Arg, ArgMatches, SubCommand};
use http::uri::PathAndQuery;
use log::info;
use std::io::Write;
use termcolor::{ColorSpec, WriteColor};
use gerlib::rest::changes::ChangeInfo;

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

    let uri: PathAndQuery =
        "/a/changes/?q=is:open+owner:self&q=is:open+reviewer:self+-owner:self&q=is:closed+(owner:self+OR+reviewer:self)+limit:10&o=DETAILED_ACCOUNTS&o=CURRENT_REVISION".to_string()
        .parse()?;

    info!("uri: {}", uri);
    let json = rest.get_json(uri, verbose >= Verbosity::Debug)?;
    let changes_vec: Vec<Vec<ChangeInfo>> = serde_json::from_str(json.as_str())?;

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
