use crate::config::{CliConfig, Verbosity};
use clap::{App, AppSettings, ArgMatches, SubCommand};

mod browser;
mod change;
mod demo;
mod draw;
mod ecs_tui;
mod layout;
mod rect;
mod scroll;
mod table;
mod term;
mod winbox;

pub fn cli() -> App<'static, 'static> {
  SubCommand::with_name("ui")
    .about("Interactive terminal browser")
    .setting(AppSettings::Hidden)
    .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
}

pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
  let args = args.unwrap();
  let _verbose: Verbosity = args.occurrences_of("verbose").into();
  browser::main(config);
  Ok(())
}
