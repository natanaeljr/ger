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

    let gerrit = Gerrit::new("https://gerrit-review.googlesource.com")
        .username("git-natanaeljrabello.gmail.com")
        .password("1//051m9TvuxT1C2CgYIARAAGAUSNwF-L9IrwlzbgR9P3KJYyfb2qGv8PVTXMR5uWjoCWeU6y_dYCP9c3mbSYm5M4y-ZXDdLh1J1LuI");

    let change_opts = gerlib::changes::ChangeOptions::new().limit(max_count);

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
            Color::Blue.paint(utils::format_short_datetime(&change.updated)),
            Color::Cyan.paint(&change.project),
            get_change_status_style(&change.status).paint(format!("{:?}", change.status)),
            ansi_term::Style::default().paint(&change.subject)
        )?;
    }

    Ok(())
}

fn get_change_status_style(status: &gerlib::changes::ChangeStatus) -> ansi_term::Style {
    use gerlib::changes::ChangeStatus;
    match status {
        ChangeStatus::NEW => Color::Green.bold(),
        ChangeStatus::MERGED => Color::Red.bold(),
        ChangeStatus::ABANDONED => Color::Black.bold(),
        ChangeStatus::DRAFT => Color::White.bold().dimmed(),
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
