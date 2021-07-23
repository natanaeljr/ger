use super::prelude::*;
use crate::config::HttpAuthMethod;

pub fn cli() -> App<'static, 'static> {
  SubCommand::with_name("add")
    .about("Add a new remote.")
    .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}\n\n{after-help}")
    .after_help("EXAMPLE:\n    ger remote add mygerrit https://gerrit-review.company.com")
    .setting(clap::AppSettings::DeriveDisplayOrder)
    .arg(Arg::with_name("name").required(true).help("Remote unique identifier."))
    .arg(
      Arg::with_name("url")
        .required(true)
        .validator(util::validate::is_url_http_https)
        .help("Remote URL including protocol. e.g. 'https://mygerrit.com'."),
    )
    .arg(
      Arg::with_name("username")
        .long("username")
        .short("u")
        .takes_value(true)
        .value_name("id")
        .help("Username for login."),
    )
    .arg(
      Arg::with_name("password")
        .long("http-password")
        .short("p")
        .takes_value(true)
        .value_name("string")
        .help(
          "HTTP password. Can be generated in gerrit user settings menu.\n\
                     Note: this password is saved in plain text in the configuration file.",
        ),
    )
    .arg(
      Arg::with_name("http-auth")
        .long("http-auth")
        .takes_value(true)
        .value_name("method")
        .possible_values(&["basic", "digest"])
        .default_value("basic")
        .help("Use HTTP Basic Authentication or Digest Authentication."),
    )
    .arg(
      Arg::with_name("no-ssl-verify")
        .long("no-ssl-verify")
        .help("Do not to verify the SSL certificate for HTTPS."),
    )
}

pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
  let args = args.unwrap();

  let name = args.value_of("name").unwrap();
  let url = args.value_of("url").unwrap();
  let username = args.value_of("username").map(|s| s.to_owned());
  let http_password = args.value_of("password").map(|s| s.to_owned());
  let no_ssl_verify = args.is_present("no-ssl-verify");
  let http_auth: HttpAuthMethod = args.value_of("http-auth").unwrap().parse()?;

  if config.user.settings.remotes.contains_key(name) {
    return Err(failure::err_msg(format!("remote '{}' already exists.", name)));
  }

  let username = match username {
    Some(u) => u,
    None => super::prompt_username(&mut config.stdout, name)?,
  };

  let http_password = match http_password {
    Some(p) => p,
    None => super::prompt_http_password(name)?,
  };

  config.user.settings.remotes.insert(
    name.into(),
    RemoteOpts {
      url: url.to_owned(),
      username,
      http_password,
      http_auth,
      no_ssl_verify,
    },
  );
  config.user.store()?;

  Ok(())
}
