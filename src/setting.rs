use serde::Deserialize;
use config::ConfigError;
use config::Config;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32
}

#[derive(Debug, Deserialize)]
pub struct Setting {
    pub server: ServerConfig
}

impl Setting {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(config::Environment::new())?;
        s.try_into()
    }
}
