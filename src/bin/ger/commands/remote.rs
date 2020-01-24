use prelude::*;

mod prelude {
    pub use crate::config::Remote;
    pub use crate::config::{CliConfig, Verbosity};
    pub use crate::util;
    pub use clap::{App, Arg, ArgMatches, SubCommand};
}

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("remote")
        .about("Manage gerrit remote servers.")
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
        .subcommands(vec![add::cli(), show::cli(), remove::cli(), default::cli()])
}

pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    match args.subcommand() {
        ("add", subargs) => add::exec(config, subargs),
        ("show", subargs) => show::exec(config, subargs),
        ("", _) => show::show_list(config, args.occurrences_of("verbose").into()),
        ("remove", subargs) => remove::exec(config, subargs),
        ("default", subargs) => default::exec(config, subargs),
        _ => Ok(()),
    }
}

/**************************************************************************************************/
mod show {
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
            None => show_remotes(config, config.user_cfg.settings.remotes.keys(), verbose),
        }
    }

    /// Show basic information about cofigured remotes
    pub fn show_list(config: &CliConfig, verbose: Verbosity) -> Result<(), failure::Error> {
        let mut name_maxlen = 0;
        let mut column2_maxlen = 0;
        // compute format variables
        for remote in config.user_cfg.settings.remotes.iter() {
            if remote.0.len() > name_maxlen {
                name_maxlen = remote.0.len();
            }
            let mut column2_len = remote.1.url.len();
            if let Some(port) = remote.1.port {
                column2_len += format!(" [{}]", port).len();
            }
            if column2_len > column2_maxlen {
                column2_maxlen = column2_len;
            }
        }
        // print remotes table
        let default_remote = config.user_cfg.settings.default_remote();
        for remote in config.user_cfg.settings.remotes.iter() {
            let mut stdout = config.stdout.lock();
            let mut port_len = 0;
            let default = default_remote.is_some() && remote.0 == default_remote.unwrap();
            if default {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            }
            let star = if default { '*' } else { ' ' };
            write!(stdout, "{0} {1}", star, remote.0)?;
            if verbose.ge(&Verbosity::Verbose) {
                write!(
                    stdout,
                    "{0:1$} - {2}",
                    "",
                    name_maxlen - remote.0.len(),
                    remote.1.url,
                )?;
                if let Some(port) = &remote.1.port {
                    let port_str = format!(" [{}]", port);
                    write!(stdout, "{}", port_str)?;
                    port_len = port_str.len();
                }
            }
            if verbose.ge(&Verbosity::High) {
                let column2_len = remote.1.url.len() + port_len;
                let padding = column2_maxlen - column2_len;
                write!(stdout, "{0:1$}", "", padding)?;
                if let Some(username) = &remote.1.username {
                    write!(stdout, " ({})", username)?
                }
            }
            stdout.write_all(b"\n")?;
            if default {
                stdout.reset()?;
            }
        }
        Ok(())
    }

    /// Show information about one or more remotes
    pub fn show_remotes<I, T>(
        config: &CliConfig, iter_remotes: I, verbose: Verbosity,
    ) -> Result<(), failure::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        for name in iter_remotes {
            let name = name.into();
            if let Some(remote) = config.user_cfg.settings.remotes.get(&name) {
                show_remote(config, (name.as_str(), remote), verbose.clone())?;
            } else {
                return Err(failure::err_msg(format!("no such remote '{}'", name)));
            }
        }
        Ok(())
    }

    /// Show information about a given remote
    pub fn show_remote(
        config: &CliConfig, remote: (&str, &Remote), _verbose: Verbosity,
    ) -> Result<(), failure::Error> {
        let mut stdout = config.stdout.lock();
        writeln!(stdout, "* remote: {}\n  url: {}", remote.0, remote.1.url)?;
        writeln!(
            stdout,
            "  port: {}",
            remote
                .1
                .port
                .unwrap_or_else(|| return util::default_port_for_url(remote.1.url.as_str()))
        )?;
        if let Some(username) = &remote.1.username {
            writeln!(stdout, "  login: {}", username)?
        }
        stdout.write_all(b"\n")?;
        Ok(())
    }
}

/**************************************************************************************************/
mod add {
    use super::prelude::*;

    pub fn cli() -> App<'static, 'static> {
        SubCommand::with_name("add")
            .about("Add a new remote.")
            .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
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
                Arg::with_name("port")
                    .takes_value(true)
                    .validator(util::validate::is_u16_range)
                    .help("Port to use on connection with server."),
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
                    .long("password")
                    .short("p")
                    .takes_value(true)
                    .value_name("STRING")
                    .min_values(0)
                    .help(
                        "HTTP password. Can be generated in gerrit user settings menu.\n\
                         Pass only the flag without value to be prompted for (recommended).\n\
                         Note: this password is saved in plain text in the configuration file.",
                    ),
            )
    }

    pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
        let args = args.unwrap();

        let name = args.value_of("name").unwrap();
        let url = args.value_of("url").unwrap();
        let port = match args.value_of("port") {
            Some(p) => Some(p.parse::<u16>().unwrap()),
            None => None,
        };
        let username = match args.value_of("username") {
            Some(u) => Some(u.to_owned()),
            None => None,
        };
        let http_password = match args.value_of("password") {
            Some(p) => Some(p.to_owned()),
            None => None,
        };

        if config.user_cfg.settings.remotes.contains_key(name) {
            return Err(failure::err_msg(format!(
                "remote '{}' already exists.",
                name
            )));
        }

        config.user_cfg.settings.remotes.insert(
            name.into(),
            Remote {
                url: url.to_owned(),
                port,
                username,
                http_password,
            },
        );
        config.user_cfg.store()?;

        Ok(())
    }
}

/**************************************************************************************************/
mod remove {
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
}

/**************************************************************************************************/
mod default {
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
            if let Some(default) = config.user_cfg.settings.default_remote() {
                writeln!(config.stdout, "{}", default)?;
            } else {
                return Err(failure::err_msg("no default remote set"));
            }
        }
        Ok(())
    }

    /// Set remote as default remote
    pub fn set(config: &mut CliConfig, remote: &str) -> Result<(), failure::Error> {
        if config.user_cfg.settings.remotes.contains_key(remote) {
            config
                .user_cfg
                .settings
                .set_default_remote(Some(remote.into()));
            config.user_cfg.store()?;
            Ok(())
        } else {
            Err(failure::err_msg(format!("no such remote: {}", remote)))
        }
    }
}
