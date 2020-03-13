use crate::config::{CliConfig, Verbosity};
use crate::handler::get_remote_restapi_handler;
use crate::util;
use clap::{App, Arg, ArgMatches, SubCommand};
use gerlib::changes::{AdditionalOpt, ChangeInfo, QueryParams};
use http::uri::PathAndQuery;
use log::info;
use std::io::Write;
use termcolor::{Color, ColorSpec, WriteColor};

/// Build the CLI
pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("list")
        .visible_alias("ls")
        .about("Lists changes.")
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
        .arg(
            Arg::with_name("limit")
                .long("limit")
                .short("n")
                .takes_value(true)
                .value_name("max-count")
                .validator(util::validate::is_u32)
                .help(
                    "Limit the number of changes to output. Defaults to the terminal height. \
                     If stdout is not a tty, the default falls back to 25.",
                ),
        )
        .arg(
            Arg::with_name("remote")
                .long("remote")
                .short("r")
                .takes_value(true)
                .help("Specify an alternative remote to use."),
        )
}

/// Execute the command
pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    let verbose: Verbosity = args.occurrences_of("verbose").into();
    let remote = args.value_of("remote");
    let limit = args
        .value_of("limit")
        .map(|n| n.parse::<u32>().unwrap())
        .unwrap_or_else(|| match term_size::dimensions_stdout() {
            Some((_, h)) => {
                let height = h as i64 - 5;
                if height > 0 {
                    height as u32
                } else {
                    25
                }
            }
            None => 25,
        });

    let mut rest = get_remote_restapi_handler(config, remote)?;
    let query_param = QueryParams {
        search_queries: None,
        additional_opts: Some(vec![
            AdditionalOpt::DetailedAccounts,
            AdditionalOpt::CurrentRevision,
        ]),
        limit: Some(limit),
        start: None,
    };
    let changes_list: Vec<Vec<ChangeInfo>> = rest.query_changes(&query_param)?;

    if changes_list.is_empty() {
        writeln!(config.stdout, "No changes.")?;
        return Ok(());
    }
    for changes in &changes_list {
        list(config, changes)?;
    }

    Ok(())
}

/// Show list of changes
pub fn list(config: &mut CliConfig, changes: &Vec<ChangeInfo>) -> Result<(), failure::Error> {
    if changes.is_empty() {
        writeln!(config.stdout, "No changes.")?;
        return Ok(());
    }

    let mut stdout = config.stdout.lock();
    for change in changes {
        stdout.reset()?;

        if let Some(current_revision) = &change.current_revision {
            write!(stdout, "{}", &current_revision[..7])?;
        }

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        write!(stdout, " {}", change.number)?;

        if let Some(owner_name) = &change.owner.name {
            stdout.set_color(
                ColorSpec::new()
                    .set_fg(Some(Color::Black))
                    .set_intense(true),
            )?;
            write!(stdout, " {}", owner_name)?;
        }

        stdout.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Magenta))
                .set_intense(true),
        )?;
        write!(
            stdout,
            " {}",
            util::format_short_datetime(&change.updated.0)
        )?;

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        write!(stdout, " {}", change.project)?;

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_intense(true))?;
        write!(stdout, " {}", change.branch)?;

        if let Some(topic) = &change.topic {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;
            write!(stdout, " {}", topic)?;
        }

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
        write!(stdout, " {}", &change.status)?;
        if change.work_in_progress {
            write!(stdout, " WIP")?;
        }

        stdout.reset()?;
        write!(stdout, " {}", change.subject)?;
        stdout.write_all(b"\n")?;
    }

    Ok(())
}
