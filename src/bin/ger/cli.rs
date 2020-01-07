use crate::commands;
use crate::config::UserConfig;
use clap::{App, AppSettings, Arg, SubCommand};
use std::str::FromStr;

/**************************************************************************************************/
/// Build GER Clap App
pub fn cli() -> App<'static, 'static> {
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

/**************************************************************************************************/
/// Execute GER CLI
pub fn main<I, T>(iter_args: I) -> Result<(), failure::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let args = cli().get_matches_from(iter_args);
    let mut config = config_from_file(args.value_of("config-file"))?;
    execute_subcommand(&mut config, args.subcommand())
}

/**************************************************************************************************/
/// Read user config from TOML config file
fn config_from_file(config_file: Option<&str>) -> Result<UserConfig, failure::Error> {
    let default_config_file = format!("{}/.ger.toml", dirs::home_dir()?.to_str()?);
    let config_file = config_file.unwrap_or(default_config_file.as_str());
    let config = UserConfig::from_file(config_file)?;
    Ok(config)
}

/**************************************************************************************************/
/// Execute subcommand by dispatching it to its handling function
fn execute_subcommand(
    config: &mut UserConfig, cmd_args: (&str, Option<&clap::ArgMatches>),
) -> Result<(), failure::Error> {
    if let Some(exec) = commands::builtin_exec(cmd_args.0) {
        return exec(config, cmd_args.1.unwrap());
    }
    Err(failure::err_msg("invalid command"))
}
