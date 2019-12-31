use ansi_term::Color;
use gerlib::Gerrit;

use super::config::Config;
use std::str::FromStr;

/// Ger CLI main entrance
pub fn cli<I, T>(iter: I, out: &mut impl std::io::Write) -> Result<(), failure::Error>
    where
        I: IntoIterator<Item=T>,
        T: Into<std::ffi::OsString> + Clone,
{
    let yaml = load_yaml!("cli.yml");
    let args = clap::App::from_yaml(yaml).get_matches_from(iter);

    let home_config_file = format!("{}/.ger.toml", dirs::home_dir().unwrap().to_str().unwrap());
    let config_file = args
        .value_of("config-file")
        .unwrap_or(home_config_file.as_str());
    let config = Config::from_file(config_file).unwrap();

    match args.subcommand() {
        ("change", Some(subargs)) => command_change(&subargs, out, &config),
        ("project", Some(subargs)) => command_project(&subargs, out, &config),
        ("config", Some(subargs)) => command_config(&subargs, out, &config),
        ("gen-completion", Some(subargs)) => command_gen_completion(&subargs, out, &config),
        _ => failure::bail!("invalid subcommand"),
    }?;
    Ok(())
}

fn command_change(
    args: &clap::ArgMatches,
    out: &mut impl std::io::Write,
    config: &Config,
) -> Result<(), failure::Error> {
    let max_count = match args.value_of("max-count").unwrap_or("25").parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            return Err(failure::err_msg(
                "argument of '-n|--max-count' isn't a positive number",
            ));
        }
    };

    let remote = if let Some(remote_arg) = args.value_of("remote") {
        if let Some(remote) = config.remotes.get(remote_arg) {
            remote
        } else {
            failure::bail!(format!(
                "remote ({}) not found in config file",
                remote_arg
            ));
        }
    } else if let Some(default_remote) = &config.default_remote {
        if let Some(remote) = config.remotes.get(default_remote) {
            remote
        } else {
            failure::bail!(format!(
                "default remote ({}) not found in config file",
                default_remote
            ));
        }
    } else if config.remotes.len() == 1 {
        config.remotes.values().next().unwrap()
    } else {
        failure::bail!("no default remote specified");
    };

    use gerlib::changes::{ChangeIs, ChangeOptions, Owner, Query, QueryOpt};
    let host = if remote.port.is_some() {
        format!("{}:{}", &remote.url, remote.port.unwrap())
    } else {
        remote.url.clone()
    };
    let gerrit = Gerrit::new(&host)
        .username(&remote.username)
        .password(&remote.http_password);

    let mut query_opts: Vec<QueryOpt> = Vec::new();

    if let Some(is_v) = args.values_of("is") {
        for is in is_v {
            let mut not = false;
            let is_s = if is.starts_with("-") {
                not = true;
                is[1..].to_lowercase().to_owned()
            } else {
                is.to_lowercase().to_owned()
            };
            let query_opt = match is_s.as_str() {
                "open" | "" => QueryOpt::Is(ChangeIs::Open),
                "draft" => QueryOpt::Is(ChangeIs::Draft),
                "wip" => QueryOpt::Is(ChangeIs::WIP),
                "closed" => QueryOpt::Is(ChangeIs::Closed),
                "reviewer" => QueryOpt::Is(ChangeIs::Reviewer),
                _ => return Err(failure::err_msg("unsupported --is value")),
            };
            query_opts.push(if not {
                QueryOpt::Not(Box::new(query_opt))
            } else {
                query_opt
            });
        }
    }

    if let Some(owner) = args.value_of("owner") {
        let mut not = false;
        let owner_s = if owner.starts_with("-") {
            not = true;
            owner[1..].to_lowercase().to_owned()
        } else {
            owner.to_lowercase().to_owned()
        };
        let query_opt = match owner_s.as_str() {
            "self" => QueryOpt::Owner(Owner::_Self_),
            _ => QueryOpt::Owner(Owner::Other(owner.to_owned())),
        };
        query_opts.push(if not {
            QueryOpt::Not(Box::new(query_opt))
        } else {
            query_opt
        });
    }

    let queries = vec![Query(query_opts)];
    let change_opts = ChangeOptions::new().limit(max_count).queries(queries);
    // println!("{}", change_opts.to_query_string());

    let changes = gerrit.get_changes(change_opts)?;

    if changes.is_empty() {
        writeln!(out, "no changes")?;
        return Ok(());
    }

    for change in changes.iter() {
        let number = format!("{}", change._number);
        writeln!(
            out,
            "{} {} {} {} {}",
            if true {
                Color::Yellow.paint(number).to_string()
            } else {
                number
            },
            Color::Blue.paint(utils::format_short_datetime(&change.updated.0)),
            Color::Cyan.paint(&change.project),
            get_change_status_style(&change.status)
                .paint(format!("{:?}", change.status).to_uppercase()),
            ansi_term::Style::default().paint(&change.subject)
        )?;
    }

    Ok(())
}

fn get_change_status_style(status: &gerlib::changes::ChangeStatus) -> ansi_term::Style {
    use gerlib::changes::ChangeStatus;
    match status {
        ChangeStatus::New => Color::Green.bold(),
        ChangeStatus::Merged => Color::Purple.bold(),
        ChangeStatus::Abandoned => Color::Black.bold(),
        ChangeStatus::Draft => Color::White.bold().dimmed(),
    }
}

fn command_project(
    _args: &clap::ArgMatches,
    out: &mut impl std::io::Write,
    _config: &Config,
) -> Result<(), failure::Error> {
    writeln!(out, "Ger PROJECT", )?;
    Ok(())
}

fn command_config(
    _args: &clap::ArgMatches,
    out: &mut impl std::io::Write,
    _config: &Config,
) -> Result<(), failure::Error> {
    writeln!(
        out,
        "{} {}",
        Color::Blue.paint("Ger"),
        Color::Blue.bold().paint("CONFIG")
    )?;
    Ok(())
}

fn command_gen_completion(
    args: &clap::ArgMatches,
    out: &mut impl std::io::Write,
    _config: &Config,
) -> Result<(), failure::Error> {
    let shell = clap::Shell::from_str(args.value_of("SHELL").unwrap()).unwrap();
    let yaml = load_yaml!("cli.yml");
    clap::App::from_yaml(yaml).gen_completions_to("ger", shell, out);
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// UTILS
mod utils {
    use chrono::{DateTime, TimeZone, Utc};

    /// Dynamic short format for DataTime
    pub fn format_short_datetime(from_utc: &DateTime<Utc>) -> String {
        use chrono::format::{Fixed, Item, Numeric, Pad};
        use chrono::offset::Local;
        use chrono::Datelike;

        let from_local = Local.from_utc_datetime(&from_utc.naive_utc());
        let now_local = Local::now();
        let duration = now_local - from_local;

        let mut format_items = Vec::new();
        if (duration.num_days() == 0) && (from_local.day() == now_local.day()) {
            format_items.reserve(5);
            format_items.push(Item::Numeric(Numeric::Hour12, Pad::Zero));
            format_items.push(Item::Literal(":"));
            format_items.push(Item::Numeric(Numeric::Minute, Pad::Zero));
            format_items.push(Item::Literal("_"));
            format_items.push(Item::Fixed(Fixed::UpperAmPm));
        } else {
            format_items.reserve(5);
            format_items.push(Item::Fixed(Fixed::ShortMonthName));
            format_items.push(Item::Literal("_"));
            format_items.push(Item::Numeric(Numeric::Day, Pad::Zero));
            if from_local.year() != now_local.year() {
                format_items.push(Item::Literal(","));
                format_items.push(Item::Numeric(Numeric::Year, Pad::Zero));
            }
        }

        from_local
            .format_with_items(format_items.into_iter())
            .to_string()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// TESTS
#[cfg(test)]
mod tests {
    /// Test output from CLI subcommand 'project'.
    #[test]
    fn cli_project() {
        let mut writer = Vec::new();
        let args = vec!["ger", "project"];
        super::cli(args, &mut writer).unwrap();
        let output = std::str::from_utf8(writer.as_slice()).unwrap();
        assert_eq!(output, "Ger PROJECT\n");
    }
}
