#![allow(dead_code)]

#[macro_use]
extern crate log;

mod cli;
mod commands;
mod config;
mod util;

use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    pretty_env_logger::init_custom_env("GER_LOG");
    trace!("Initiating ger");

    let rv = cli::main(&mut std::env::args_os())?;
    Ok(rv)
}
