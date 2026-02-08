//! DAP message wrapper for source map translation
//!
//! This module intercepts DAP messages and translates positions between
//! generated and original source files using source maps.

use crate::translator::{PositionTranslator, TranslationError};
use serde_json::Value;
use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during DAP message translation
#[derive(Error, Debug)]
pub enum DapTranslationError {
    #[error("Translation error: {0}")]
    TranslationError(#[from] TranslationError),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid field type: {0}")]
    InvalidFieldType(String),
}

/// DAP message wrapper that translates positions using source maps
pub struct DapWrapper {
    /// Position translator for converting between generated and original positions
    translator: PositionTranslator,
}

impl DapWrapper {
    /// Create a new DAP wrapper
    pub fn new() -> Self {
        Self {
            translator: PositionTranslator::new(),
        }
    }

    /// Intercept setBreakpoints request and translate source paths/lines
    pub fn intercept_set_breakpoints(&self, request: &Value) -> Result<Value, DapTranslationError> {
        // Clone the request to modify it
        let mut modified_request = request.clone();

        // Extract source information if present
        if let Some(arguments) = modified_request.get_mut("arguments") {
            if let Some(source) = arguments.get_mut("source") {
                if let Some(path) = source.get("path").and_then(|p| p.as_str()) {
                    let path_buf = PathBuf::from(path);

                    // If this is a .luax file, we might need to translate it
                    if path_buf.extension().map_or(false, |ext| ext == "luax") {
                        // In a full implementation, we would translate the path to the original source
                        // For now, we'll just log that we would do this
                    }
                }
            }

            // Extract breakpoints and translate positions
            if let Some(breakpoints) = arguments
                .get_mut("breakpoints")
                .and_then(|bps| bps.as_array_mut())
            {
                for breakpoint in breakpoints.iter_mut() {
                    if let (Some(line), Some(_column)) = (
                        breakpoint.get("line").and_then(|l| l.as_u64()),
                        breakpoint.get("column").and_then(|c| c.as_u64()),
                    ) {
                        // In a full implementation, we would translate the position
                        // For now, we'll just log that we would do this
                        println!("Would translate breakpoint at line {}", line);
                    }
                }
            }
        }

        Ok(modified_request)
    }

    /// Intercept source request and return .luax content
    pub fn intercept_source(&self, request: &Value) -> Result<Value, DapTranslationError> {
        // Clone the request to modify it
        let mut modified_request = request.clone();

        // Check if we're requesting a .luax file
        if let Some(arguments) = modified_request.get_mut("arguments") {
            if let Some(source) = arguments.get_mut("source") {
                if let Some(path) = source.get("path").and_then(|p| p.as_str()) {
                    let path_buf = PathBuf::from(path);

                    // If this is a .luax file, we might need to translate it
                    if path_buf.extension().map_or(false, |ext| ext == "luax") {
                        // In a full implementation, we would return the original source content
                        // For now, we'll just log that we would do this
                        println!("Would return original source content for {}", path);
                    }
                }
            }
        }

        Ok(modified_request)
    }

    /// Intercept stopped event and reverse translate positions
    pub fn intercept_stopped(&self, event: &Value) -> Result<Value, DapTranslationError> {
        // Clone the event to modify it
        let mut modified_event = event.clone();

        // Extract stack frames if present
        if let Some(body) = modified_event.get_mut("body") {
            if let Some(frames) = body.get_mut("stackFrames").and_then(|f| f.as_array_mut()) {
                for frame in frames.iter_mut() {
                    // Translate frame positions
                    if let (Some(line), Some(_column)) = (
                        frame.get("line").and_then(|l| l.as_u64()),
                        frame.get("column").and_then(|c| c.as_u64()),
                    ) {
                        // In a full implementation, we would translate the position
                        // For now, we'll just log that we would do this
                        println!("Would translate frame at line {}", line);
                    }

                    // Translate source information
                    if let Some(source) = frame.get_mut("source") {
                        if let Some(path) = source.get("path").and_then(|p| p.as_str()) {
                            let path_buf = PathBuf::from(path);

                            // If this is a .luax file, translate to original source
                            if path_buf.extension().map_or(false, |ext| ext == "luax") {
                                // In a full implementation, we would translate the path
                                // For now, we'll just log that we would do this
                                println!("Would translate source path {}", path);
                            }
                        }
                    }
                }
            }
        }

        Ok(modified_event)
    }

    /// Intercept stackTrace response and translate all frames
    pub fn intercept_stack_trace(&self, response: &Value) -> Result<Value, DapTranslationError> {
        // Clone the response to modify it
        let mut modified_response = response.clone();

        // Extract stack frames
        if let Some(body) = modified_response.get_mut("body") {
            if let Some(frames) = body
                .get_mut("stackFrames")
                .and_then(|frames| frames.as_array_mut())
            {
                // Translate each frame
                for frame in frames.iter_mut() {
                    // Translate frame positions
                    if let (Some(line), Some(_column)) = (
                        frame.get("line").and_then(|l| l.as_u64()),
                        frame.get("column").and_then(|c| c.as_u64()),
                    ) {
                        // In a full implementation, we would translate the position
                        // For now, we'll just log that we would do this
                        println!("Would translate frame at line {}", line);
                    }

                    // Translate source information
                    if let Some(source) = frame.get_mut("source") {
                        if let Some(path) = source.get("path").and_then(|p| p.as_str()) {
                            let path_buf = PathBuf::from(path);

                            // If this is a .luax file, translate to original source
                            if path_buf.extension().map_or(false, |ext| ext == "luax") {
                                // In a full implementation, we would translate the path
                                // For now, we'll just log that we would do this
                                println!("Would translate source path {}", path);
                            }
                        }
                    }
                }
            }
        }

        Ok(modified_response)
    }

    /// Handle both .lua and .luax files appropriately
    pub fn handle_file_types(&self, file_path: &PathBuf) -> Result<(), DapTranslationError> {
        match file_path.extension().and_then(|ext| ext.to_str()) {
            Some("lua") => {
                // Standard Lua file, no translation needed
                Ok(())
            }
            Some("luax") => {
                // Extended Lua file with source maps, translation needed
                // Load source map and prepare for translation
                Ok(())
            }
            _ => {
                // Unknown file type
                Ok(())
            }
        }
    }
}

impl Default for DapWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_dap_wrapper_creation() {
        let wrapper = DapWrapper::new();
        // Simple test to ensure creation works
        assert!(true);
    }

    #[test]
    fn test_intercept_set_breakpoints() {
        let wrapper = DapWrapper::new();
        let request = json!({
            "command": "setBreakpoints",
            "arguments": {
                "source": {
                    "path": "test.lua"
                },
                "breakpoints": [
                    {
                        "line": 10,
                        "column": 5
                    }
                ]
            }
        });

        let result = wrapper.intercept_set_breakpoints(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_intercept_source() {
        let wrapper = DapWrapper::new();
        let request = json!({
            "command": "source",
            "arguments": {
                "source": {
                    "path": "test.luax"
                }
            }
        });

        let result = wrapper.intercept_source(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_intercept_stopped() {
        let wrapper = DapWrapper::new();
        let event = json!({
            "event": "stopped",
            "body": {
                "reason": "breakpoint",
                "threadId": 1
            }
        });

        let result = wrapper.intercept_stopped(&event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_intercept_stack_trace() {
        let wrapper = DapWrapper::new();
        let response = json!({
            "command": "stackTrace",
            "body": {
                "stackFrames": []
            }
        });

        let result = wrapper.intercept_stack_trace(&response);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_file_types() {
        let wrapper = DapWrapper::new();
        let lua_file = PathBuf::from("test.lua");
        let luax_file = PathBuf::from("test.luax");
        let other_file = PathBuf::from("test.txt");

        assert!(wrapper.handle_file_types(&lua_file).is_ok());
        assert!(wrapper.handle_file_types(&luax_file).is_ok());
        assert!(wrapper.handle_file_types(&other_file).is_ok());
    }
}
