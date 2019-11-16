#[macro_use]
extern crate clap;

fn command_change(matches: &clap::ArgMatches) {
    if let Some(change_id) = matches.value_of("id") {
        println!("Change ID: {}", change_id);
    } else {
        println!("List of changes not implemented yet");
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = clap::App::from_yaml(yaml);
    let matches = app.get_matches();

    println!("{:#?}", matches);

    match matches.subcommand_name() {
        Some("change") => command_change(matches.subcommand_matches("change").unwrap()),
        None => {
            println!("{}", matches.usage());
        }
        _ => println!("Command unimplemented"),
    }
}
