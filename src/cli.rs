use crate::commands;
use crate::config::{CliConfig, UserConfig};
use clap::{App, AppSettings, Arg, ArgMatches};
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
            Arg::with_name("color")
                .long("color")
                .env("GER_COLOR")
                .visible_alias("colour")
                .takes_value(true)
                .value_name("when")
                .require_equals(true)
                .possible_values(&["auto", "always", "never"])
                .hide_env_values(true)
                .help("Control when to use colors on output."),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .global(true)
                .multiple(true)
                .help("Set level of verbosity to output (up to -vvv)"),
        )
        .subcommands(crate::commands::builtin())
        .template("{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")
}

/**************************************************************************************************/
/// Execute GER CLI
pub fn main<I, T>(iter_args: I) -> Result<(), failure::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let args = cli().get_matches_from(iter_args);
    let mut config = configure(&args)?;
    execute_subcommand(&mut config, args.subcommand())
}

/**************************************************************************************************/
/// Configure CLI running settings
fn configure(args: &ArgMatches) -> Result<CliConfig, failure::Error> {
    let config = CliConfig {
        user: UserConfig::from_file(std::env::var("GER_CONFIG").ok())?,
        stdout: StandardStream::stdout(match args.value_of("color") {
            Some("always") => ColorChoice::Always,
            Some("never") => ColorChoice::Never,
            Some(_) | None => match atty::is(atty::Stream::Stdout) {
                true => ColorChoice::Auto,
                false => ColorChoice::Never,
            },
        }),
    };
    Ok(config)
}

/**************************************************************************************************/
/// Execute subcommand by dispatching it to its handling function
fn execute_subcommand(
    config: &mut CliConfig, cmd_args: (&str, Option<&ArgMatches>),
) -> Result<(), failure::Error> {
    if let Some(exec) = commands::builtin_exec(cmd_args.0) {
        return exec(config, cmd_args.1);
    }
    Err(failure::err_msg("invalid command"))
}
