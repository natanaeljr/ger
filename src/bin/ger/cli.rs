use ansi_term::Color;
use chrono::{DateTime, TimeZone, Utc};
use failure::ResultExt;

/// Ger CLI main entrance
pub fn cli<I, T>(iter: I, out: &mut impl std::io::Write) -> Result<(), failure::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let yaml = load_yaml!("cli.yml");
    let args = clap::App::from_yaml(yaml).get_matches_from(iter);

    match args.subcommand() {
        ("change", subargs) => command_change(subargs, out),
        ("project", subargs) => command_project(subargs, out),
        ("config", subargs) => command_config(subargs, out),
        _ => failure::bail!("invalid subcommand"),
    }?;
    Ok(())
}

fn command_change(
    _args: Option<&clap::ArgMatches>,
    out: &mut impl std::io::Write,
) -> Result<(), failure::Error> {
    let changes = gerlib::get_changes().context("failed to get changes")?;

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
            Color::Blue.paint(format_short_datetime(&change.updated)),
            Color::Cyan.paint(&change.project),
            Color::Green.bold().paint(format!("{:?}", change.status)),
            ansi_term::Style::default().paint(&change.subject)
        )?;
    }

    Ok(())
}

fn command_project(
    _args: Option<&clap::ArgMatches>,
    out: &mut impl std::io::Write,
) -> Result<(), failure::Error> {
    writeln!(out, "Ger PROJECT",)?;
    Ok(())
}

fn command_config(
    _args: Option<&clap::ArgMatches>,
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
fn format_short_datetime(from_utc: &DateTime<Utc>) -> String {
    use chrono::format::{Fixed, Item, Numeric, Pad};
    use chrono::offset::Local;

    let from_local = Local.from_utc_datetime(&from_utc.naive_utc());
    let now_local = Local::now();
    let duration = now_local - from_local;
    let mut format_items = Vec::<Item>::new();

    // TODO: present hour::min AM/PM instead

    if duration.num_days() >= 1 {
        // TODO: substitute push functions
        format_items.push(Item::Fixed(Fixed::ShortMonthName));
        format_items.push(Item::Literal("-"));
        format_items.push(Item::Numeric(Numeric::Day, Pad::Zero));
    }
    // TODO: add year

    from_local
        .format_with_items(format_items.into_iter())
        .to_string()
}

///////////////////////////////////////////////////////////////////////////////////////////////////
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
