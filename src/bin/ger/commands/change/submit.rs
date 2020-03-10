use super::show;
use crate::config::{CliConfig, Verbosity};
use crate::handler::get_remote_restapi_handler;
use clap::{App, Arg, ArgMatches, SubCommand};
use gerlib::rest::changes::{ChangeInfo, SubmitInput};
use http::uri::PathAndQuery;
use log::info;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("submit")
        .about("Submit a change.")
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
        .setting(clap::AppSettings::DeriveDisplayOrder)
        .arg(Arg::with_name("change-id").required(true).help(
            "Change identifier. \
             Can be either a legacy numerical id (e.g. 15813), \
             full or abbreviated Change-Id (e.g. Ic0ff33) \
             or commit SHA-1 (e.g. d81b32ef).",
        ))
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

    let mut _rest = get_remote_restapi_handler(config, remote)?;
    let uri: PathAndQuery = format!("/a/changes/{}/submit", change_id).parse()?;
    info!("uri: {}", uri);

    let submit_input = SubmitInput {
        on_behalf_of: None, // TODO
        notify: None,
        notify_details: None,
    };
    let json_input = serde_json::to_string_pretty(&submit_input)?;
    let json_output = String::new();
//    let json_output = rest.post_json(
//        uri,
//        200,
//        json_input.as_bytes(),
//        verbose >= Verbosity::Verbose,
//    )?;
    let change: ChangeInfo = serde_json::from_str(&json_output)?;

    show::show(config, &change)?;

    Ok(())
}
