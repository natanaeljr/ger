use clap::{App, AppSettings, Arg, SubCommand};
mod validator;

/**************************************************************************************************/
/// Build GER Clap App
pub fn build_cli() -> App<'static, 'static> {
    App::new("ger")
        .version("0.1.0")
        .about("Gerrit command-line interface.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("config-file")
                .long("config-file")
                .takes_value(true)
                .value_name("FILE")
                .help("Specify alternative TOML configuration file."),
        )
        .arg(
            Arg::with_name("remote")
                .long("remote")
                .short("r")
                .takes_value(true)
                .value_name("NAME")
                .help("Remote server to use from config-file."),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .global(true)
                .multiple(true)
                .help("Set level of verbosity (up to 3)"),
        )
        .subcommands(build_subcommands())
}

/**************************************************************************************************/
fn build_subcommands() -> Vec<App<'static, 'static>> {
    vec![subcommand_change()]
}

/**************************************************************************************************/
fn subcommand_change() -> App<'static, 'static> {
    SubCommand::with_name("change")
        .about("List changes from the gerrit server.")
        .arg(
            Arg::with_name("CHANGE")
                .required(false)
                .multiple(true)
                .help(
                    "Specify changes to look for.\n\
                     Can be either a legacy numerical id (e.g. 15813),\
                     full or abbreviated Change-Id (e.g. Ic0ff33)\
                     or commit SHA-1 (e.g. d81b32ef).",
                ),
        )
        .arg(
            Arg::with_name("max-count")
                .short("n")
                .takes_value(true)
                .value_name("NUMBER")
                .default_value("20")
                .validator(validator::is_u32)
                .help("Limit the number of changes to output."),
        )
}
