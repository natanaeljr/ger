#![allow(dead_code)]

#[macro_use]
extern crate clap;
extern crate ansi_term;
extern crate chrono;
extern crate dirs;
extern crate exitfailure;
extern crate failure;
extern crate gerlib;

mod cli;
mod commands;
mod config;
mod ger;
mod util;

use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    let rv = ger::Ger::run_cli(&mut std::env::args_os(), &mut std::io::stdout())?;
    Ok(rv)
}
