use crate::config::{CliConfig, Verbosity};
use crate::handler::get_remote_restapi_handler;
use clap::{App, Arg, ArgMatches, SubCommand};
use gerlib::changes::ChangeInfo;
use http::uri::PathAndQuery;
use log::info;
use std::io::Write;
use termcolor::{Color, ColorSpec, WriteColor};

/// Build the CLI
pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("show")
        .about("Show information about changes.")
        .arg(Arg::with_name("change").required(true).help(
            "Change identifier.\n\
             Can be either a legacy numerical id (e.g. 15813),\
             full or abbreviated Change-Id (e.g. Ic0ff33)\
             or commit SHA-1 (e.g. d81b32ef).",
        ))
        .arg(
            Arg::with_name("remote")
                .long("remote")
                .short("r")
                .takes_value(true)
                .value_name("name")
                .help("Specify an alternative remote to use."),
        )
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
}

/// Execute the command
pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    let verbose: Verbosity = args.occurrences_of("verbose").into();
    let remote = args.value_of("remote");
    let change_id = args.value_of("change").unwrap();

    let mut rest = get_remote_restapi_handler(config, remote)?;

    let uri: PathAndQuery = format!(
        "/a/changes/{}/?o=CURRENT_REVISION&o=CURRENT_COMMIT&o=DETAILED_ACCOUNTS",
        change_id
    )
    .parse()?;

    info!("uri: {}", uri);
    let json = rest.request_json(uri, verbose >= Verbosity::Debug)?;
    let change: ChangeInfo = serde_json::from_str(json.as_str())?;

    show(config, &change)?;

    Ok(())
}

pub fn show(config: &mut CliConfig, change: &ChangeInfo) -> Result<(), failure::Error> {
    let mut stdout = config.stdout.lock();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    writeln!(stdout, "Change {}", change.number)?;

    if let Some(owner_name) = &change.owner.name {
        stdout.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Black))
                .set_intense(true),
        )?;
        writeln!(stdout, "Owner: {}", owner_name)?;
    }

    stdout.reset()?;
    writeln!(
        stdout,
        "Commit: {}",
        change.current_revision.as_ref().unwrap()
    )?;

    let lines = change
        .revisions
        .as_ref()
        .unwrap()
        .get(change.current_revision.as_ref().unwrap())
        .unwrap()
        .commit
        .as_ref()
        .unwrap()
        .message
        .lines();

    stdout.reset()?;
    stdout.write_all(b"\n")?;

    for line in lines {
        writeln!(stdout, "    {}", line)?;
    }

    Ok(())
}
