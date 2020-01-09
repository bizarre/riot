use std::env;
use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub api_key: Option<String>,
    pub region: Option<String>
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::new();

        settings
        // Add in `./Settings.toml`
        // Add in settings from the environment (with a prefix of RIOT)
        // Eg.. `RIOT_API_KEY=xyz` would set the `api_key` key
        .merge(config::Environment::with_prefix("RIOT")).unwrap();

        settings.try_into()
    }
}