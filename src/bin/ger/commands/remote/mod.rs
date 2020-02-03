use prelude::*;
use std::io::Write;
use termcolor::StandardStream;

mod prelude {
    pub use crate::config::{CliConfig, RemoteOpts, Verbosity};
    pub use crate::util;
    pub use clap::{App, Arg, ArgMatches, SubCommand};
}

mod add;
mod default;
mod remove;
mod show;

/// Build the CLI
pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("remote")
        .about("Manage gerrit remote servers.")
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
        .subcommands(vec![add::cli(), show::cli(), remove::cli(), default::cli()])
}

/// Execute the remote command
pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    match args.subcommand() {
        ("add", subargs) => add::exec(config, subargs),
        ("show", subargs) => show::exec(config, subargs),
        ("", _) => show::show_list(config, args.occurrences_of("verbose").into()),
        ("remove", subargs) => remove::exec(config, subargs),
        ("default", subargs) => default::exec(config, subargs),
        _ => Ok(()),
    }
}

/// Prompt for Username for given remote
fn prompt_username(stdout: &mut StandardStream, remote: &str) -> Result<String, failure::Error> {
    write!(stdout, "Username for '{}': ", remote)?;
    stdout.flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().into())
}

/// Prompt for HTTP Password for given remote
fn prompt_http_password(remote: &str) -> Result<String, failure::Error> {
    let prompt = format!("HTTP-Password for '{}': ", remote);
    let input = rpassword::read_password_from_tty(Some(prompt.as_str()))?;
    Ok(input.trim().into())
}
