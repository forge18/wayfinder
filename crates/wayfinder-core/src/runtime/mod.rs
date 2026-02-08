use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LuaVersion {
    V51,
    V52,
    V53,
    V54,
}

impl fmt::Display for LuaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LuaVersion::V51 => write!(f, "5.1"),
            LuaVersion::V52 => write!(f, "5.2"),
            LuaVersion::V53 => write!(f, "5.3"),
            LuaVersion::V54 => write!(f, "5.4"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuntimeType {
    PUC,
    LuaNext,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RuntimeVersion {
    pub runtime: RuntimeType,
    pub version: LuaVersion,
}

impl fmt::Display for RuntimeVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.runtime {
            RuntimeType::PUC => write!(f, "PUC Lua {}", self.version),
            RuntimeType::LuaNext => write!(f, "LuaNext {}", self.version),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StepMode {
    Over,
    In,
    Out,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VariableScope {
    Local,
    Upvalue,
    Global,
    Table { reference: i64 },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Frame {
    pub id: i64,
    pub name: String,
    pub source: Option<Source>,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Source {
    pub name: String,
    pub path: String,
    pub source_reference: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub type_: String,
    pub variables_reference: Option<i64>,
    pub named_variables: Option<u32>,
    pub indexed_variables: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Table {
        reference: i64,
        length: u32,
    },
    Function {
        reference: i64,
        name: Option<String>,
    },
    UserData,
    Thread,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Breakpoint {
    pub id: i64,
    pub verified: bool,
    pub line: u32,
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BreakpointType {
    Line { source: String, line: u32 },
    Function { name: String },
    Exception { filter: String },
}

#[derive(thiserror::Error, Debug)]
pub enum RuntimeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Process exited with code: {0}")]
    ProcessExited(i32),

    #[error("Process killed")]
    ProcessKilled,

    #[error("Communication error: {0}")]
    Communication(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

#[async_trait::async_trait]
pub trait DebugRuntime: Send + Sync {
    async fn version(&self) -> RuntimeVersion;

    async fn set_breakpoint(&mut self, breakpoint: BreakpointType) -> Result<Breakpoint>;

    async fn remove_breakpoint(&mut self, id: i64) -> Result<()>;

    async fn step(&mut self, mode: StepMode) -> Result<()>;

    async fn continue_(&mut self) -> Result<()>;

    async fn pause(&mut self) -> Result<()>;

    async fn stack_trace(&mut self, thread_id: Option<u64>) -> Result<Vec<Frame>>;

    async fn scopes(&mut self, frame_id: i64) -> Result<Vec<Scope>>;

    async fn variables(
        &mut self,
        variables_reference: i64,
        filter: Option<VariableScope>,
    ) -> Result<Vec<Variable>>;

    async fn evaluate(&mut self, frame_id: i64, expression: &str) -> Result<Value>;

    async fn run_to_location(&mut self, source: &str, line: u32) -> Result<()>;

    async fn source(&mut self, source_reference: i64) -> Result<String>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Scope {
    pub variables_reference: i64,
    pub name: String,
    pub expensive: bool,
}

pub mod mock;
pub mod puc_lua;
