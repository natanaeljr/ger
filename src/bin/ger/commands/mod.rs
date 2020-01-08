use crate::config::CliConfig;
use clap::{App, ArgMatches};

pub mod change;

pub fn builtin() -> Vec<App<'static, 'static>> {
    vec![change::cli()]
}

pub fn builtin_exec(
    cmd: &str,
) -> Option<fn(&mut CliConfig, Option<&ArgMatches>) -> Result<(), failure::Error>> {
    let func = match cmd {
        "change" => change::exec,
        _ => return None,
    };
    Some(func)
}
