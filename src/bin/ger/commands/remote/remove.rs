use super::prelude::*;
use std::io::Write;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("remove")
        .visible_alias("rm")
        .about("Remove a remote from config.")
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
        .arg(
            Arg::with_name("remote")
                .required(true)
                .multiple(true)
                .help("Remote name."),
        )
}

pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    let remotes = args.values_of("remote").unwrap();
    for remote in remotes.into_iter() {
        let mut stdout = config.stdout.lock();
        match config.user_cfg.settings.remotes.remove(remote) {
            Some(_) => writeln!(stdout, "removed remote {}", remote)?,
            None => writeln!(stdout, "fatal: no such remote: {}", remote)?,
        };
    }
    config.user_cfg.store()?;
    Ok(())
}
