use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

use crate::errors::Errors;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub title: String,
    pub description: String,
    pub long_description: String,
    pub library_version: String,
}

/// Get the config from the llmstxt.toml file formatted as a HashMap.
pub fn get_config(dir: &Path) -> Result<Config> {
    let config_path = dir.join("llmstxt.toml");
    let config_path_str = config_path.to_str().unwrap();
    let config = config::Config::builder()
        .add_source(config::File::with_name(config_path_str))
        .build()
        .context(Errors::ReadConfig(config_path.clone()))?;

    config
        .try_deserialize::<Config>()
        .context(Errors::DeserializeConfig(config_path))
}
