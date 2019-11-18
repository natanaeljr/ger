use ansi_term::Color;
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
            "* {} {} {}",
            if true {
                Color::Yellow.paint(number).to_string()
            } else {
                number
            },
            Color::Green.bold().paint(format!("{:?}", change.status)),
            ansi_term::Style::default().paint(format!("{}", change.subject))
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
