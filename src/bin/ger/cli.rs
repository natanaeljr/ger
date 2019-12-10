use ansi_term::Color;
use gerlib::Gerrit;

/// Ger CLI main entrance
pub fn cli<I, T>(iter: I, out: &mut impl std::io::Write) -> Result<(), failure::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let yaml = load_yaml!("cli.yml");
    let args = clap::App::from_yaml(yaml).get_matches_from(iter);

    match args.subcommand() {
        ("change", Some(subargs)) => command_change(&subargs, out),
        ("project", Some(subargs)) => command_project(&subargs, out),
        ("config", Some(subargs)) => command_config(&subargs, out),
        _ => failure::bail!("invalid subcommand"),
    }?;
    Ok(())
}

fn command_change(
    args: &clap::ArgMatches,
    out: &mut impl std::io::Write,
) -> Result<(), failure::Error> {
    let max_count = match args.value_of("max-count").unwrap_or("25").parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            return Err(failure::err_msg(
                "argument of '-n|--max-count' isn't a positive number",
            ))
        }
    };

    use gerlib::changes::{ChangeIs, ChangeOptions, Owner, Query, QueryOpt};

    let gerrit = Gerrit::new("")
        .username("")
        .password("");

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
                "draft" | "wip" => QueryOpt::Is(ChangeIs::Draft),
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
) -> Result<(), failure::Error> {
    writeln!(out, "Ger PROJECT",)?;
    Ok(())
}

fn command_config(
    _args: &clap::ArgMatches,
    out: &mut impl std::io::Write,
) -> Result<(), failure::Error> {
    writeln!(
        out,
        "{} {}",
        Color::Blue.paint("Ger"),
        Color::Blue.bold().paint("CONFIG")
    )?;
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
