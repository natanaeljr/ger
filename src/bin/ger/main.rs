#[macro_use]
extern crate clap;
extern crate ansi_term;
extern crate exitfailure;
extern crate failure;

mod cli;

fn main() -> Result<(), exitfailure::ExitFailure> {
    let result = cli::cli_main(&mut std::io::stdout());
    Ok(result?)
}
