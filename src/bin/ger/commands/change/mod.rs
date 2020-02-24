use crate::config::CliConfig;
use clap::{App, ArgMatches, SubCommand};

mod dashboard;
mod list;
mod show;

/// Build the CLI
pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("change")
        .about("Lists changes and information about changes.")
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
        .subcommands(vec![dashboard::cli(), list::cli(), show::cli()])
}

/// Execute the command
pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    match args.subcommand() {
        ("dashboard", subargs) | ("", subargs) => {
            let def_args = ArgMatches::default();
            dashboard::exec(config, subargs.or(Some(&def_args)))
        }
        ("list", subargs) => list::exec(config, subargs),
        ("show", subargs) => show::exec(config, subargs),
        _ => Ok(()),
    }
}
