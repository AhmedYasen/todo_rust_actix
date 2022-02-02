use config::{ConfigError, Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}



#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server_config: ServerConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::default();
        config.merge(File::with_name("config")).unwrap();

        config.try_into()
    }
}