#[macro_use]
extern crate clap;
extern crate ansi_term;
extern crate exitfailure;
extern crate failure;

mod cli;

fn main() -> Result<(), exitfailure::ExitFailure> {
    let result = cli::cli(&mut std::env::args_os(), &mut std::io::stdout())?;
    Ok(result)
}
