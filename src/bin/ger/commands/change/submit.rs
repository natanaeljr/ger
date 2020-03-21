use super::show;
use crate::config::CliConfig;
use crate::handler::get_remote_restapi_handler;
use clap::{App, Arg, ArgMatches, SubCommand};
use gerlib::changes::{ChangeInfo, SubmitInput};

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
    let remote = args.value_of("remote");
    let change_id = args.value_of("change-id").unwrap();

    let mut rest = get_remote_restapi_handler(config, remote)?;
    let submit_input = SubmitInput {
        on_behalf_of: None, // TODO
        notify: None,
        notify_details: None,
    };
    let change: ChangeInfo = rest.submit_change(change_id, &submit_input)?;

    show::show(config, &change)?;

    Ok(())
}
