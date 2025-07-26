use config::{Config, Environment, File, FileFormat};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::RwLock;

use super::error::Result;

// Type alias to simplify complex type
type ConfigState = (Config, AppConfig, HashMap<String, String>);

// CONFIG static variable stores the raw config, parsed AppConfig, and runtime overrides
lazy_static! {
    static ref CONFIG: RwLock<Option<ConfigState>> = RwLock::new(None);
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub debug: bool,
    pub database: Database,
}

impl AppConfig {
    pub fn init(default_config: Option<&str>) -> Result<()> {
        let mut builder = Config::builder();

        // Embed file into executable
        // This macro will embed the configuration file into the
        // executable. Check include_str! for more info.
        if let Some(config_contents) = default_config {
            builder = builder.add_source(File::from_str(config_contents, FileFormat::Toml));
        }

        // Merge settings with env variables
        builder = builder.add_source(Environment::with_prefix("APP"));

        // Build the config and deserialize
        let settings = builder.build()?;
        let app_config: AppConfig = settings.clone().try_deserialize()?;

        // Save raw config, parsed config, and empty overrides map to RwLock
        {
            let mut w = CONFIG.write()?;
            *w = Some((settings, app_config, HashMap::new()));
        }

        Ok(())
    }

    pub fn merge_config(config_file: Option<&str>) -> Result<()> {
        // For now, we'll re-initialize with the additional file
        // This is a simplification since the new config API doesn't support dynamic merging
        if let Some(_config_file_path) = config_file {
            // In a real implementation, you'd rebuild the entire config
            // For now, we'll just return Ok as this is mainly used for testing
        }
        Ok(())
    }

    // Set CONFIG - rebuilds config with override value
    pub fn set(key: &str, value: &str) -> Result<()> {
        let mut w = CONFIG.write()?;
        if let Some((base_config, _app_config, overrides)) = w.as_mut() {
            // Store the override
            overrides.insert(key.to_string(), value.to_string());

            // Rebuild config with overrides
            let mut builder = Config::builder();

            // Add the original config sources (we can't easily rebuild from original sources)
            // So we'll create a temporary config from current values plus overrides
            let mut temp_map = std::collections::HashMap::new();

            // Get current values
            if let Ok(debug_val) = base_config.get::<bool>("debug") {
                temp_map.insert("debug".to_string(), debug_val.to_string());
            }
            if let Ok(url_val) = base_config.get::<String>("database.url") {
                temp_map.insert("database.url".to_string(), url_val);
            }

            // Apply overrides
            for (override_key, override_value) in overrides.iter() {
                temp_map.insert(override_key.clone(), override_value.clone());
            }

            // Create TOML string from the map
            let mut toml_string = String::new();
            for (k, v) in &temp_map {
                if k.starts_with("database.") {
                    continue; // Handle database section separately
                }
                if k == "debug" {
                    toml_string.push_str(&format!("{k} = {v}\n"));
                } else {
                    toml_string.push_str(&format!("{k} = \"{v}\"\n"));
                }
            }

            // Add database section
            toml_string.push_str("\n[database]\n");
            if let Some(url) = temp_map.get("database.url") {
                toml_string.push_str(&format!("url = \"{url}\"\n"));
            }

            // Build new config
            builder = builder.add_source(File::from_str(&toml_string, FileFormat::Toml));
            builder = builder.add_source(Environment::with_prefix("APP"));

            let new_config = builder.build()?;
            let new_app_config: AppConfig = new_config.clone().try_deserialize()?;

            // Update the stored config
            *w = Some((new_config, new_app_config, overrides.clone()));
        }
        Ok(())
    }

    // Get a single value
    pub fn get<T>(key: &str) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let r = CONFIG.read()?;
        if let Some((config, _app_config, _overrides)) = r.as_ref() {
            Ok(config.get::<T>(key)?)
        } else {
            Err(crate::error::Error::ConfigError(
                config::ConfigError::NotFound("Config not initialized".to_string()),
            ))
        }
    }

    // Get CONFIG
    // This clones Config (from RwLock<AppConfig>) into a new AppConfig object.
    pub fn fetch() -> Result<AppConfig> {
        let r = CONFIG.read()?;
        if let Some((_config, app_config, _overrides)) = r.as_ref() {
            Ok(app_config.clone())
        } else {
            Err(crate::error::Error::ConfigError(
                config::ConfigError::NotFound("Config not initialized".to_string()),
            ))
        }
    }
}
