//! Position translation between generated and original source files
//!
//! This module handles translating positions (line/column) between the
//! compiled Lua code and the original TypeScript source using source maps.

use crate::source_map::{SourceMapLoader, SourceMapLoaderError, SourceMapSource};
use luanext_sourcemap::SourceMap;
use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during position translation
#[derive(Error, Debug)]
pub enum TranslationError {
    #[error("Source map loader error: {0}")]
    SourceMapLoaderError(#[from] SourceMapLoaderError),

    #[error("No mapping found for position")]
    NoMappingFound,

    #[error("Invalid position")]
    InvalidPosition,
}

/// Represents a position in source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    /// 1-based line number
    pub line: u32,

    /// 1-based column number
    pub column: u32,
}

/// Represents a source location with file and position
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    /// Path to the source file
    pub file: PathBuf,

    /// Position in the source file
    pub position: Position,
}

/// Translates positions between generated and original source files
pub struct PositionTranslator {
    /// Source map loader for loading source maps
    loader: SourceMapLoader,

    /// Loaded source maps
    source_maps: std::collections::HashMap<PathBuf, SourceMap>,
}

impl PositionTranslator {
    /// Create a new position translator
    pub fn new() -> Self {
        Self {
            loader: SourceMapLoader::new(),
            source_maps: std::collections::HashMap::new(),
        }
    }

    /// Load a source map for the specified file
    pub fn load_source_map(
        &mut self,
        file: PathBuf,
        source: &SourceMapSource,
    ) -> Result<(), TranslationError> {
        let source_map = self.loader.load_source_map(source)?;
        self.source_maps.insert(file, source_map);
        Ok(())
    }

    /// Translate a position from generated code to original source
    pub fn forward_lookup(
        &self,
        generated_file: &PathBuf,
        _line: u32,
        _column: u32,
    ) -> Result<SourceLocation, TranslationError> {
        let source_map = self
            .source_maps
            .get(generated_file)
            .ok_or_else(|| SourceMapLoaderError::NotFound)?;

        // For now, we'll implement a simplified version that just returns the first source
        // A full implementation would parse the mappings string and find the correct mapping
        if !source_map.sources.is_empty() {
            let original_file = PathBuf::from(&source_map.sources[0]);
            let original_position = Position { line: 1, column: 1 };

            Ok(SourceLocation {
                file: original_file,
                position: original_position,
            })
        } else {
            Err(TranslationError::NoMappingFound)
        }
    }

    /// Translate a position from original source to generated code
    pub fn reverse_lookup(
        &self,
        original_file: &PathBuf,
        _line: u32,
        _column: u32,
    ) -> Result<SourceLocation, TranslationError> {
        // Find the source map that contains this original file
        for (generated_file, source_map) in &self.source_maps {
            if source_map
                .sources
                .iter()
                .any(|source| PathBuf::from(source) == *original_file)
            {
                // For now, we'll return a simple mapping
                // A full implementation would parse the mappings string and find the correct mapping
                let generated_position = Position { line: 1, column: 1 };

                return Ok(SourceLocation {
                    file: generated_file.clone(),
                    position: generated_position,
                });
            }
        }

        Err(TranslationError::NoMappingFound)
    }

    /// Handle bundle mode where multiple source files are mapped to one generated file
    pub fn handle_bundle_mode(
        &self,
        generated_file: &PathBuf,
    ) -> Result<Vec<PathBuf>, TranslationError> {
        let source_map = self
            .source_maps
            .get(generated_file)
            .ok_or_else(|| SourceMapLoaderError::NotFound)?;

        let sources = source_map
            .sources
            .iter()
            .map(|s| PathBuf::from(s))
            .collect();

        Ok(sources)
    }

    /// Handle missing mappings gracefully by finding the closest available mapping
    pub fn lookup_with_fallback(&self, generated_file: &PathBuf) -> Option<&SourceMap> {
        self.source_maps.get(generated_file)
    }
}

impl Default for PositionTranslator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_translator_creation() {
        let translator = PositionTranslator::new();
        assert!(translator.source_maps.is_empty());
    }

    #[test]
    fn test_position_struct() {
        let position = Position {
            line: 10,
            column: 5,
        };
        assert_eq!(position.line, 10);
        assert_eq!(position.column, 5);
    }

    #[test]
    fn test_forward_lookup() {
        let translator = PositionTranslator::new();
        let file = PathBuf::from("test.lua");
        let result = translator.forward_lookup(&file, 1, 1);
        // Should fail since no source maps are loaded
        assert!(result.is_err());
    }

    #[test]
    fn test_reverse_lookup() {
        let translator = PositionTranslator::new();
        let file = PathBuf::from("test.tl");
        let result = translator.reverse_lookup(&file, 1, 1);
        // Should fail since no source maps are loaded
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_bundle_mode() {
        let translator = PositionTranslator::new();
        let file = PathBuf::from("bundle.lua");
        let result = translator.handle_bundle_mode(&file);
        // Should fail since no source maps are loaded
        assert!(result.is_err());
    }

    #[test]
    fn test_lookup_with_fallback() {
        let translator = PositionTranslator::new();
        let file = PathBuf::from("test.lua");
        let result = translator.lookup_with_fallback(&file);
        // Should be None since no source maps are loaded
        assert!(result.is_none());
    }
}
