use crate::config::{CliConfig, Verbosity};
use clap::{App, AppSettings, ArgMatches, SubCommand};

mod winbox;
mod browser;
mod change;
mod demo;
mod draw;
mod ecs_tui;
mod layout;
mod scroll;
mod table;
mod term;
mod rect;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("ui")
        .about("Interactive terminal browser")
        .setting(AppSettings::Hidden)
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
}

pub fn exec(_config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    let _verbose: Verbosity = args.occurrences_of("verbose").into();
    browser::main();
    Ok(())
}
