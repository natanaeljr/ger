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
mod cli_tmp;
mod config;
mod ger;
mod commands;

fn main() -> Result<(), exitfailure::ExitFailure> {
    let rv = ger::Ger::run_cli(&mut std::env::args_os(), &mut std::io::stdout())?;
    Ok(rv)
}
