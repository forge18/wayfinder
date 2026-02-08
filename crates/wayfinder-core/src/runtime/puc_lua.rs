use super::{super::*, BreakpointType, DebugRuntime, LuaVersion, RuntimeError, RuntimeType, Scope, StepMode, Value};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct PUCLuaRuntime {
    state: Arc<Mutex<PUCLuaState>>,
    breakpoints: Arc<Mutex<HashMap<String, Vec<u32>>>>,
}

#[derive(Debug, Default)]
struct PUCLuaState {
    running: bool,
    paused: bool,
    current_line: u32,
    current_source: Option<String>,
}

impl PUCLuaRuntime {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(PUCLuaState::default())),
            breakpoints: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl DebugRuntime for PUCLuaRuntime {
    async fn version(&self) -> RuntimeVersion {
        RuntimeVersion {
            runtime: RuntimeType::PUC,
            version: LuaVersion::V54,
        }
    }

    async fn set_breakpoint(&mut self, breakpoint: BreakpointType) -> Result<Breakpoint, RuntimeError> {
        match breakpoint {
            BreakpointType::Line { source, line } => {
                let mut breakpoints = self.breakpoints.lock().unwrap();
                breakpoints.entry(source).or_default().push(line);
                Ok(Breakpoint {
                    id: 1,
                    verified: true,
                    line,
                    message: None,
                })
            }
            BreakpointType::Function { name } => Ok(Breakpoint {
                id: 1,
                verified: true,
                line: 1,
                message: Some(format!("Function breakpoint: {}", name)),
            }),
            BreakpointType::Exception { filter } => Ok(Breakpoint {
                id: 1,
                verified: true,
                line: 0,
                message: Some(format!("Exception breakpoint: {}", filter)),
            }),
        }
    }

    async fn remove_breakpoint(&mut self, _id: i64) -> Result<(), RuntimeError> {
        Ok(())
    }

    async fn step(&mut self, _mode: StepMode) -> Result<(), RuntimeError> {
        let mut state = self.state.lock().unwrap();
        state.paused = true;
        Ok(())
    }

    async fn continue_(&mut self) -> Result<(), RuntimeError> {
        let mut state = self.state.lock().unwrap();
        state.running = true;
        state.paused = false;
        Ok(())
    }

    async fn pause(&mut self) -> Result<(), RuntimeError> {
        let mut state = self.state.lock().unwrap();
        state.paused = true;
        Ok(())
    }

    async fn stack_trace(&mut self, _thread_id: Option<u64>) -> Result<Vec<Frame>, RuntimeError> {
        let state = self.state.lock().unwrap();
        Ok(vec![Frame {
            id: 0,
            name: "main".to_string(),
            source: state.current_source.clone().map(|s| Source {
                name: s.clone(),
                path: s,
                source_reference: Some(0),
            }),
            line: state.current_line,
            column: 1,
        }])
    }

    async fn scopes(&mut self, frame_id: i64) -> Result<Vec<Scope>, RuntimeError> {
        Ok(vec![
            Scope {
                variables_reference: frame_id,
                name: "Locals".to_string(),
                expensive: false,
            },
            Scope {
                variables_reference: -1,
                name: "Globals".to_string(),
                expensive: true,
            },
        ])
    }

    async fn variables(
        &mut self,
        _variables_reference: i64,
        _filter: Option<VariableScope>,
    ) -> Result<Vec<Variable>, RuntimeError> {
        Ok(vec![])
    }

    async fn evaluate(&mut self, _frame_id: i64, expression: &str) -> Result<Value, RuntimeError> {
        match expression.trim() {
            "nil" => Ok(Value::Nil),
            "true" => Ok(Value::Boolean(true)),
            "false" => Ok(Value::Boolean(false)),
            s if s.parse::<f64>().is_ok() => Ok(Value::Number(s.parse().unwrap())),
            _ => Ok(Value::String(format!("<unknown: {}>", expression))),
        }
    }

    async fn run_to_location(&mut self, _source: &str, _line: u32) -> Result<(), RuntimeError> {
        Ok(())
    }

    async fn source(&mut self, _source_reference: i64) -> Result<String, RuntimeError> {
        Err(RuntimeError::NotImplemented("source not implemented".to_string()))
    }
}