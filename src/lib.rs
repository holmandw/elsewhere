use std::fs;
use std::path::PathBuf;

use chrono::Utc;
use chrono_tz::Tz;
use dirs::home_dir;
use serde::{Deserialize, Serialize};

static CONFIG_FILENAME: &'static str = ".ew.toml";
static DEFAULT_STRFTIME: &'static str = "%H:%M %z    %a %b %d %Y";

pub struct App {
    config: TomlConfig,
    config_file: PathBuf,
}

impl App {
    /// Creates a new `App` instance with either
    /// the values stored in the config file or
    /// the default file location and config values.
    /// If it doesn't exist, this creates a file at
    /// `$HOME/.ew.toml` and populates default values.
    pub fn new() -> App {
        let pb = Self::get_default_pb();
        let config = TomlConfig::get_or_create(&pb);
        App {
            config,
            config_file: pb,
        }
    }

    /// Add a new person to the config.
    pub fn add(&mut self, p: Person) {
        self.config.add(p);
        self.write();
    }

    pub fn rm(&mut self, name: &str) {
        self.config.remove(name);
        self.write();
    }

    /// Print out all of the entries. Self is mutably borrowed
    /// so the vector can be sorted if applicable.
    pub fn list(&mut self) {
        // TODO: return a value, let something else write to stdout
        let now = Utc::now();
        let buff_size = self
            .config
            .person
            .iter()
            .map(|p| p.name.len())
            .max()
            .unwrap_or(0)
            + 4;
        if self.config.person.len() == 0 {
            println!("no entries!");
            return;
        }

        if self.config.should_sort() {
            self.config.sort()
        }

        let fmt = self
            .config
            .date_fmt
            .clone()
            .unwrap_or((&DEFAULT_STRFTIME).to_string());

        self.config.person.iter().for_each(|p| {
            if let Ok::<Tz, _>(tz) = p.tz.parse() {
                let now_local = now.with_timezone(&tz).format(&fmt);
                println!("{:buff_size$}{}", p.name, now_local);
            } else {
                println!("invalid tz {} for {}", p.tz, p.name);
            }
        });
    }

    fn write(&self) {
        self.config.write(&self.config_file);
    }

    fn get_default_pb() -> PathBuf {
        [home_dir().unwrap(), CONFIG_FILENAME.into()]
            .iter()
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TomlConfig {
    person: Vec<Person>,
    date_fmt: Option<String>,
    sort: Option<bool>,
}

impl TomlConfig {
    fn empty() -> TomlConfig {
        TomlConfig {
            person: Vec::new(),
            date_fmt: Some(DEFAULT_STRFTIME.into()),
            sort: Some(true),
        }
    }

    pub fn write(&self, pb: &PathBuf) {
        let toml_s = toml::to_string(&self).unwrap();
        fs::write(&pb, toml_s).unwrap();
    }
    /// Either get the contents of a config or create an empty config.
    pub fn get_or_create(pb: &PathBuf) -> TomlConfig {
        match fs::exists(&pb) {
            Ok(true) => {
                let contents = fs::read_to_string(&pb).unwrap();
                if let Ok(config) = toml::from_str(&contents) {
                    config
                } else {
                    TomlConfig::empty()
                }
            }
            Ok(false) => {
                println!("writing default config to `{}`!", pb.display());
                let config = TomlConfig::empty();
                let s: String = toml::to_string(&config).unwrap();
                fs::write(&pb, s).unwrap();
                config
            }
            Err(err) => panic!("{err}"),
        }
    }

    fn sort(&mut self) {
        self.person.sort_by(|p1, p2| p1.name.cmp(&p2.name));
    }

    fn should_sort(&self) -> bool {
        match self.sort {
            Some(true) => true,
            Some(false) => false,
            None => true,
        }
    }

    fn add(&mut self, p: Person) {
        self.person.push(p);

        if self.should_sort() {
            self.sort();
        }
    }

    fn remove(&mut self, name: &str) {
        if let Some(idx) = self
            .person
            .iter()
            .position(|p| p.name.to_lowercase() == name.to_lowercase())
        {
            self.person.swap_remove(idx);
        }
        if self.should_sort() {
            self.sort();
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    name: String,
    tz: String,
}

impl Person {
    pub fn new(name: &str, tz: &str) -> Person {
        Person {
            name: name.into(),
            tz: tz.into(),
        }
    }
}
