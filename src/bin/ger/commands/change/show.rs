use crate::config::CliConfig;
use clap::{App, Arg, ArgMatches, SubCommand};

/// Build the CLI
pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("show")
        .about("Show information about changes.")
        .arg(
            Arg::with_name("CHANGE")
                .required(false)
                .multiple(true)
                .help(
                    "Change identifier.\n\
                     Can be either a legacy numerical id (e.g. 15813),\
                     full or abbreviated Change-Id (e.g. Ic0ff33)\
                     or commit SHA-1 (e.g. d81b32ef).",
                ),
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
pub fn exec(_config: &mut CliConfig, _args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    Err(failure::err_msg("Show not implemented yet!"))
}
