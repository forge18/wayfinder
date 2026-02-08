use super::{super::*, BreakpointType, DebugRuntime, LuaVersion, RuntimeError, RuntimeType, Scope, StepMode, Value};
use crate::runtime::lua_state::{Lua, DebugInfo};
use crate::runtime::lua_ffi::*;
use async_trait::async_trait;
use libc::c_int;
use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

static mut PAUSED: AtomicBool = AtomicBool::new(false);
static mut SHOULD_STEP: AtomicBool = AtomicBool::new(false);
static mut CURRENT_LINE: AtomicUsize = AtomicUsize::new(1);
static mut CURRENT_SOURCE: Option<String> = None;
static mut STEP_MODE: AtomicUsize = AtomicUsize::new(0);
static mut STEP_DEPTH: AtomicUsize = AtomicUsize::new(0);
static mut STEP_TRIGGERED: AtomicBool = AtomicBool::new(false);

extern "C" fn lua_hook_callback(_L: LuaState, ar: *mut lua_Debug) {
    unsafe {
        if lua_getinfo(_L, b"lS\0".as_ptr() as *const i8, ar) == 0 {
            return;
        }

        let line = (*ar).currentline as u32;
        CURRENT_LINE.store(line as usize, Ordering::SeqCst);

        let source = {
            let source_ptr = (*ar).source;
            if !source_ptr.is_null() {
                let c_str = CStr::from_ptr(source_ptr);
                Some(c_str.to_string_lossy().to_string())
            } else {
                None
            }
        };
        CURRENT_SOURCE = source;

        let step_mode = StepMode::from_u32(STEP_MODE.load(Ordering::SeqCst) as u32);
        let should_step = SHOULD_STEP.load(Ordering::SeqCst);

        let triggered_for_step = if should_step {
            match step_mode {
                StepMode::In => true,
                StepMode::Over => {
                    let depth = (*ar).linedefined as usize;
                    if depth <= STEP_DEPTH.load(Ordering::SeqCst) {
                        true
                    } else {
                        false
                    }
                }
                StepMode::Out => {
                    false
                }
            }
        } else {
            false
        };

        if triggered_for_step {
            STEP_TRIGGERED.store(true, Ordering::SeqCst);
            PAUSED.store(true, Ordering::SeqCst);
        }
    }
}

pub struct PUCLuaRuntime {
    lua: Arc<Mutex<Lua>>,
    breakpoints: Arc<Mutex<HashMap<String, Vec<u32>>>>,
    step_mode: Arc<Mutex<StepMode>>,
}

impl PUCLuaRuntime {
    pub fn new() -> Self {
        unsafe {
            PAUSED.store(false, Ordering::SeqCst);
            SHOULD_STEP.store(false, Ordering::SeqCst);
            CURRENT_LINE.store(1, Ordering::SeqCst);
        }

        Self {
            lua: Arc::new(Mutex::new(Lua::new())),
            breakpoints: Arc::new(Mutex::new(HashMap::new())),
            step_mode: Arc::new(Mutex::new(StepMode::Over)),
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

    pub fn get_stack_info(&mut self, _level: i32) -> Option<DebugInfo<'static>> {
        let lua = self.lua.lock().unwrap();

        unsafe {
            let mut ar = DebugInfo::new();
            let result = lua_getinfo(lua.state(), b"SnSluf\0".as_ptr() as *const i8, ar.ptr());

            if result == 0 {
                return None;
            }

            Some(ar)
        }
    }

    fn is_breakpoint_hit(&self, source: &str, line: u32) -> bool {
        let breakpoints = self.breakpoints.lock().unwrap();
        if let Some(lines) = breakpoints.get(source) {
            lines.contains(&line)
        } else {
            false
        }
    }

    pub fn is_breakpoint_hit_at_current_location(&self) -> bool {
        let source = unsafe { CURRENT_SOURCE.clone() };
        let line = unsafe { CURRENT_LINE.load(Ordering::SeqCst) as u32 };

        if let Some(ref s) = source {
            self.is_breakpoint_hit(s, line)
        } else {
            false
        }
    }

    pub fn should_remain_paused(&self) -> bool {
        if self.is_breakpoint_hit_at_current_location() {
            return true;
        }
        unsafe { STEP_TRIGGERED.load(Ordering::SeqCst) }
    }

    pub fn clear_step_triggered(&self) {
        unsafe {
            STEP_TRIGGERED.store(false, Ordering::SeqCst);
        }
    }

    pub fn install_hook(&self) {
        let lua = self.lua.lock().unwrap();
        unsafe {
            lua_sethook(lua.state(), lua_hook_callback, LUA_MASKLINE, 0);
        }
    }

    pub fn is_paused(&self) -> bool {
        unsafe { PAUSED.load(Ordering::SeqCst) }
    }

    pub fn wait_for_pause(&self, timeout_ms: u64) -> bool {
        let start = std::time::Instant::now();
        while !self.is_paused() {
            if start.elapsed() > Duration::from_millis(timeout_ms) {
                return false;
            }
            thread::sleep(Duration::from_millis(10));
        }
        true
    }

    pub fn handle_pause(&self) -> bool {
        let is_breakpoint = self.is_breakpoint_hit_at_current_location();
        let step_triggered = unsafe { STEP_TRIGGERED.load(Ordering::SeqCst) };

        if is_breakpoint || step_triggered {
            self.clear_step_triggered();
            true
        } else {
            self.clear_pause();
            self.install_hook();
            false
        }
    }

    pub fn clear_pause(&self) {
        unsafe {
            PAUSED.store(false, Ordering::SeqCst);
            SHOULD_STEP.store(false, Ordering::SeqCst);
            STEP_TRIGGERED.store(false, Ordering::SeqCst);
        }
    }

    pub fn set_step(&self, mode: StepMode) {
        unsafe {
            SHOULD_STEP.store(true, Ordering::SeqCst);
            STEP_MODE.store(mode.to_u32() as usize, Ordering::SeqCst);

            let mut lua = self.lua.lock().unwrap();
            let mut ar = DebugInfo::new();
            if lua_getinfo(lua.state(), b"n\0".as_ptr() as *const i8, ar.ptr()) != 0 {
                let depth = ar.linedefined() as usize;
                if depth == 0 {
                    STEP_DEPTH.store(0, Ordering::SeqCst);
                } else {
                    STEP_DEPTH.store(depth + 1, Ordering::SeqCst);
                }
            }
        }
        self.install_hook();
    }

    pub fn resume(&self) {
        self.clear_pause();
        self.install_hook();
    }

    pub fn get_current_location(&self) -> (Option<String>, u32) {
        unsafe {
            let line = CURRENT_LINE.load(Ordering::SeqCst) as u32;
            (CURRENT_SOURCE.clone(), line)
        }
    }

    pub fn get_current_line(&self) -> u32 {
        unsafe { CURRENT_LINE.load(Ordering::SeqCst) as u32 }
    }

    pub fn get_current_source(&self) -> Option<String> {
        unsafe { CURRENT_SOURCE.clone() }
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

                self.install_hook();

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
        self.set_step(mode);
        Ok(())
    }

    async fn continue_(&mut self) -> Result<(), RuntimeError> {
        self.resume();
        Ok(())
    }

    async fn pause(&mut self) -> Result<(), RuntimeError> {
        unsafe {
            PAUSED.store(true, Ordering::SeqCst);
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    fn block_on<F: std::future::Future>(future: F) -> F::Output {
        Runtime::new().unwrap().block_on(future)
    }

    #[test]
    fn test_runtime_creation() {
        let runtime = PUCLuaRuntime::new();
        assert!(!runtime.is_paused());
    }

    #[test]
    fn test_breakpoint_storage() {
        block_on(async {
            let mut runtime = PUCLuaRuntime::new();

            runtime.set_breakpoint(BreakpointType::Line {
                source: "test.lua".to_string(),
                line: 10,
            }).await.unwrap();

            let breakpoints = runtime.breakpoints.lock().unwrap();
            assert!(breakpoints.contains_key("test.lua"));
            assert!(breakpoints["test.lua"].contains(&10));
        });
    }

    #[test]
    fn test_is_breakpoint_hit() {
        block_on(async {
            let mut runtime = PUCLuaRuntime::new();

            runtime.set_breakpoint(BreakpointType::Line {
                source: "test.lua".to_string(),
                line: 10,
            }).await.unwrap();
            runtime.set_breakpoint(BreakpointType::Line {
                source: "test.lua".to_string(),
                line: 20,
            }).await.unwrap();
            runtime.set_breakpoint(BreakpointType::Line {
                source: "other.lua".to_string(),
                line: 5,
            }).await.unwrap();

            assert!(runtime.is_breakpoint_hit("test.lua", 10));
            assert!(runtime.is_breakpoint_hit("test.lua", 20));
            assert!(!runtime.is_breakpoint_hit("test.lua", 15));
            assert!(!runtime.is_breakpoint_hit("other.lua", 10));
            assert!(runtime.is_breakpoint_hit("other.lua", 5));
        });
    }

    #[test]
    fn test_step_mode_conversion() {
        assert_eq!(StepMode::Over.to_u32(), 0);
        assert_eq!(StepMode::In.to_u32(), 1);
        assert_eq!(StepMode::Out.to_u32(), 2);

        assert_eq!(StepMode::from_u32(0), StepMode::Over);
        assert_eq!(StepMode::from_u32(1), StepMode::In);
        assert_eq!(StepMode::from_u32(2), StepMode::Out);
        assert_eq!(StepMode::from_u32(99), StepMode::Out);
    }

    #[test]
    fn test_execute_simple_code() {
        let runtime = PUCLuaRuntime::new();

        let result = runtime.execute_code("1 + 2");
        assert!(result.is_ok());
        let value = result.unwrap();
        match value {
            Value::Number(n) => assert_eq!(n, 3.0),
            _ => panic!("Expected Number, got {:?}", value),
        }
    }

    #[test]
    fn test_execute_string_concatenation() {
        let runtime = PUCLuaRuntime::new();

        let result = runtime.execute_code("'hello' .. ' ' .. 'world'");
        assert!(result.is_ok());
        let value = result.unwrap();
        match value {
            Value::String(s) => assert_eq!(s, "hello world"),
            _ => panic!("Expected String, got {:?}", value),
        }
    }

    #[test]
    fn test_execute_boolean() {
        let runtime = PUCLuaRuntime::new();

        let result = runtime.execute_code("true");
        assert!(result.is_ok());
        let value = result.unwrap();
        match value {
            Value::Boolean(b) => assert!(b),
            _ => panic!("Expected Boolean, got {:?}", value),
        }
    }

    #[test]
    fn test_execute_nil() {
        let runtime = PUCLuaRuntime::new();

        let result = runtime.execute_code("nil");
        assert!(result.is_ok());
        let value = result.unwrap();
        match value {
            Value::Nil => (),
            _ => panic!("Expected Nil, got {:?}", value),
        }
    }

    #[test]
    fn test_install_hook() {
        let runtime = PUCLuaRuntime::new();
        runtime.install_hook();
    }

    #[test]
    fn test_clear_pause() {
        let runtime = PUCLuaRuntime::new();
        runtime.clear_pause();
        assert!(!runtime.is_paused());
    }

    #[test]
    fn test_set_step() {
        let runtime = PUCLuaRuntime::new();
        runtime.set_step(StepMode::In);
        runtime.set_step(StepMode::Over);
        runtime.set_step(StepMode::Out);
    }

    #[test]
    fn test_lua_state_operations() {
        let mut runtime = PUCLuaRuntime::new();
        let top = runtime.get_top();
        assert_eq!(top, 0);

        runtime.set_top(5);
        let top = runtime.get_top();
        assert_eq!(top, 5);

        runtime.set_top(0);
        let top = runtime.get_top();
        assert_eq!(top, 0);
    }

    #[test]
    fn test_type_of() {
        let mut runtime = PUCLuaRuntime::new();

        runtime.execute_code("nil").ok();
        assert_eq!(runtime.type_of(-1), 0);

        runtime.execute_code("true").ok();
        assert_eq!(runtime.type_of(-1), 1);

        runtime.execute_code("123").ok();
        assert_eq!(runtime.type_of(-1), 3);

        runtime.execute_code("'hello'").ok();
        assert_eq!(runtime.type_of(-1), 4);

        runtime.set_top(0);
    }

    #[test]
    fn test_get_global() {
        let mut runtime = PUCLuaRuntime::new();

        runtime.execute_code("myvar = 42").ok();
        let count = runtime.get_global("myvar");
        assert_eq!(count, 1);

        let value = runtime.execute_code("myvar").ok();
        match value {
            Some(Value::Number(n)) => assert_eq!(n, 42.0),
            _ => panic!("Expected Number"),
        }

        runtime.set_top(0);
    }

    #[test]
    fn test_load_string() {
        let runtime = PUCLuaRuntime::new();

        let result = runtime.load_string("x = 10");
        assert!(result.is_ok());

        let result = runtime.pcall(0, 0);
        assert!(result.is_ok());

        let value = runtime.execute_code("x").ok();
        match value {
            Some(Value::Number(n)) => assert_eq!(n, 10.0),
            _ => panic!("Expected Number"),
        }
    }
}
