use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::{env, fs};


#[derive(Debug, Deserialize)]
pub struct TransactionSourceSettings {
    pub dir: String
}
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub transaction_source: TransactionSourceSettings
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("ENV").unwrap_or_else(|_| "local".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("env.default.toml"))
            // Add in the current environment file for any env overrides
            .add_source(File::with_name(&format!("env.{}.toml", env)).required(false))
            // Apply any user overrides through environment
            .add_source(Environment::default())
            .build()?;

        s.try_deserialize()
    }
}