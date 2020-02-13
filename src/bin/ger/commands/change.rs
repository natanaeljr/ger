use crate::config::{CliConfig, Verbosity};
use crate::handler::get_remote_restapi_handler;
use crate::util;
use clap::{App, Arg, ArgMatches, SubCommand};
use gerlib::changes::{ChangeInfo, ChangeIs, ChangeOptions, Query, QueryOpt};
use http::uri::PathAndQuery;
use log::info;
use std::io::Write;
use termcolor::{Color, ColorSpec, WriteColor};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("change")
        .about("Lists changes and information about changes.")
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
                .value_name("limit")
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
    let max_count = args.value_of("max-count");

    let mut rest = get_remote_restapi_handler(config, remote)?;

    let queries = ChangeOptions {
        queries: vec![Query(QueryOpt::Is(ChangeIs::Open))],
        additional_opts: vec![],
        limit: max_count.map(|n| n.parse::<u32>().unwrap()),
        start: None,
    };

    let query_str = queries.to_query_string();
    let uri: PathAndQuery = format!("/a/changes/{}", query_str).parse()?;
    info!("uri: {}", uri);
    let json = rest.request_json(uri, verbose >= Verbosity::Debug)?;
    let changes: Vec<ChangeInfo> = serde_json::from_str(json.as_str())?;

    show_list(config, &changes)?;

    Ok(())
}

pub fn show_list(config: &mut CliConfig, changes: &Vec<ChangeInfo>) -> Result<(), failure::Error> {
    if changes.is_empty() {
        writeln!(config.stdout, "No changes.")?;
        return Ok(());
    }

    let mut stdout = config.stdout.lock();
    for change in changes {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        write!(stdout, "{}", change.number)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        write!(stdout, " {}", change.project)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
        write!(stdout, " {}", change.status)?;

        stdout.reset()?;
        write!(stdout, " {}", change.subject)?;
        stdout.write_all(b"\n")?;
    }

    Ok(())
}
