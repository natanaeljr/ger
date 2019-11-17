use ansi_term::Color;
use failure::ResultExt;

/// Ger CLI main entrance
pub fn cli_main(out: impl std::io::Write) -> Result<(), failure::Error> {
    let yaml = load_yaml!("cli.yml");
    let args = clap::App::from_yaml(yaml).get_matches();

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
    mut out: impl std::io::Write,
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
    mut out: impl std::io::Write,
) -> Result<(), failure::Error> {
    writeln!(
        out,
        "{} {}",
        Color::Purple.paint("Ger"),
        Color::Purple.bold().paint("PROJECT")
    )?;
    Ok(())
}

fn command_config(
    _args: Option<&clap::ArgMatches>,
    mut out: impl std::io::Write,
) -> Result<(), failure::Error> {
    writeln!(
        out,
        "{} {}",
        Color::Blue.paint("Ger"),
        Color::Blue.bold().paint("CONFIG")
    )?;
    Ok(())
}
