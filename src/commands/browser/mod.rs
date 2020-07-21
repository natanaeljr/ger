use crate::config::{CliConfig, Verbosity};
use clap::{App, AppSettings, ArgMatches, SubCommand};

mod user_input;
#[allow(dead_code)]
pub mod util;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("enter")
        .about("Interactive terminal browser")
        .setting(AppSettings::Hidden)
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
}

pub fn exec(_config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    let _verbose: Verbosity = args.occurrences_of("verbose").into();
    browser()
}

pub fn browser() -> Result<(), failure::Error> {
    user_input::main().unwrap();
    Ok(())
}
