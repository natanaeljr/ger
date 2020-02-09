use crate::config::{CliConfig, Verbosity};
use crate::handler::get_remote_restapi_handler;
use crate::util;
use clap::{App, Arg, ArgMatches, SubCommand};
use gerlib::projects::ProjectInfo;
use http::uri::PathAndQuery;
use log::info;
use std::collections::HashMap;
use std::io::Write;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("project")
        .about("Lists projects and information about projects.")
        .arg(
            Arg::with_name("project")
                .required(false)
                .multiple(true)
                .help("Project name."),
        )
        .arg(
            Arg::with_name("max-count")
                .short("n")
                .takes_value(true)
                .value_name("limit")
                .validator(util::validate::is_u32)
                .help("Limit the number of projects to output."),
        )
        .arg(
            Arg::with_name("remote")
                .long("remote")
                .short("r")
                .takes_value(true)
                .value_name("NAME")
                .help("Specify an alternative remote to use."),
        )
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
}

pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    let verbose: Verbosity = args.occurrences_of("verbose").into();
    let remote = args.value_of("remote");
    let max_count = args.value_of("max-count");

    let mut rest = get_remote_restapi_handler(config, remote)?;

    let mut query_str = "?d".to_owned();
    if let Some(limit) = max_count {
        query_str = format!("{}&n={}", query_str, limit);
    }
    let uri: PathAndQuery = format!("/a/projects/{}", query_str).parse()?;
    info!("uri: {}", uri);

    let json = rest.request_json(uri, verbose >= Verbosity::Debug)?;
    let projects: HashMap<String, ProjectInfo> = serde_json::from_str(json.as_str())?;
    if projects.is_empty() {
        writeln!(config.stdout, "No projects.")?;
        return Ok(());
    }

    let mut project_maxlen = 0;
    for project in projects.keys() {
        if project.len() > project_maxlen {
            project_maxlen = project.len();
        }
    }

    let mut stdout = config.stdout.lock();
    let no_description = "<no description>".to_string();
    for project in projects.into_iter() {
        write!(stdout, "{}", project.0)?;
        if verbose >= Verbosity::Verbose {
            let padding = project_maxlen - project.0.len();
            let description = project.1.description.as_ref().unwrap_or(&no_description);
            let mut lines = description.lines();
            writeln!(stdout, "{0:1$} - {2}", "", padding, lines.next().unwrap())?;
            let padding = project_maxlen - project.0.len() + project.0.len();
            for line in lines {
                writeln!(stdout, "{0:1$}   {2}", "", padding, line)?;
            }
        } else {
            stdout.write_all(b"\n")?;
        }
    }

    Ok(())
}
