use super::show;
use crate::config::CliConfig;
use crate::handler::get_remote_restapi_handler;
use clap::{App, Arg, ArgMatches, SubCommand};
use gerlib::changes::{ChangeInfo, ChangeInput, ChangeStatus};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("create")
        .about("Create a new change.")
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
        .setting(clap::AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("project")
                .long("project")
                .short("p")
                .takes_value(true)
                .required(true)
                .help("The name of the project."),
        )
        .arg(
            Arg::with_name("branch")
                .long("branch")
                .short("b")
                .takes_value(true)
                .required(true)
                .help("The name of the target branch."),
        )
        .arg(
            Arg::with_name("subject")
                .long("subject")
                .short("s")
                .takes_value(true)
                .required(true)
                .help(
                    "The commit message of the change.\n\
                     Comment lines (beginning with #) will be removed.",
                ),
        )
        .arg(
            Arg::with_name("topic")
                .long("topic")
                .short("t")
                .takes_value(true)
                .help("The topic to which this change belongs."),
        )
        .arg(
            Arg::with_name("draft")
                .long("draft")
                .help("Init change status as in DRAFT. Only for old gerrit versions (v2)."),
        )
        .arg(
            Arg::with_name("remote")
                .long("remote")
                .short("r")
                .takes_value(true)
                .help("Specify an alternative remote to use."),
        )
}

pub fn exec(config: &mut CliConfig, args: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let args = args.unwrap();
    let remote = args.value_of("remote");

    let mut rest = get_remote_restapi_handler(config, remote)?;

    let change_input = ChangeInput {
        project: args.value_of("project").unwrap().into(),
        branch: args.value_of("branch").unwrap().into(),
        subject: args.value_of("subject").unwrap().into(),
        topic: args.value_of("topic").map(|t| t.into()),
        status: match args.is_present("draft") {
            true => Some(ChangeStatus::Draft),
            false => None,
        },
        is_private: None,
        work_in_progress: None,
        base_change: None,
        base_commit: None,
        new_branch: None,
        merge: None,
        author: None,
        notify: None,
        notify_details: None,
    };

    let change_info: ChangeInfo = rest.create_change(&change_input)?;

    show::show(config, &change_info)?;

    Ok(())
}
