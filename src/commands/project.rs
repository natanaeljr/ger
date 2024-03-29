use crate::config::{CliConfig, Verbosity};
use crate::handler::get_remote_restapi_handler;
use crate::util;
use clap::{App, Arg, ArgMatches, SubCommand};
use gerlib::projects::ProjectInfo;
use log::info;
use std::collections::HashMap;
use std::io::Write;

pub fn cli() -> App<'static, 'static> {
  SubCommand::with_name("project")
    .about("Lists projects and information about projects.")
    .arg(Arg::with_name("project").required(false).multiple(true).help("Project name."))
    .arg(
      Arg::with_name("limit")
        .long("limit")
        .short("n")
        .takes_value(true)
        .value_name("max-count")
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
    .arg(
      Arg::with_name("prefix")
        .long("prefix")
        .short("p")
        .takes_value(true)
        .value_name("STRING")
        .help(
          "Limit the results to those projects that start with the specified prefix.\n\
                     The match is case sensitive. May not be used together with m or r.",
        ),
    )
    .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
}

pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
  let args = args.unwrap();
  let verbose: Verbosity = args.occurrences_of("verbose").into();
  let remote = args.value_of("remote");
  let limit = args.value_of("limit");
  let prefix = args.value_of("prefix");

  let mut _rest = get_remote_restapi_handler(config, remote)?;

  let mut query_str = "?d".to_owned();
  if let Some(prefix) = prefix {
    query_str = format!("{}&p={}", query_str, prefix);
  }
  if let Some(limit) = limit {
    query_str = format!("{}&n={}", query_str, limit);
  }
  let uri: String = format!("/a/projects/{}", query_str).parse()?;
  info!("uri: {}", uri);

  let _json = String::new();
  //    let json = rest.get_json(uri, verbose >= Verbosity::Debug)?;
  // let projects: HashMap<String, ProjectInfo> = serde_json::from_str(json.as_str())?;
  let projects: HashMap<String, ProjectInfo> = HashMap::new();
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
    if verbose == Verbosity::Verbose {
      let padding = project_maxlen - project.0.len();
      let description = project.1.description.as_ref().unwrap_or(&no_description);
      let desc = description.replace('\n', " ");
      writeln!(stdout, "{0:1$} - {2}", "", padding, desc)?;
    } else if verbose >= Verbosity::High {
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
