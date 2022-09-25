use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;
#[derive(Deserialize, Debug)]
pub struct Data {
    channels: Vec<Channel>,
}
#[derive(Deserialize, Debug)]
pub struct Channel {
    name: String,
    api_key: String,
}
pub(crate) fn read_config() -> Data {
    let filename = "config.toml";
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };
    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };
    data
}
