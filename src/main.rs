use std::fs;
use std::path::PathBuf;

use chrono::Utc;
use chrono_tz::Tz;
use clap::{arg, Command};
use dirs::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    person: Vec<Person>,
}

impl Config {
    fn empty() -> Config {
        Config { person: Vec::new() }
    }

    /// Either get the contents of a config or create an empty config.
    fn get_or_create_config() -> Config {
        let pb = get_pb();
        match fs::exists(&pb) {
            Ok(true) => {
                let contents = fs::read_to_string(&pb).unwrap();
                if let Ok(config) = toml::from_str(&contents) {
                    config
                } else {
                    Config::empty()
                }
            }
            Ok(false) => {
                fs::write(&pb, b"").unwrap();
                Config::empty()
            }
            Err(err) => panic!("{err}"),
        }
    }
    fn sorted(&self) -> Self {
        let mut copy = self.person.clone();
        copy.sort_by(|p1, p2| p1.name.cmp(&p2.name));
        Config { person: copy }
    }

    fn add(&mut self, p: Person, sort: bool) {
        self.person.push(p);
        if sort {
            self.person = self.sorted().person;
        }
    }

    fn remove(&mut self, name: &str, sort: bool) {
        if let Some(idx) = self.person.iter().position(|p| p.name == name) {
            self.person.swap_remove(idx);
        }
        if sort {
            self.person = self.sorted().person;
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Person {
    name: String,
    tz: String,
}

impl Person {
    fn new(name: &str, tz: &str) -> Person {
        Person {
            name: name.into(),
            tz: tz.into(),
        }
    }
}

static CONFIG_FILENAME: &'static str = ".ew.toml";

fn get_pb() -> PathBuf {
    [home_dir().unwrap(), CONFIG_FILENAME.into()]
        .iter()
        .collect()
}

fn add_person(person: Person, sort: bool) {
    let mut config = Config::get_or_create_config();
    config.add(person, sort);
    let toml_s = toml::to_string(&config).unwrap();
    let pb = get_pb();
    fs::write(pb, toml_s).unwrap();
}

fn remove_by_name(name: &str, sort: bool) {
    let mut config = Config::get_or_create_config();
    config.remove(name, sort);
    let toml_s = toml::to_string(&config).unwrap();
    let pb = get_pb();
    fs::write(pb, toml_s).unwrap();
}

fn run() {
    let config = Config::get_or_create_config();
    let now = Utc::now();
    let buff_size = config
        .person
        .iter()
        .map(|p| p.name.len())
        .max()
        .unwrap_or(0)
        + 4;
    if config.person.len() == 0 {
        println!("no entries!");
        return;
    }
    for p in config.person {
        let n = p.name;
        let tz: Tz = p.tz.parse().expect("invalid timezone");
        let now_local = now.with_timezone(&tz).format("%H:%M %z\t%a %b %d %Y");
        println!("{n:buff_size$}{now_local}");
    }
}

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
            let name = sub_matches.get_one::<String>("name").map(|s| s.as_str());
            let tz = sub_matches.get_one::<String>("tz").map(|s| s.as_str());
            name.zip(tz)
                .map(|(n, t)| Person::new(n, t))
                .map(|p| add_person(p, true));
        }
        Some(("rm", sub_matches)) => {
            sub_matches
                .get_one::<String>("name")
                .map(|s| s.as_str())
                .map(|n| remove_by_name(n, true));
        }
        None => run(),
        _ => unreachable!(),
    };
}
