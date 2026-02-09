// Allow common warnings in development/incomplete features
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(non_snake_case)] // Lua C API uses mixed case (_L, luaL_newstate, etc.)
#![allow(static_mut_refs)] // Required for Lua FFI interaction

pub mod config;
pub mod dap;
pub mod debug;
pub mod hot_reload;
pub mod memory;
pub mod profiling;
pub mod runtime;
pub mod session;

pub use config::{DebuggerConfig, EvalSafety};
pub use dap::{Event, Message, ProtocolMessage, Response};
pub use debug::breakpoints::{BreakpointManager, LineBreakpoint, FunctionBreakpoint};
pub use memory::MemoryStatistics;
pub use profiling::{ProfileData, ProfilingMode, FunctionProfile};
pub use runtime::{
    Breakpoint, BreakpointType, Frame, RuntimeError, RuntimeType, RuntimeVersion, Scope, Source,
    StepMode, Variable, VariableScope, Value,
};
pub use session::{DapServer, DebugSession};