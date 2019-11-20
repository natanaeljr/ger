#[macro_use]
extern crate clap;
extern crate ansi_term;
extern crate chrono;
extern crate exitfailure;
extern crate failure;
use chrono::prelude::*;
mod cli;

fn main() -> Result<(), exitfailure::ExitFailure> {
    test();
    let result = cli::cli(&mut std::env::args_os(), &mut std::io::stdout())?;
    Ok(result)
}

fn test() {
    let time = Utc.datetime_from_str("2019-11-18 22:19:54.000000000", "%Y-%m-%d %H:%M:%S.%f");
    println!("{:#?}\n", time);
}
