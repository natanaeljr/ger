#[macro_use]
extern crate clap;

fn command_change(args: Option<&clap::ArgMatches>) {}

fn command_project(args: Option<&clap::ArgMatches>) {}

fn command_config(args: Option<&clap::ArgMatches>) {}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let args = clap::App::from_yaml(yaml).get_matches();

    match args.subcommand() {
        ("change", subargs) => command_change(subargs),
        ("project", subargs) => command_project(subargs),
        ("config", subargs) => command_config(subargs),
        _ => (),
    }
}
