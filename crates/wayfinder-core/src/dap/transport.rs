use super::{Event, Message, ProtocolMessage, Response};
use std::io::{self, BufRead, Write};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};

const CONTENT_LENGTH_HEADER: &str = "Content-Length: ";

pub struct StdioTransport {
    stdin: tokio::process::ChildStdin,
    stdout: BufReader<tokio::process::ChildStdout>,
    buffer: String,
}

impl StdioTransport {
    pub fn new(child: Child) -> Self {
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let stdout = BufReader::new(stdout);

        Self {
            stdin,
            stdout,
            buffer: String::new(),
        }
    }

    pub async fn read_message(&mut self) -> io::Result<ProtocolMessage> {
        let mut headers = Vec::new();
        let mut content_length: Option<usize> = None;

        loop {
            let mut line = String::new();
            self.stdout.read_line(&mut line).await?;
            let line = line.trim_end();

            if line.is_empty() {
                break;
            }
            headers.push(line.to_string());

            if let Some(content_length_str) = line.strip_prefix(CONTENT_LENGTH_HEADER) {
                content_length = Some(content_length_str.trim().parse().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid Content-Length")
                })?);
            }
        }

        let content_length = content_length.ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Missing Content-Length header")
        })?;

        let mut body = vec![0u8; content_length];
        self.stdout.read_exact(&mut body).await?;

        let body_str = String::from_utf8(body).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 in message body")
        })?;

        serde_json::from_str::<serde_json::Value>(&body_str).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid JSON: {}", e))
        })?;

        let value: serde_json::Value = serde_json::from_str(&body_str).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid JSON-RPC message: {}", e))
        })?;

        parse_message(value)
    }

    pub async fn write_message(&mut self, message: &ProtocolMessage) -> io::Result<()> {
        let body = serde_json::to_string(&serialize_message(message)).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Failed to serialize: {}", e))
        })?;

        let header = format!("Content-Length: {}\r\n\r\n", body.len());
        self.stdin.write_all(header.as_bytes()).await?;
        self.stdin.write_all(body.as_bytes()).await?;
        self.stdin.flush().await?;
        Ok(())
    }
}

fn parse_message(value: serde_json::Value) -> io::Result<ProtocolMessage> {
    if let Some(id) = value.get("id").and_then(|v| v.as_u64()) {
        if value.get("result").is_some() || value.get("error").is_some() {
            let result = if let Some(error) = value.get("error") {
                let code = error.get("code").and_then(|v| v.as_i32()).unwrap_or(-1);
                let message = error
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error")
                    .to_string();
                Err(super::ResponseError { code, message })
            } else {
                Ok(value.get("result").cloned().unwrap_or(serde_json::Value::Null))
            };
            Ok(ProtocolMessage::Response(Response { id, result }))
        } else {
            let method = value
                .get("method")
                .and_then(|v| v.as_str())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing method"))?
                .to_string();
            let params = value.get("params").cloned().unwrap_or(serde_json::Value::Null);
            Ok(ProtocolMessage::Request(Message { id, method, params }))
        }
    } else if let Some(event_type) = value.get("event").and_then(|v| v.as_str()) {
        let body = value.get("body").cloned();
        Ok(ProtocolMessage::Event(Event {
            event: event_type.to_string(),
            body,
        }))
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid message: must have id or event",
        ))
    }
}

fn serialize_message(message: &ProtocolMessage) -> serde_json::Value {
    match message {
        ProtocolMessage::Request(msg) => serde_json::json!({
            "id": msg.id,
            "method": msg.method,
            "params": msg.params
        }),
        ProtocolMessage::Response(resp) => match &resp.result {
            Ok(result) => serde_json::json!({
                "id": resp.id,
                "result": result
            }),
            Err(error) => serde_json::json!({
                "id": resp.id,
                "error": {
                    "code": error.code,
                    "message": error.message
                }
            }),
        },
        ProtocolMessage::Event(evt) => {
            let mut value = serde_json::json!({
                "event": evt.event
            });
            if let Some(body) = &evt.body {
                value["body"] = body.clone();
            }
            value
        }
    }
}