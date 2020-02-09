use crate::config::CliConfig;
use clap::{App, ArgMatches};

pub mod change;
pub mod project;
pub mod remote;

pub fn builtin() -> Vec<App<'static, 'static>> {
    vec![change::cli(), project::cli(), remote::cli()]
}

pub fn builtin_exec(
    cmd: &str,
) -> Option<fn(&mut CliConfig, Option<&ArgMatches>) -> Result<(), failure::Error>> {
    let func = match cmd {
        "change" => change::exec,
        "project" => project::exec,
        "remote" => remote::exec,
        _ => return None,
    };
    Some(func)
}
