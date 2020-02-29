use crate::config::{CliConfig, Verbosity};
use crate::handler::get_remote_restapi_handler;
use clap::{App, Arg, ArgMatches, SubCommand};
use gerlib::changes::TopicInput;
use http::uri::PathAndQuery;
use log::info;
use std::io::Write;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("topic")
        .about("Get, set or delete the topic of changes.")
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
        .setting(clap::AppSettings::DeriveDisplayOrder)
        .arg(Arg::with_name("change-id").required(true).help(
            "Change identifier. \
             Can be either a legacy numerical id (e.g. 15813), \
             full or abbreviated Change-Id (e.g. Ic0ff33) \
             or commit SHA-1 (e.g. d81b32ef).",
        ))
        .arg(
            Arg::with_name("set")
                .long("set")
                .short("s")
                .takes_value(true)
                .value_name("topic")
                .conflicts_with("delete")
                .help("Set the topic of a change."),
        )
        .arg(
            Arg::with_name("delete")
                .long("delete")
                .short("d")
                .conflicts_with("set")
                .help("Delete the topic of a change."),
        )
        .arg(
            Arg::with_name("remote")
                .long("remote")
                .short("r")
                .takes_value(true)
                .help("Specify an alternative remote to use."),
        )
}

pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    let verbose: Verbosity = args.occurrences_of("verbose").into();
    let remote = args.value_of("remote");
    let change_id = args.value_of("change-id").unwrap();

    let mut rest = get_remote_restapi_handler(config, remote)?;
    let uri: PathAndQuery = format!("/a/changes/{}/topic", change_id).parse()?;
    info!("uri: {}", uri);

    let json_output = if let Some(topic) = args.value_of("set") {
        let topic_input = TopicInput {
            topic: Some(topic.into()),
        };
        let json_input = serde_json::to_string_pretty(&topic_input)?;
        info!("put request, data: {}", json_input);
        Some(rest.put_json(uri, json_input.as_bytes(), verbose >= Verbosity::Verbose)?)
    } else if args.is_present("delete") {
        info!("delete request");
        let res = rest.delete(uri, verbose >= Verbosity::Verbose)?;
        if !res.is_empty() {
            writeln!(config.stdout, "{}", res)?;
        }
        None
    } else {
        info!("get request");
        Some(rest.get_json(uri, verbose >= Verbosity::Verbose)?)
    };

    if let Some(json_output) = json_output {
        let topic: String = serde_json::from_str(&json_output)?;
        writeln!(config.stdout, "{}", topic)?;
    }

    Ok(())
}