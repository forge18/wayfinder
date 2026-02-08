use super::{Frame, RuntimeError, RuntimeVersion, Scope, StepMode, Value, Variable, VariableScope};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct MockRuntime {
    state: Arc<Mutex<MockState>>,
    breakpoints: Arc<Mutex<HashMap<String, Vec<u32>>>>,
}

#[derive(Debug, Default)]
struct MockState {
    running: bool,
    paused: bool,
    current_frame: Option<Frame>,
    variables: HashMap<i64, Vec<Variable>>,
}

impl MockRuntime {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(MockState::default()));
        let breakpoints = Arc::new(Mutex::new(HashMap::new()));

        let mut variables = HashMap::new();
        variables.insert(
            0,
            vec![
                Variable {
                    name: "x".to_string(),
                    value: "10".to_string(),
                    type_: "number".to_string(),
                    variables_reference: None,
                    named_variables: None,
                    indexed_variables: None,
                },
                Variable {
                    name: "y".to_string(),
                    value: "20".to_string(),
                    type_: "number".to_string(),
                    variables_reference: None,
                    named_variables: None,
                    indexed_variables: None,
                },
            ],
        );

        Self { state, breakpoints }
    }
}

#[async_trait::async_trait]
impl super::DebugRuntime for MockRuntime {
    async fn version(&self) -> RuntimeVersion {
        RuntimeVersion {
            runtime: super::RuntimeType::PUC,
            version: super::LuaVersion::V54,
        }
    }

    async fn set_breakpoint(
        &mut self,
        breakpoint: super::BreakpointType,
    ) -> Result<super::Breakpoint, RuntimeError> {
        match breakpoint {
            super::BreakpointType::Line { source, line } => {
                let mut breakpoints = self.breakpoints.lock().unwrap();
                breakpoints.entry(source).or_default().push(line);
                Ok(super::Breakpoint {
                    id: 1,
                    verified: true,
                    line,
                    message: None,
                })
            }
            super::BreakpointType::Function { name } => Ok(super::Breakpoint {
                id: 1,
                verified: true,
                line: 1,
                message: Some(format!("Function breakpoint: {}", name)),
            }),
            super::BreakpointType::Exception { filter } => Ok(super::Breakpoint {
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
        state.current_frame = Some(Frame {
            id: 0,
            name: "test_function".to_string(),
            source: Some(super::Source {
                name: "test.lua".to_string(),
                path: "/test/test.lua".to_string(),
                source_reference: Some(0),
            }),
            line: 5,
            column: 1,
        });
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
        if let Some(frame) = &state.current_frame {
            Ok(vec![frame.clone()])
        } else {
            Ok(vec![Frame {
                id: 0,
                name: "main".to_string(),
                source: Some(super::Source {
                    name: "main.lua".to_string(),
                    path: "/test/main.lua".to_string(),
                    source_reference: Some(0),
                }),
                line: 1,
                column: 1,
            }])
        }
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
        variables_reference: i64,
        _filter: Option<VariableScope>,
    ) -> Result<Vec<Variable>, RuntimeError> {
        let state = self.state.lock().unwrap();
        Ok(state
            .variables
            .get(&variables_reference)
            .cloned()
            .unwrap_or_default())
    }

    async fn evaluate(&mut self, _frame_id: i64, expression: &str) -> Result<Value, RuntimeError> {
        match expression.trim() {
            "x" => Ok(Value::Number(10.0)),
            "y" => Ok(Value::Number(20.0)),
            "nil" => Ok(Value::Nil),
            "true" => Ok(Value::Boolean(true)),
            "false" => Ok(Value::Boolean(false)),
            s if s.parse::<f64>().is_ok() => Ok(Value::Number(s.parse().unwrap())),
            s if s.starts_with('"') && s.ends_with('"') => {
                Ok(Value::String(s[1..s.len() - 1].to_string()))
            }
            _ => Ok(Value::String(format!("<unknown: {}>", expression))),
        }
    }

    async fn run_to_location(&mut self, _source: &str, _line: u32) -> Result<(), RuntimeError> {
        Ok(())
    }

    async fn source(&mut self, _source_reference: i64) -> Result<String, RuntimeError> {
        Ok("-- Mock source code".to_string())
    }

    async fn get_exception_info(&mut self, _thread_id: u64) -> Result<super::ExceptionInfo, RuntimeError> {
        Ok(super::ExceptionInfo {
            exception_type: "RuntimeError".to_string(),
            message: "An error occurred".to_string(),
            stack_trace: vec![Frame {
                id: 1,
                name: "main".to_string(),
                source: Some(super::Source {
                    name: "main.lua".to_string(),
                    path: "/test/main.lua".to_string(),
                    source_reference: Some(0),
                }),
                line: 10,
                column: 5,
            }],
            inner_exception: None,
            details: None,
        })
    }
}
