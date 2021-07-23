use super::prelude::*;
use std::io::Write;

/// Build the CLI for show command
pub fn cli() -> App<'static, 'static> {
  SubCommand::with_name("default")
    .about("Set the default remote or display current one.")
    .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
    .arg(Arg::with_name("remote").help("Remote name."))
}

/// Execute the default command
pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
  let args = args.unwrap();
  if let Some(remote) = args.value_of("remote") {
    set(config, remote)?
  } else {
    if let Some(default) = config.user.settings.default_remote_verify() {
      writeln!(config.stdout, "{}", default)?;
    } else {
      return Err(failure::err_msg("no default remote"));
    }
  }
  Ok(())
}

/// Set remote as default remote
pub fn set(config: &mut CliConfig, remote: &str) -> Result<(), failure::Error> {
  if config.user.settings.remotes.contains_key(remote) {
    config.user.settings.set_default_remote(Some(remote.into()));
    config.user.store()?;
    Ok(())
  } else {
    Err(failure::err_msg(format!("no such remote: {}", remote)))
  }
}
