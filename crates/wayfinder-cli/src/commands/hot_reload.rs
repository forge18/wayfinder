//! Hot reload command implementation
//!
//! This module handles sending hot reload requests to a running DAP server.

use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use serde_json::{json, Value as JsonValue};

/// Hot reload configuration
#[derive(Debug)]
pub struct HotReloadConfig {
    /// Module name to reload
    pub module: String,
    /// Host to connect to
    pub host: String,
    /// Port to connect to (None for stdio mode)
    pub port: Option<u16>,
}

/// Send a hot reload request to the DAP server
pub async fn send_hot_reload(config: HotReloadConfig) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(port) = config.port {
        // Connect via TCP
        send_hot_reload_tcp(config.module, config.host, port).await
    } else {
        // Use stdio mode (for direct communication)
        send_hot_reload_stdio(config.module).await
    }
}

/// Send hot reload via TCP connection
async fn send_hot_reload_tcp(module: String, host: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = format!("{}:{}", host, port).parse()?;

    eprintln!("Connecting to DAP server at {}...", address);

    // Attempt to connect with a timeout
    let stream = match tokio::time::timeout(Duration::from_secs(5), TcpStream::connect(&address)).await {
        Ok(Ok(stream)) => stream,
        Ok(Err(e)) => {
            return Err(format!("Failed to connect to DAP server: {}", e).into());
        }
        Err(_) => {
            return Err("Connection timeout - is the DAP server running?".into());
        }
    };

    eprintln!("✓ Connected to DAP server");

    // Split the stream for reading and writing
    let (read_half, write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);
    let mut writer = write_half;

    // Create the hotReload DAP request
    let request = json!({
        "seq": 1,
        "type": "request",
        "command": "hotReload",
        "arguments": {
            "module": module
        }
    });

    eprintln!("Sending hot reload request for module: {}", module);

    // Send the request
    write_dap_message(&mut writer, &request).await?;

    // Wait for the response
    eprintln!("Waiting for response...");
    match tokio::time::timeout(Duration::from_secs(10), read_dap_message(&mut reader)).await {
        Ok(Ok(response)) => {
            eprintln!("✓ Received response from DAP server");

            // Check if the request was successful
            if let Some(success) = response.get("success").and_then(|v| v.as_bool()) {
                if success {
                    println!("✓ Hot reload successful for module: {}", module);

                    // Display any warnings if present
                    if let Some(body) = response.get("body") {
                        if let Some(warnings) = body.get("warnings").and_then(|v| v.as_array()) {
                            if !warnings.is_empty() {
                                println!("\nWarnings:");
                                for warning in warnings {
                                    if let Some(msg) = warning.as_str() {
                                        println!("  ⚠ {}", msg);
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // Extract error message
                    let error_msg = response.get("message")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error");
                    eprintln!("✗ Hot reload failed: {}", error_msg);
                    return Err(format!("Hot reload failed: {}", error_msg).into());
                }
            } else {
                eprintln!("✗ Invalid response from DAP server");
                return Err("Invalid response format".into());
            }
        }
        Ok(Err(e)) => {
            return Err(format!("Error reading response: {}", e).into());
        }
        Err(_) => {
            return Err("Timeout waiting for response from DAP server".into());
        }
    }

    Ok(())
}

/// Send hot reload via stdio (for direct process communication)
async fn send_hot_reload_stdio(module: String) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Sending hot reload request via stdio...");

    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();

    // Create the hotReload DAP request
    let request = json!({
        "seq": 1,
        "type": "request",
        "command": "hotReload",
        "arguments": {
            "module": module
        }
    });

    eprintln!("Sending hot reload request for module: {}", module);

    // Send the request
    write_dap_message(&mut stdout, &request).await?;

    // Read response from stdin
    let mut reader = BufReader::new(stdin);
    eprintln!("Waiting for response...");

    match tokio::time::timeout(Duration::from_secs(10), read_dap_message(&mut reader)).await {
        Ok(Ok(response)) => {
            if let Some(success) = response.get("success").and_then(|v| v.as_bool()) {
                if success {
                    println!("✓ Hot reload successful for module: {}", module);
                } else {
                    let error_msg = response.get("message")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error");
                    return Err(format!("Hot reload failed: {}", error_msg).into());
                }
            }
        }
        Ok(Err(e)) => {
            return Err(format!("Error reading response: {}", e).into());
        }
        Err(_) => {
            return Err("Timeout waiting for response".into());
        }
    }

    Ok(())
}

/// Read a DAP message with Content-Length headers
async fn read_dap_message<R: tokio::io::AsyncBufRead + Unpin>(reader: &mut R) -> Result<JsonValue, Box<dyn std::error::Error>> {
    let mut content_length: Option<usize> = None;

    // Read headers
    loop {
        let mut header = String::new();
        reader.read_line(&mut header).await?;
        let header = header.trim();

        // Empty line marks end of headers
        if header.is_empty() {
            break;
        }

        // Parse Content-Length header
        if let Some(length_str) = header.strip_prefix("Content-Length: ") {
            content_length = Some(length_str.trim().parse()?);
        }
    }

    // Read the message body
    let content_length = content_length.ok_or("Missing Content-Length header")?;
    let mut body = vec![0u8; content_length];
    tokio::io::AsyncReadExt::read_exact(reader, &mut body).await?;

    // Parse JSON
    let message: JsonValue = serde_json::from_slice(&body)?;
    Ok(message)
}

/// Write a DAP message with Content-Length header
async fn write_dap_message<W: tokio::io::AsyncWrite + Unpin>(writer: &mut W, message: &JsonValue) -> Result<(), Box<dyn std::error::Error>> {
    let body = serde_json::to_string(message)?;
    let header = format!("Content-Length: {}\r\n\r\n", body.len());

    writer.write_all(header.as_bytes()).await?;
    writer.write_all(body.as_bytes()).await?;
    writer.flush().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_reload_config_creation() {
        let config = HotReloadConfig {
            module: "mymodule".to_string(),
            host: "127.0.0.1".to_string(),
            port: Some(5678),
        };

        assert_eq!(config.module, "mymodule");
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, Some(5678));
    }
}
