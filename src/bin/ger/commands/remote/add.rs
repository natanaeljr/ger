use super::prelude::*;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("add")
        .about("Add a new remote.")
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}\n\n{after-help}")
        .after_help("EXAMPLE:\n    ger remote add mygerrit https://mygerrit.company.com")
        .setting(clap::AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("name")
                .required(true)
                .help("Remote unique identifier."),
        )
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
                .value_name("ID")
                .help("Username for login."),
        )
        .arg(
            Arg::with_name("password")
                .long("http-password")
                .short("p")
                .takes_value(true)
                .value_name("STRING")
                .help(
                    "HTTP password. Can be generated in gerrit user settings menu.\n\
                         Note: this password is saved in plain text in the configuration file.",
                ),
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

    if config.user_cfg.settings.remotes.contains_key(name) {
        return Err(failure::err_msg(format!(
            "remote '{}' already exists.",
            name
        )));
    }

    let username = match username {
        Some(u) => u,
        None => super::prompt_username(&mut config.stdout, name)?,
    };

    let http_password = match http_password {
        Some(p) => p,
        None => super::prompt_http_password(name)?,
    };

    config.user_cfg.settings.remotes.insert(
        name.into(),
        RemoteOpts {
            url: url.to_owned(),
            username,
            http_password,
            no_ssl_verify,
        },
    );
    config.user_cfg.store()?;

    Ok(())
}
