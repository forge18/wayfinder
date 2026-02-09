//! Configuration loading and management
//!
//! This module handles loading configuration from YAML files and merging
//! with command-line arguments.

use home::home_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Runtime to use (e.g., "lua5.1", "lua5.2", "lua5.3", "lua5.4")
    pub runtime: Option<String>,
    /// Whether to stop on entry
    #[serde(rename = "stopOnEntry")]
    pub stop_on_entry: bool,
    /// Current working directory
    pub cwd: Option<String>,
    /// Environment variables
    pub env: Option<HashMap<String, String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            runtime: None,
            stop_on_entry: false,
            cwd: None,
            env: None,
        }
    }
}

/// Internal structure for YAML deserialization
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConfigFile {
    /// Runtime to use (e.g., "lua5.1", "lua5.2", "lua5.3", "lua5.4")
    runtime: Option<String>,
    /// Whether to stop on entry
    #[serde(rename = "stopOnEntry")]
    stop_on_entry: Option<bool>,
    /// Current working directory
    cwd: Option<String>,
    /// Environment variables
    env: Option<HashMap<String, String>>,
}

impl Config {
    /// Load configuration from a YAML file
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path)?;
        let config_file: ConfigFile = serde_yaml::from_str(&content)?;

        Ok(Self {
            runtime: config_file.runtime,
            stop_on_entry: config_file.stop_on_entry.unwrap_or(false),
            cwd: config_file.cwd,
            env: config_file.env,
        })
    }

    /// Find and load configuration from standard locations
    pub fn load_from_standard_locations() -> Result<Option<Self>, Box<dyn std::error::Error>> {
        // Try current directory first
        if let Ok(cwd) = std::env::current_dir() {
            let path = cwd.join("wayfinder.yaml");
            if path.exists() {
                return Ok(Some(Self::load(&path)?));
            }
        }

        // Try home directory
        if let Some(home) = home_dir() {
            let path = home.join(".wayfinder.yaml");
            if path.exists() {
                return Ok(Some(Self::load(&path)?));
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.runtime, None);
        assert_eq!(config.stop_on_entry, false);
        assert_eq!(config.cwd, None);
        assert_eq!(config.env, None);
    }

    #[test]
    fn test_load_config_from_file() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let config_path = temp_dir.path().join("wayfinder.yaml");

        let config_content = r#"
runtime: lua5.4
stopOnEntry: true
cwd: /tmp
env:
  DEBUG: true
  LUA_PATH: ./?.lua
"#;

        fs::write(&config_path, config_content)?;

        let config = Config::load(&config_path)?;

        assert_eq!(config.runtime, Some("lua5.4".to_string()));
        assert_eq!(config.stop_on_entry, true);
        assert_eq!(config.cwd, Some("/tmp".to_string()));

        let env = config.env.unwrap();
        assert_eq!(env.get("DEBUG"), Some(&"true".to_string()));
        assert_eq!(env.get("LUA_PATH"), Some(&"./?.lua".to_string()));

        Ok(())
    }

    #[test]
    fn test_load_config_missing_file() {
        let config = Config::load(Path::new("/nonexistent/config.yaml")).unwrap();
        assert_eq!(config, Config::default());
    }
}
