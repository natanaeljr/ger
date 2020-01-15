use crate::config::CliConfig;
use crate::util;
use clap::{App, Arg, ArgMatches, SubCommand};
use termcolor::{Color, ColorSpec, WriteColor};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("change")
        .about("Lists information about changes.")
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
                .validator(util::validate::is_u32)
                .help("Limit the number of changes to output."),
        )
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
}

pub fn exec(config: &mut CliConfig, _args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    config
        .stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))
        .unwrap();
    println!("Command: change");
    config.stdout.reset().unwrap();
    Ok(())
}
