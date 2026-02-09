//! DAP server command implementation
//!
//! This module handles running Wayfinder as a DAP (Debug Adapter Protocol) server.

use std::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use serde_json::Value as JsonValue;
use wayfinder_core::runtime::puc_lua::PUCLuaRuntime;
use wayfinder_core::session::DapServer;

/// DAP server configuration
#[derive(Debug)]
pub struct DapConfig {
    /// Port to listen on (None for stdio mode)
    pub port: Option<u16>,
    /// Whether to support multiple clients
    pub multi_client: bool,
}

/// Run as a DAP server
pub async fn run_dap_server(config: DapConfig) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(port) = config.port {
        // Run in TCP server mode
        run_tcp_server(port, config.multi_client).await
    } else {
        // Run in stdio mode
        run_stdio_server().await
    }
}

/// Run DAP server in TCP mode
async fn run_tcp_server(port: u16, _multi_client: bool) -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("127.0.0.1:{}", port);
    println!("Starting DAP server on {}", address);
    
    // Create TCP listener
    let listener = TcpListener::bind(&address)?;
    listener.set_nonblocking(true)?;
    
    // Convert to tokio listener
    let listener = tokio::net::TcpListener::from_std(listener)?;
    
    println!("DAP server listening on {}", address);
    
    // Accept connections
    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("Client connected from {}", addr);
                
                // Handle the connection
                if let Err(e) = handle_tcp_connection(stream).await {
                    eprintln!("Error handling connection: {}", e);
                }
                
                // For now, we'll only handle one client
                // In a multi-client implementation, we would spawn a task for each client
                break;
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
    
    Ok(())
}

/// Handle a TCP connection
async fn handle_tcp_connection(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let peer_addr = stream.peer_addr()?;
    eprintln!("Handling connection from {}", peer_addr);

    // Create DAP server
    let mut server: DapServer<PUCLuaRuntime> = DapServer::new();

    // Set up the runtime
    let runtime = crate::create_puc_lua_runtime(None);
    server.set_runtime(runtime);

    // Split the stream for reading and writing
    let (read_half, write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);
    let mut writer = write_half;

    eprintln!("Starting DAP event loop for {}", peer_addr);

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
                    eprintln!("Received disconnect/terminate from {}", peer_addr);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading DAP message from {}: {}", peer_addr, e);
                break;
            }
        }
    }

    eprintln!("Connection from {} closed", peer_addr);
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

/// Run DAP server in stdio mode
async fn run_stdio_server() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Starting DAP server in stdio mode");
    eprintln!("Reading from stdin, writing to stdout");
    eprintln!("Waiting for DAP initialize request...");

    // Create DAP server
    let mut server: DapServer<PUCLuaRuntime> = DapServer::new();

    // Set up the runtime
    let runtime = crate::create_puc_lua_runtime(None);
    server.set_runtime(runtime);

    // Set up stdin/stdout for DAP communication
    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut reader = BufReader::new(stdin);

    // DAP message loop
    loop {
        // Read the message from stdin
        match read_dap_message(&mut reader).await {
            Ok(message) => {
                eprintln!("Received DAP message: {}", message.get("method").and_then(|m| m.as_str()).unwrap_or("unknown"));

                // Extract method, params, and id
                let method = message.get("method").and_then(|m| m.as_str()).unwrap_or("");
                let params = message.get("params").unwrap_or(&JsonValue::Null);
                let id = message.get("id").and_then(|i| i.as_u64()).unwrap_or(0);

                // Handle the request
                if let Some(response) = server.handle_request(method, params, id).await {
                    // Send the response
                    write_dap_message(&mut stdout, &response).await?;
                }

                // Check if we should exit
                if method == "disconnect" || method == "terminate" {
                    eprintln!("Received disconnect/terminate, shutting down");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading DAP message: {}", e);
                // On EOF or error, exit the loop
                break;
            }
        }
    }

    eprintln!("DAP server shutting down");
    Ok(())
}

/// Read a DAP message from stdin using Content-Length headers
async fn read_dap_message(reader: &mut BufReader<tokio::io::Stdin>) -> Result<JsonValue, Box<dyn std::error::Error>> {
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

/// Write a DAP message to stdout with Content-Length header
async fn write_dap_message(writer: &mut tokio::io::Stdout, message: &JsonValue) -> Result<(), Box<dyn std::error::Error>> {
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
    fn test_dap_config_creation() {
        let tcp_config = DapConfig {
            port: Some(12345),
            multi_client: true,
        };
        
        assert_eq!(tcp_config.port, Some(12345));
        assert_eq!(tcp_config.multi_client, true);
        
        let stdio_config = DapConfig {
            port: None,
            multi_client: false,
        };
        
        assert_eq!(stdio_config.port, None);
        assert_eq!(stdio_config.multi_client, false);
    }
}