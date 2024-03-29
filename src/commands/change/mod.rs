use crate::config::CliConfig;
use clap::{App, ArgMatches, SubCommand};

mod abandon;
mod create;
mod dashboard;
mod list;
mod show;
mod submit;
mod topic;

/// Build the CLI
pub fn cli() -> App<'static, 'static> {
  SubCommand::with_name("change")
    .about("Lists changes and information about changes.")
    .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
    .subcommands(vec![
      dashboard::cli(),
      create::cli(),
      list::cli(),
      show::cli(),
      topic::cli(),
      submit::cli(),
      abandon::cli(),
    ])
}

/// Execute the command
pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
  let args = args.unwrap();
  match args.subcommand() {
    ("", subargs) => {
      let def_args = ArgMatches::default();
      dashboard::exec(config, subargs.or(Some(&def_args)))
    }
    ("dashboard", subargs) => dashboard::exec(config, subargs),
    ("create", subargs) => create::exec(config, subargs),
    ("list", subargs) => list::exec(config, subargs),
    ("show", subargs) => show::exec(config, subargs),
    ("topic", subargs) => topic::exec(config, subargs),
    ("submit", subargs) => submit::exec(config, subargs),
    ("abandon", subargs) => abandon::exec(config, subargs),
    _ => Ok(()),
  }
}
