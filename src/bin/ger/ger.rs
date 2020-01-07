use std::str::FromStr;

use crate::config::Config;
use crate::cli::build_cli;
use crate::commands;


pub struct Ger<'a> {
    pub config: Config,
    pub out: &'a mut dyn std::io::Write,
}

impl<'a> Ger<'a> {
    /// Run commands-line interface
    pub fn run_cli<I, T>(iter_args: I, out: &mut impl std::io::Write) -> Result<(), failure::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let args = build_cli().get_matches_from(iter_args);
        let ger = Ger {
            config: Self::config_from_file(args.value_of("config-file"))?,
            out,
        };
        ger.run_command(args.subcommand())
    }

    /// Set the `config` field from config file
    pub fn config_from_file(config_file: Option<&str>) -> Result<Config, failure::Error> {
        let home_config_file = format!("{}/.ger.toml", dirs::home_dir().unwrap().to_str().unwrap());
        let config_file = config_file.unwrap_or(home_config_file.as_str());
        let config = Config::from_file(config_file)?;
        Ok(config)
    }

    /// Run a commands by dispatching it to its function
    pub fn run_command(
        mut self, cmd_set: (&str, Option<&clap::ArgMatches<'a>>),
    ) -> Result<(), failure::Error> {
        if let Some(exec) = commands::builtin_exec(cmd_set.0) {
            return exec(&mut self.config, cmd_set.1.unwrap());
        }
        Err(failure::err_msg("invalid command"))
    }
}

/**************************************************************************************************/
#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::config::Config;

    #[test]
    fn basic() {
        let ger = super::Ger {
            config: Config {
                remotes: HashMap::new(),
                default_remote: None,
            },
            out: &mut std::io::stdout(),
        };
    }
}
