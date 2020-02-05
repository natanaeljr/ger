#![allow(dead_code)]

extern crate log;

pub mod cli;
pub mod commands;
pub mod config;
pub mod handler;
pub mod util;

use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    pretty_env_logger::init_custom_env("GER_LOG");

    let rv = cli::main(&mut std::env::args_os())?;
    Ok(rv)
}
