use clap::{App, AppSettings, Arg, SubCommand};

/**************************************************************************************************/
/// Build GER Clap App
pub fn build_cli() -> App<'static, 'static> {
    App::new("ger")
        .version("0.1.0")
        .about("Gerrit commands-line interface.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::DontCollapseArgsInUsage)
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
        .subcommands(crate::commands::builtin())
}

/**************************************************************************************************/
/// Validate a string is convertible to u32
pub fn is_u32(v: String) -> Result<(), String> {
    if v.parse::<u32>().is_ok() { return Ok(()); }
    Err(String::from("not a number"))
}
