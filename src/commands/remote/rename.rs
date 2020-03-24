use super::prelude::*;
use std::io::Write;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("rename")
        .visible_alias("mv")
        .about("Rename a remote.")
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
        .arg(
            Arg::with_name("old")
                .takes_value(true)
                .required(true)
                .index(1)
                .help("Current remote name that is going to be renamed."),
        )
        .arg(
            Arg::with_name("new")
                .takes_value(true)
                .required(true)
                .index(2)
                .help("New remote name."),
        )
}

pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    let verbose: Verbosity = args.occurrences_of("verbose").into();
    let old = args.value_of("old").unwrap();
    let new = args.value_of("new").unwrap();

    match config.user.settings.remotes.remove(old) {
        Some(remote) => {
            config.user.settings.remotes.insert(new.to_owned(), remote);
            ()
        }
        None => return Err(failure::err_msg(format!("no such remote: {}", old))),
    }

    match config.user.settings.default_remote() {
        Some(name) => {
            if name == old {
                config
                    .user
                    .settings
                    .set_default_remote(Some(new.to_owned()));
            }
        }
        None => {}
    }

    config.user.store()?;
    if verbose >= Verbosity::Verbose {
        writeln!(config.stdout, "renamed remote '{}' to '{}'", old, new)?;
    }

    Ok(())
}
