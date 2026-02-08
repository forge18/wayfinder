//! Configuration for source map behavior
//!
//! This module handles configuration options related to source map handling,
//! including user preferences for how to deal with missing or malformed source maps.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur with source map configuration
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),

    #[error("Configuration not found: {0}")]
    NotFound(String),
}

/// Behavior when encountering source maps
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceMapBehavior {
    /// Ask the user what to do when a source map is missing
    Ask,

    /// Continue debugging with .lua files only when source maps are missing
    Lenient,

    /// Stop debugging with an error when source maps are missing
    Strict,
}

/// User preferences for source map handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMapPreferences {
    /// Default behavior for handling source maps
    pub default_behavior: SourceMapBehavior,

    /// Per-file behavior overrides
    pub file_overrides: HashMap<String, SourceMapBehavior>,

    /// Whether to persist user preferences
    pub persist_preferences: bool,
}

impl SourceMapPreferences {
    /// Create new source map preferences with default values
    pub fn new() -> Self {
        Self {
            default_behavior: SourceMapBehavior::Ask,
            file_overrides: HashMap::new(),
            persist_preferences: true,
        }
    }

    /// Get the behavior for a specific file
    pub fn get_behavior_for_file(&self, file_path: &str) -> &SourceMapBehavior {
        self.file_overrides
            .get(file_path)
            .unwrap_or(&self.default_behavior)
    }

    /// Set the behavior for a specific file
    pub fn set_behavior_for_file(&mut self, file_path: String, behavior: SourceMapBehavior) {
        self.file_overrides.insert(file_path, behavior);
    }

    /// Set the default behavior
    pub fn set_default_behavior(&mut self, behavior: SourceMapBehavior) {
        self.default_behavior = behavior;
    }

    /// Enable or disable preference persistence
    pub fn set_persist_preferences(&mut self, persist: bool) {
        self.persist_preferences = persist;
    }

    /// Implement "ask" behavior (prompt user)
    pub fn should_ask_user(&self, file_path: &str) -> bool {
        matches!(
            self.get_behavior_for_file(file_path),
            SourceMapBehavior::Ask
        )
    }

    /// Implement "lenient" behavior (debug .lua only)
    pub fn is_lenient_mode(&self, file_path: &str) -> bool {
        matches!(
            self.get_behavior_for_file(file_path),
            SourceMapBehavior::Lenient
        )
    }

    /// Implement "strict" behavior (error if missing)
    pub fn is_strict_mode(&self, file_path: &str) -> bool {
        matches!(
            self.get_behavior_for_file(file_path),
            SourceMapBehavior::Strict
        )
    }
}

impl Default for SourceMapPreferences {
    fn default() -> Self {
        Self::new()
    }
}

/// Global configuration for the source map integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Source map preferences
    pub source_map_preferences: SourceMapPreferences,

    /// Whether to enable coroutine debugging
    pub enable_coroutine_debugging: bool,

    /// Whether to enable bundle mode support
    pub enable_bundle_mode: bool,
}

impl Config {
    /// Create new configuration with default values
    pub fn new() -> Self {
        Self {
            source_map_preferences: SourceMapPreferences::new(),
            enable_coroutine_debugging: true,
            enable_bundle_mode: true,
        }
    }

    /// Load configuration from a file or other source
    pub fn load() -> Result<Self, ConfigError> {
        // In a real implementation, this would load from a file
        // For now, we'll just return the default configuration
        Ok(Self::new())
    }

    /// Save configuration to a file or other destination
    pub fn save(&self) -> Result<(), ConfigError> {
        // In a real implementation, this would save to a file
        // For now, we'll just return Ok
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::new();
        assert_eq!(
            config.source_map_preferences.default_behavior,
            SourceMapBehavior::Ask
        );
        assert!(config.enable_coroutine_debugging);
        assert!(config.enable_bundle_mode);
    }

    #[test]
    fn test_source_map_behavior_enum() {
        let ask = SourceMapBehavior::Ask;
        let lenient = SourceMapBehavior::Lenient;
        let strict = SourceMapBehavior::Strict;

        assert_ne!(ask, lenient);
        assert_ne!(lenient, strict);
        assert_ne!(strict, ask);
    }

    #[test]
    fn test_preferences_behavior_lookup() {
        let mut preferences = SourceMapPreferences::new();
        assert_eq!(
            preferences.get_behavior_for_file("test.lua"),
            &SourceMapBehavior::Ask
        );

        preferences.set_behavior_for_file("test.lua".to_string(), SourceMapBehavior::Strict);
        assert_eq!(
            preferences.get_behavior_for_file("test.lua"),
            &SourceMapBehavior::Strict
        );
        assert_eq!(
            preferences.get_behavior_for_file("other.lua"),
            &SourceMapBehavior::Ask
        );
    }
}
