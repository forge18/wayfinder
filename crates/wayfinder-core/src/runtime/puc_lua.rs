use super::{super::*, BreakpointType, DebugRuntime, LuaVersion, RuntimeError, RuntimeType, Scope, StepMode, Value};
use crate::runtime::lua_state::{Lua, DebugInfo};
use async_trait::async_trait;
use libc::c_int;
use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::{Arc, Mutex};
use crate::runtime::lua_ffi::{lua_getinfo, lua_getlocal, lua_getupvalue, lua_next};

pub struct PUCLuaRuntime {
    lua: Arc<Mutex<Lua>>,
    breakpoints: Arc<Mutex<HashMap<String, Vec<u32>>>>,
    step_mode: Arc<Mutex<StepMode>>,
    paused: Arc<Mutex<bool>>,
    current_line: Arc<Mutex<u32>>,
    current_source: Arc<Mutex<Option<String>>>,
}

impl PUCLuaRuntime {
    pub fn new() -> Self {
        Self {
            lua: Arc::new(Mutex::new(Lua::new())),
            breakpoints: Arc::new(Mutex::new(HashMap::new())),
            step_mode: Arc::new(Mutex::new(StepMode::Over)),
            paused: Arc::new(Mutex::new(false)),
            current_line: Arc::new(Mutex::new(1)),
            current_source: Arc::new(Mutex::new(None)),
        }
    }

    fn lua_to_value(lua: &mut Lua, index: c_int) -> Value {
        let lua_type = lua.type_of(index);

        match lua_type {
            0 => Value::Nil,
            1 => Value::Boolean(lua.pop_boolean()),
            2 => Value::UserData,
            3 => Value::Number(lua.pop_number()),
            4 => Value::String(lua.pop_string()),
            5 => {
                let len = lua.len(index);
                Value::Table {
                    reference: 0,
                    length: len as u32,
                }
            }
            6 => Value::Function {
                reference: 0,
                name: None,
            },
            7 => Value::UserData,
            8 => Value::Thread,
            _ => Value::Nil,
        }
    }

    pub fn execute_code(&self, code: &str) -> Result<Value, String> {
        let mut lua = self.lua.lock().unwrap();
        lua.execute(code)?;
        Ok(Self::lua_to_value(&mut lua, -1))
    }

    pub fn load_file(&self, filename: &str) -> Result<c_int, String> {
        let mut lua = self.lua.lock().unwrap();
        lua.load_file(filename)
    }

    pub fn load_string(&self, code: &str) -> Result<c_int, String> {
        let mut lua = self.lua.lock().unwrap();
        lua.load_string(code)
    }

    pub fn pcall(&self, nargs: c_int, nresults: c_int) -> Result<c_int, String> {
        let mut lua = self.lua.lock().unwrap();
        lua.pcall(nargs, nresults)
    }

    pub fn get_global(&mut self, name: &str) -> c_int {
        let mut lua = self.lua.lock().unwrap();
        lua.get_global(name)
    }

    pub fn set_top(&mut self, idx: c_int) {
        let mut lua = self.lua.lock().unwrap();
        lua.set_top(idx);
    }

    pub fn get_top(&self) -> c_int {
        let lua = self.lua.lock().unwrap();
        lua.get_top()
    }

    pub fn type_of(&self, idx: c_int) -> c_int {
        let lua = self.lua.lock().unwrap();
        lua.type_of(idx)
    }

    pub fn is_nil(&self, idx: c_int) -> bool {
        let lua = self.lua.lock().unwrap();
        lua.is_nil(idx)
    }

    pub fn pop_string(&mut self) -> String {
        let mut lua = self.lua.lock().unwrap();
        lua.pop_string()
    }

    pub fn get_local_variable(&mut self, ar: &mut DebugInfo, n: c_int) -> Option<(String, Value)> {
        let mut lua = self.lua.lock().unwrap();

        let name = unsafe {
            let ptr = lua_getlocal(lua.state(), ar.ptr(), n);
            if ptr.is_null() {
                return None;
            }
            let name = CStr::from_ptr(ptr).to_string_lossy().to_string();
            lua.set_top(-2);
            name
        };

        let value = Self::lua_to_value(&mut lua, -1);
        Some((name, value))
    }

    pub fn get_upvalue(&mut self, func_index: c_int, n: c_int) -> Option<(String, Value)> {
        let mut lua = self.lua.lock().unwrap();

        unsafe {
            let ptr = lua_getupvalue(lua.state(), func_index, n);
            if ptr.is_null() {
                return None;
            }
            let name = CStr::from_ptr(ptr).to_string_lossy().to_string();
            let value = Self::lua_to_value(&mut lua, -1);
            lua.set_top(-2);
            Some((name, value))
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
                breakpoints.entry(source.clone()).or_default().push(line);

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

    async fn step(&mut self, mode: StepMode) -> Result<(), RuntimeError> {
        let mut step_mode = self.step_mode.lock().unwrap();
        *step_mode = mode;
        Ok(())
    }

    async fn continue_(&mut self) -> Result<(), RuntimeError> {
        Ok(())
    }

    async fn pause(&mut self) -> Result<(), RuntimeError> {
        Ok(())
    }

    async fn stack_trace(&mut self, _thread_id: Option<u64>) -> Result<Vec<Frame>, RuntimeError> {
        let mut frames = Vec::new();

        for level in 0..10 {
            let lua = self.lua.lock().unwrap();

            unsafe {
                let mut ar = DebugInfo::new();
                let result = lua_getinfo(lua.state(), b"nSluf\0".as_ptr() as *const i8, ar.ptr());

                if result == 0 {
                    break;
                }

                let name = ar.name().unwrap_or("unknown").to_string();
                let source = ar.source().map(|s| s.to_string());

                frames.push(Frame {
                    id: level as i64,
                    name,
                    source: source.map(|s| Source {
                        name: s.clone(),
                        path: s,
                        source_reference: Some(0),
                    }),
                    line: ar.current_line() as u32,
                    column: 1,
                });
            }
        }

        Ok(frames)
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
        _filter: Option<super::VariableScope>,
    ) -> Result<Vec<super::Variable>, RuntimeError> {
        let mut variables = Vec::new();

        if variables_reference >= 0 {
            let mut lua = self.lua.lock().unwrap();
            lua.set_top(variables_reference as c_int);

            let mut index = 1;
            while index > 0 {
                unsafe {
                    lua.push_nil();
                    if lua_next(lua.state(), variables_reference as c_int) == 0 {
                        break;
                    }

                    let key = lua.pop_string();
                    let value_type = lua.type_of(-1);
                    let value_str = match value_type {
                        0 => "nil".to_string(),
                        1 => format!("{}", lua.pop_boolean()),
                        3 => format!("{}", lua.pop_number()),
                        4 => lua.pop_string(),
                        5 => format!("table ({})", lua.len(-1)),
                        6 => "function".to_string(),
                        _ => format!("{}", lua.type_name(value_type)),
                    };

                    variables.push(super::Variable {
                        name: key,
                        value: value_str,
                        type_: lua.type_name(value_type).to_string(),
                        variables_reference: if value_type == 5 { Some(0) } else { None },
                        named_variables: None,
                        indexed_variables: None,
                    });
                }
                index -= 1;
            }
        }

        Ok(variables)
    }

    async fn evaluate(&mut self, _frame_id: i64, expression: &str) -> Result<Value, RuntimeError> {
        let trimmed = expression.trim();

        if trimmed.is_empty() {
            return Ok(Value::Nil);
        }

        if let Ok(result) = self.execute_code(trimmed) {
            return Ok(result);
        }

        match trimmed {
            "nil" => Ok(Value::Nil),
            "true" => Ok(Value::Boolean(true)),
            "false" => Ok(Value::Boolean(false)),
            s if s.parse::<f64>().is_ok() => Ok(Value::Number(s.parse().unwrap())),
            _ => Ok(Value::String(format!("<error: {}>", expression))),
        }
    }

    async fn run_to_location(&mut self, _source: &str, _line: u32) -> Result<(), RuntimeError> {
        Ok(())
    }

    async fn source(&mut self, _source_reference: i64) -> Result<String, RuntimeError> {
        Err(RuntimeError::NotImplemented("source not implemented".to_string()))
    }
}
