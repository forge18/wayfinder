//! Attach command implementation
//!
//! This module handles attaching to running Lua processes for debugging.

use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use serde_json::Value as JsonValue;
use wayfinder_core::runtime::puc_lua::PUCLuaRuntime;
use wayfinder_core::session::DapServer;

/// Attach configuration
#[derive(Debug)]
pub struct AttachConfig {
    /// Port to connect to (for TCP connections)
    pub port: Option<u16>,
    /// Process ID to attach to
    pub pid: Option<u32>,
}

/// Attach to a running Lua process
pub async fn attach_to_process(config: AttachConfig) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(port) = config.port {
        // Connect via TCP
        println!("Attaching to process on port {}", port);
        attach_via_tcp(port).await?;
    } else if let Some(pid) = config.pid {
        // Attach via PID
        println!("Attaching to process with PID {}", pid);
        attach_via_pid(pid).await?;
    } else {
        return Err("Either port or PID must be specified for attach".into());
    }

    Ok(())
}

/// Attach to a process via TCP connection
async fn attach_via_tcp(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = format!("127.0.0.1:{}", port).parse()?;

    eprintln!("Connecting to {}...", address);

    // Attempt to connect with a timeout
    let stream = match tokio::time::timeout(Duration::from_secs(10), TcpStream::connect(&address)).await {
        Ok(Ok(stream)) => stream,
        Ok(Err(e)) => {
            return Err(format!("Failed to connect to {}: {}", address, e).into());
        }
        Err(_) => {
            return Err(format!("Connection timeout after 10 seconds").into());
        }
    };

    eprintln!("✓ Connected to process on port {}", port);
    eprintln!("Setting up DAP session...");

    // Create DAP server for this attachment
    let mut server: DapServer<PUCLuaRuntime> = DapServer::new();

    // Set up the runtime
    let runtime = crate::create_puc_lua_runtime();
    server.set_runtime(runtime);

    // Split the stream for reading and writing
    let (read_half, write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);
    let mut writer = write_half;

    eprintln!("Starting DAP message loop...");

    // DAP message loop
    loop {
        // Read the message from the TCP stream
        match read_dap_message_tcp(&mut reader).await {
            Ok(message) => {
                eprintln!("Received DAP message: {}", message.get("method").and_then(|m| m.as_str()).unwrap_or("unknown"));

                // Extract method, params, and id
                let method = message.get("method").and_then(|m| m.as_str()).unwrap_or("");
                let params = message.get("params").unwrap_or(&JsonValue::Null);
                let id = message.get("id").and_then(|i| i.as_u64()).unwrap_or(0);

                // Handle the request
                if let Some(response) = server.handle_request(method, params, id).await {
                    // Send the response
                    write_dap_message_tcp(&mut writer, &response).await?;
                }

                // Check if we should exit
                if method == "disconnect" || method == "terminate" {
                    eprintln!("Received disconnect/terminate");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading DAP message: {}", e);
                break;
            }
        }
    }

    eprintln!("Connection closed");
    Ok(())
}

/// Read a DAP message from TCP stream using Content-Length headers
async fn read_dap_message_tcp<R: tokio::io::AsyncBufRead + Unpin>(reader: &mut R) -> Result<JsonValue, Box<dyn std::error::Error>> {
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

/// Write a DAP message to TCP stream with Content-Length header
async fn write_dap_message_tcp<W: tokio::io::AsyncWrite + Unpin>(writer: &mut W, message: &JsonValue) -> Result<(), Box<dyn std::error::Error>> {
    let body = serde_json::to_string(message)?;
    let header = format!("Content-Length: {}\r\n\r\n", body.len());

    writer.write_all(header.as_bytes()).await?;
    writer.write_all(body.as_bytes()).await?;
    writer.flush().await?;

    Ok(())
}

/// Attach to a process via PID
async fn attach_via_pid(pid: u32) -> Result<(), Box<dyn std::error::Error>> {
    // Validate the process exists
    validate_pid(pid)?;

    println!("✓ Process with PID {} exists", pid);

    // In a real implementation, we would:
    // 1. Use ptrace (Unix) or DebugActiveProcess (Windows) to attach
    // 2. Inject the Lua debug library into the process
    // 3. Establish communication with the injected debug adapter
    // 4. Run the DAP event loop

    println!("\n--- Attached to Process ---");
    println!("Note: PID-based attachment requires platform-specific");
    println!("debugging APIs and Lua runtime injection.");
    println!("This is a placeholder implementation.");
    println!("\nIn a full implementation, this would:");
    println!("  1. Attach to the process using OS debug APIs");
    println!("  2. Inject Lua debugging hooks");
    println!("  3. Establish DAP communication channel");
    println!("  4. Start processing debug commands");

    Ok(())
}

/// Validate that a process with the given PID exists
#[allow(dead_code)]
fn validate_pid(pid: u32) -> Result<(), Box<dyn std::error::Error>> {
    // On Unix systems, we could check /proc/{pid}
    // On Windows, we could use OpenProcess
    // For cross-platform compatibility, we'll just return Ok for now
    
    #[cfg(unix)]
    {
        let path = format!("/proc/{}", pid);
        if std::path::Path::new(&path).exists() {
            Ok(())
        } else {
            Err(format!("Process with PID {} not found", pid).into())
        }
    }
    
    #[cfg(windows)]
    {
        // Windows implementation would use OpenProcess
        // For now, we'll just return Ok
        Ok(())
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        // For other platforms, we'll just return Ok
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attach_config_creation() {
        let config_with_port = AttachConfig {
            port: Some(12345),
            pid: None,
        };
        
        assert_eq!(config_with_port.port, Some(12345));
        assert_eq!(config_with_port.pid, None);
        
        let config_with_pid = AttachConfig {
            port: None,
            pid: Some(1234),
        };
        
        assert_eq!(config_with_pid.port, None);
        assert_eq!(config_with_pid.pid, Some(1234));
    }
}