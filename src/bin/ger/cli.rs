use clap::{App, AppSettings, Arg, SubCommand};

/**************************************************************************************************/
/// Build GER Clap App
pub fn build_cli() -> App<'static, 'static> {
    App::new("ger")
        .version("0.1.0")
        .about("Gerrit command-line interface.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::DontCollapseArgsInUsage)
        .global_setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name("config-file")
                .long("config-file")
                .takes_value(true)
                .value_name("FILE")
                .help("Alternative TOML configuration filepath."),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .global(true)
                .multiple(true)
                .help("Set level of verbosity (up to -vvv)"),
        )
        .subcommands(crate::commands::builtin())
        .template("{about}\n\n{usage}\n\n{all-args}")
}
