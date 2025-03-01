use clap::{arg, Command};

use ew::{App, Person};

fn main() {
    let cmd = Command::new("ew")
        .bin_name("ew")
        .about("Elsewhere `ew` helps keep track of what time it is elsewhere")
        .subcommand_required(false)
        .subcommand(
            Command::new("add")
                .about("Add a new person to the `ew` config")
                .arg(arg!(name: [NAME]))
                .arg(arg!(tz: [TIMEZONE])),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a person from the `ew` config by name")
                .arg(arg!(name: [NAME])),
        );
    match cmd.get_matches().subcommand() {
        Some(("add", sub_matches)) => {
            sub_matches
                .get_one::<String>("name")
                .map(|s| s.as_str())
                .zip(sub_matches.get_one::<String>("tz").map(|s| s.as_str()))
                .map(|(n, t)| Person::new(n, t))
                .zip(Some(App::new()))
                .map(|(p, mut app)| app.add(p));
        }
        Some(("rm", sub_matches)) => {
            sub_matches
                .get_one::<String>("name")
                .map(|s| s.as_str())
                .zip(Some(App::new()))
                .map(|(n, mut app)| app.rm(n));
        }
        None => App::new().list(),
        _ => unreachable!(),
    };
}
