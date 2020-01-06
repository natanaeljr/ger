use clap::{App, Arg, SubCommand, ArgMatches};
use crate::config::Config;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("change")
        .about("List changes from the gerrit server.")
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
                .validator(validator::is_u32)
                .help("Limit the number of changes to output."),
        )
}

pub fn exec(config: &Config, args: &ArgMatches) -> Result<(), failure::Error> {
    Ok(())
}
