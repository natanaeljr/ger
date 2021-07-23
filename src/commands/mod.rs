use clap::{App, ArgMatches};

use crate::config::CliConfig;
use crate::ui;

pub mod change;
pub mod project;
pub mod remote;

pub fn builtin() -> Vec<App<'static, 'static>> {
  vec![change::cli(), project::cli(), remote::cli(), ui::cli()]
}

pub fn builtin_exec(cmd: &str) -> Option<fn(&mut CliConfig, Option<&ArgMatches>) -> Result<(), failure::Error>> {
  let func = match cmd {
    "change" => change::exec,
    "project" => project::exec,
    "remote" => remote::exec,
    "ui" => ui::exec,
    _ => return None,
  };
  Some(func)
}
