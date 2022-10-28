use serde_derive::Deserialize;
use std::{fmt, fs};
use std::process::exit;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub plugin: Vec<Plugin>,
    pub web: WebConfig,
}

#[derive(Deserialize, Debug)]
pub struct Plugin {
    pub name: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct WebConfig {
    pub user: String,
    pub password: String,
}

pub(crate) fn read_config() -> Config {
    let filename = "config.toml";
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };
    let data: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };
    data
}
