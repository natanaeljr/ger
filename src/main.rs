extern crate log;
extern crate strum;
#[macro_use]
extern crate strum_macros;

use exitfailure::ExitFailure;

pub mod cli;
pub mod commands;
pub mod config;
pub mod handler;
pub mod ui;
pub mod util;

fn main() -> Result<(), ExitFailure> {
    pretty_env_logger::init_custom_env("GER_LOG");

    let rv = cli::main(&mut std::env::args_os())?;
    Ok(rv)
}
