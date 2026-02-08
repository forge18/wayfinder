use serde::{Deserialize, Serialize};
use std::fmt;

fn default_null() -> serde_json::Value {
    serde_json::Value::Null
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub method: String,
    #[serde(default = "default_null")]
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Response {
    pub id: u64,
    pub result: Result<serde_json::Value, ResponseError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProtocolMessage {
    Request(Message),
    Response(Response),
    Event(Event),
}

impl fmt::Display for ProtocolMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolMessage::Request(msg) => write!(f, "Request({})", msg.method),
            ProtocolMessage::Response(resp) => write!(f, "Response({:?})", resp.result),
            ProtocolMessage::Event(evt) => write!(f, "Event({})", evt.event),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Event {
    pub event: String,
    #[serde(default)]
    pub body: Option<serde_json::Value>,
}

impl Event {
    pub fn new(event: impl Into<String>, body: Option<serde_json::Value>) -> Self {
        Self {
            event: event.into(),
            body,
        }
    }

    pub fn initialized() -> Self {
        Self::new("initialized", None)
    }

    pub fn stopped(reason: &str, thread_id: Option<u64>, all_threads_stopped: bool) -> Self {
        let mut body = serde_json::json!({
            "reason": reason,
            "allThreadsStopped": all_threads_stopped,
        });
        if let Some(id) = thread_id {
            body["threadId"] = serde_json::json!(id);
        }
        Self::new("stopped", Some(body))
    }

    pub fn continued(thread_id: Option<u64>, all_threads_continued: bool) -> Self {
        let mut body = serde_json::json!({
            "allThreadsContinued": all_threads_continued,
        });
        if let Some(id) = thread_id {
            body["threadId"] = serde_json::json!(id);
        }
        Self::new("continued", Some(body))
    }

    pub fn exited(exit_code: i32) -> Self {
        let body = serde_json::json!({
            "exitCode": exit_code
        });
        Self::new("exited", Some(body))
    }

    pub fn terminated() -> Self {
        Self::new("terminated", Some(serde_json::json!({})))
    }

    pub fn output(category: &str, text: &str) -> Self {
        let body = serde_json::json!({
            "category": category,
            "output": text,
        });
        Self::new("output", Some(body))
    }

    pub fn thread(thread_id: u64, reason: &str) -> Self {
        let body = serde_json::json!({
            "threadId": thread_id,
            "reason": reason,
        });
        Self::new("thread", Some(body))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Source {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub source_reference: Option<i64>,
}

impl Message {
    pub fn new(id: u64, method: impl Into<String>, params: serde_json::Value) -> Self {
        Self {
            id,
            method: method.into(),
            params,
        }
    }
}

impl Response {
    pub fn new_ok(id: u64, result: serde_json::Value) -> Self {
        Self {
            id,
            result: Ok(result),
        }
    }

    pub fn new_error(id: u64, code: i32, message: impl Into<String>) -> Self {
        Self {
            id,
            result: Err(ResponseError {
                code,
                message: message.into(),
            }),
        }
    }
}
