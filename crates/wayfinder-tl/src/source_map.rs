//! Source map loading and representation
//!
//! This module handles loading source maps from various sources including
//! external files, inline comments, and data URIs.

use base64::Engine;
use luanext_sourcemap::SourceMap;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur when loading or processing source maps
#[derive(Error, Debug)]
pub enum SourceMapLoaderError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid source map source: {0}")]
    InvalidSource(String),

    #[error("Source map not found")]
    NotFound,
}

/// Represents the source of a source map
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceMapSource {
    /// Source map stored in an external file
    File(PathBuf),

    /// Source map embedded inline in the source file
    Inline(String),

    /// Source map encoded as a data URI
    DataUri(String),
}

/// Source map loader responsible for loading and caching source maps
pub struct SourceMapLoader {
    /// Cache of loaded source maps to avoid reloading
    cache: HashMap<PathBuf, SourceMap>,
}

impl SourceMapLoader {
    /// Create a new source map loader
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Load a source map from the specified source
    pub fn load_source_map(
        &mut self,
        source: &SourceMapSource,
    ) -> Result<SourceMap, SourceMapLoaderError> {
        match source {
            SourceMapSource::File(path) => {
                // Check cache first
                if let Some(cached) = self.cache.get(path) {
                    // Since SourceMap doesn't implement Clone, we need to serialize/deserialize
                    let serialized = serde_json::to_string(cached)?;
                    let deserialized: SourceMap = serde_json::from_str(&serialized)?;
                    return Ok(deserialized);
                }

                // Load from file
                let content = std::fs::read_to_string(path)?;
                let source_map: SourceMap = serde_json::from_str(&content)?;
                // Store in cache by serializing and deserializing
                let serialized = serde_json::to_string(&source_map)?;
                let cached: SourceMap = serde_json::from_str(&serialized)?;
                self.cache.insert(path.clone(), cached);
                Ok(source_map)
            }

            SourceMapSource::Inline(content) => {
                // Parse inline source map
                let source_map: SourceMap = serde_json::from_str(content)?;
                Ok(source_map)
            }

            SourceMapSource::DataUri(uri) => {
                // Parse data URI
                if !uri.starts_with("data:application/json;base64,") {
                    return Err(SourceMapLoaderError::InvalidSource(
                        "Unsupported data URI format".to_string(),
                    ));
                }

                let encoded = &uri["data:application/json;base64,".len()..];
                let decoded = base64::engine::general_purpose::STANDARD
                    .decode(encoded)
                    .map_err(|_| {
                        SourceMapLoaderError::InvalidSource("Invalid base64 encoding".to_string())
                    })?;

                let content = String::from_utf8(decoded).map_err(|_| {
                    SourceMapLoaderError::InvalidSource("Invalid UTF-8 encoding".to_string())
                })?;

                let source_map: SourceMap = serde_json::from_str(&content)?;
                Ok(source_map)
            }
        }
    }

    /// Extract inline source map from source file content
    pub fn extract_inline_source_map(source_content: &str) -> Option<String> {
        // Look for sourceMappingURL comment
        for line in source_content.lines().rev() {
            if let Some(pos) = line.find("sourceMappingURL=") {
                let url = &line[pos + "sourceMappingURL=".len()..];
                return Some(url.to_string());
            }
        }
        None
    }

    /// Clear the source map cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for SourceMapLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_map_loader_creation() {
        let loader = SourceMapLoader::new();
        assert!(loader.cache.is_empty());
    }

    #[test]
    fn test_source_map_source_enum() {
        let file_source = SourceMapSource::File(PathBuf::from("test.map"));
        let inline_source = SourceMapSource::Inline("inline content".to_string());
        let data_uri_source =
            SourceMapSource::DataUri("data:application/json;base64,encoded".to_string());

        assert_ne!(file_source, inline_source);
        assert_ne!(inline_source, data_uri_source);
    }

    #[test]
    fn test_extract_inline_source_map() {
        let content = r#"local x = 1
local y = 2
--# sourceMappingURL=test.map"#;

        let result = SourceMapLoader::extract_inline_source_map(content);
        assert_eq!(result, Some("test.map".to_string()));
    }

    #[test]
    fn test_extract_inline_source_map_not_found() {
        let content = r#"local x = 1
local y = 2"#;

        let result = SourceMapLoader::extract_inline_source_map(content);
        assert_eq!(result, None);
    }

    #[test]
    fn test_clear_cache() {
        let mut loader = SourceMapLoader::new();
        assert!(loader.cache.is_empty());
        loader.clear_cache();
        assert!(loader.cache.is_empty());
    }
}
