use crate::config::{CliConfig, Verbosity};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

mod home;
#[allow(dead_code)]
pub mod util;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("ui")
        .about("Interactive terminal browser")
        .setting(AppSettings::Hidden)
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
        .arg(
            Arg::with_name("mode")
                .long("mode")
                .short("m")
                .help("UI mode [testing]")
                .possible_values(&["input"])
                .takes_value(true),
        )
}

pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    let _verbose: Verbosity = args.occurrences_of("verbose").into();
    let mode = args.value_of("mode").unwrap_or("home");
    browser(config, mode)
}

pub fn browser(config: &mut CliConfig, mode: &str) -> Result<(), failure::Error> {
    match mode {
        "home" => home::main(config).unwrap(),
        &_ => unreachable!(),
    }
    Ok(())
}
