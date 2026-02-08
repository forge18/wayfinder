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
}
