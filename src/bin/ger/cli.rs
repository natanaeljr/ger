use crate::commands;
use crate::config::{CliConfig, UserConfig};
use clap::{App, AppSettings, Arg};
use termcolor::{ColorChoice, StandardStream};

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
    let mut config = CliConfig {
        user_cfg: UserConfig::from_file(args.value_of("config-file"))?,
        stdout: StandardStream::stdout(match atty::is(atty::Stream::Stdout) {
            true => ColorChoice::Auto,
            false => ColorChoice::Never,
        }),
    };
    execute_subcommand(&mut config, args.subcommand())
}

/**************************************************************************************************/
/// Execute subcommand by dispatching it to its handling function
fn execute_subcommand(
    config: &mut CliConfig, cmd_args: (&str, Option<&clap::ArgMatches>),
) -> Result<(), failure::Error> {
    if let Some(exec) = commands::builtin_exec(cmd_args.0) {
        return exec(config, cmd_args.1);
    }
    Err(failure::err_msg("invalid command"))
}
