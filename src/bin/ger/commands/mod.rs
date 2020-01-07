use crate::config::UserConfig;
use clap::{App, ArgMatches};
use failure::Error;

pub mod change;

pub fn builtin() -> Vec<App<'static, 'static>> {
    vec![change::cli()]
}

pub fn builtin_exec(
    cmd: &str,
) -> Option<fn(&mut UserConfig, Option<&ArgMatches>) -> Result<(), failure::Error>> {
    let func = match cmd {
        "change" => change::exec,
        _ => return None,
    };
    Some(func)
}
