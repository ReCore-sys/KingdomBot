use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub(crate) discord_token: String,
    pub(crate) perlin_seed: u32,
}

pub fn get_config() -> Config {
    // Load config from a json and parse it into a Config struct
    let config_string = std::fs::read_to_string("config.json").expect("Failed to read config.json");
    let config: Config = serde_json::from_str(&config_string).unwrap();
    config
}