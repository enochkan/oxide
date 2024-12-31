use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub api_port: u16,
}

impl Config {
    pub fn load() -> Self {
        dotenv::dotenv().ok();
        let config_str = std::fs::read_to_string("Config.toml").expect("Failed to read config file");
        toml::from_str(&config_str).expect("Failed to parse config")
    }
}
