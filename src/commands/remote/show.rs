use super::prelude::*;
use std::io::Write;
use termcolor::{Color, ColorSpec, WriteColor};

/// Build the CLI for show command
pub fn cli() -> App<'static, 'static> {
  SubCommand::with_name("show")
    .about("Show information about remote.")
    .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
    .arg(Arg::with_name("remote").multiple(true).help("Remote name."))
}

/// Execute the show command
pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
  let args = args.unwrap();
  let verbose: Verbosity = args.occurrences_of("verbose").into();
  match args.values_of("remote") {
    Some(remotes) => show_remotes(config, remotes.into_iter(), verbose),
    None => show_remotes(config, config.user.settings.remotes.keys(), verbose),
  }
}

/// Show basic information about cofigured remotes
pub fn show_list(config: &CliConfig, verbose: Verbosity) -> Result<(), failure::Error> {
  let mut name_maxlen = 0;
  let mut url_maxlen = 0;
  // compute format variables
  for remote in config.user.settings.remotes.iter() {
    if remote.0.len() > name_maxlen {
      name_maxlen = remote.0.len();
    }
    if remote.1.url.len() > url_maxlen {
      url_maxlen = remote.1.url.len();
    }
  }
  // print remotes table
  let mut stdout = config.stdout.lock();
  let default_remote = config.user.settings.default_remote_verify();
  for remote in config.user.settings.remotes.iter() {
    let default = default_remote.is_some() && remote.0 == default_remote.unwrap();
    if default {
      stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    }
    let star = if default { '*' } else { ' ' };
    write!(stdout, "{0} {1}", star, remote.0)?;
    if verbose.ge(&Verbosity::Verbose) {
      let padding = name_maxlen - remote.0.len();
      write!(stdout, "{0:1$} - {2}", "", padding, remote.1.url)?;
    }
    if verbose.ge(&Verbosity::High) {
      let padding = url_maxlen - remote.1.url.len();
      write!(stdout, "{0:1$} ({2})", "", padding, remote.1.username)?;
    }
    stdout.write_all(b"\n")?;
    if default {
      stdout.reset()?;
    }
  }
  Ok(())
}

/// Show information about one or more remotes
pub fn show_remotes<I, T>(config: &CliConfig, iter_remotes: I, verbose: Verbosity) -> Result<(), failure::Error>
where
  I: IntoIterator<Item = T>,
  T: Into<String>,
{
  for name in iter_remotes {
    let name = name.into();
    if let Some(remote) = config.user.settings.remotes.get(&name) {
      show_remote(config, (name.as_str(), remote), verbose.clone())?;
    } else {
      return Err(failure::err_msg(format!("no such remote '{}'", name)));
    }
  }
  Ok(())
}

/// Show information about a given remote
pub fn show_remote(config: &CliConfig, remote: (&str, &RemoteOpts), verbose: Verbosity) -> Result<(), failure::Error> {
  let mut stdout = config.stdout.lock();
  let default_remote = config.user.settings.default_remote_verify();
  let default = default_remote.is_some() && remote.0 == default_remote.unwrap();
  if default {
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
  }
  let star = if default { '*' } else { ' ' };
  writeln!(
    stdout,
    "{} remote: {}\n  url: {}\n  username: {}",
    star, remote.0, remote.1.url, remote.1.username
  )?;
  if verbose >= Verbosity::High {
    writeln!(stdout, "  http_password: {}", remote.1.http_password)?
  }
  writeln!(stdout, "  http_auth: {}", remote.1.http_auth)?;
  if remote.1.no_ssl_verify {
    writeln!(stdout, "  no_ssl_verify: {}", remote.1.no_ssl_verify)?;
  }
  stdout.write_all(b"\n")?;
  if default {
    stdout.reset()?;
  }
  Ok(())
}
