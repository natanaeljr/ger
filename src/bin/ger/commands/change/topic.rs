use std::io::Write;

use clap::{App, Arg, ArgMatches, SubCommand};

use gerlib::changes::TopicInput;

use crate::config::{CliConfig, Verbosity};
use crate::handler::get_remote_restapi_handler;

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

    let topic_res = if let Some(topic) = args.value_of("set") {
        let topic = TopicInput {
            topic: topic.into(),
        };
        Some(rest.set_topic(change_id, &topic)?)
    } else if args.is_present("delete") {
        rest.delete_topic(change_id)?;
        None
    } else {
        Some(rest.get_topic(change_id)?)
    };

    if let Some(topic) = &topic_res {
        writeln!(config.stdout, "{}", topic)?;
    }

    Ok(())
}
