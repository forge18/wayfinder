use super::{super::*, BreakpointType, DebugRuntime, ExceptionInfo, LuaVersion, RuntimeError, RuntimeType, Scope, StepMode, Value};
use super::super::config::DebuggerConfig;
use super::super::debug::breakpoints::LineBreakpoint;
use super::super::debug::watchpoints::{DataBreakpoint, WatchpointManager, DataType};
use super::lua_state::Lua;
use std::sync::RwLock;

/// Check if any watchpoints have been triggered
#[allow(dead_code)]
unsafe fn check_watchpoints(_L: LuaState, _ar: *mut lua_Debug) -> bool {
    // In a complete implementation, this would:
    // 1. Access the watchpoint manager (probably through a static or passed parameter)
    // 2. Iterate through all active data breakpoints
    // 3. For each watchpoint:
    //    - Determine the variable type (local, global, upvalue, table field)
    //    - Get the current value using appropriate Lua debug API functions
    //    - Compare with the previous value
    //    - If changed and access type matches, trigger the watchpoint
    // 4. Return true if any watchpoint was triggered
    
    // For now, we'll return false as this is a complex feature that requires
    // significant implementation work
    false
}
use crate::runtime::lua_state::DebugInfo;
use crate::runtime::lua_ffi::*;

// In dynamic mode, FFI functions don't exist so we need to use wrapper methods
// Define module-level helpers that dispatch through the Lua wrapper
#[cfg(feature = "dynamic-lua")]
#[allow(dead_code)]
mod ffi_compat {
    // These helper functions take a Lua wrapper and forward to its methods
    // The calling code will need to be refactored to pass the Lua wrapper
}
use async_trait::async_trait;
use libc::c_int;
use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use once_cell::sync::Lazy;

static mut PAUSED: AtomicBool = AtomicBool::new(false);
static mut SHOULD_STEP: AtomicBool = AtomicBool::new(false);
static mut CURRENT_LINE: AtomicUsize = AtomicUsize::new(1);
static mut CURRENT_SOURCE: Option<String> = None;
static mut STEP_MODE: AtomicUsize = AtomicUsize::new(0);
static mut STEP_DEPTH: AtomicUsize = AtomicUsize::new(0);
static mut STEP_TRIGGERED: AtomicBool = AtomicBool::new(false);
// Note: Storing runtime references in static variables is not thread-safe
// This is a simplification for the prototype

// Profiler registry: maps runtime ID to active profiler
static PROFILER_REGISTRY: Lazy<Mutex<HashMap<usize, Arc<Mutex<crate::profiling::Profiler>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

// Thread-local to track current runtime ID (used in hook callback)
thread_local! {
    static CURRENT_RUNTIME_ID: std::cell::Cell<usize> = std::cell::Cell::new(0);
}

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
                },
                StepMode::Out => {
                    let depth = (*ar).linedefined as usize;
                    depth < STEP_DEPTH.load(Ordering::SeqCst)
                }
            }
        } else {
            false
        };

        // Check for watchpoint triggers
        // Note: This is a simplified approach as we can't easily pass the runtime instance
        // to the static hook callback. In a full implementation, we would need a more
        // sophisticated approach, possibly using thread-local storage or a global registry.
        let watchpoint_triggered = false; // Placeholder - would need access to runtime instance

        if triggered_for_step || watchpoint_triggered {
            STEP_TRIGGERED.store(true, Ordering::SeqCst);
            PAUSED.store(true, Ordering::SeqCst);
        }

        // Handle profiling events
        let event = (*ar).event;
        if event == LUA_HOOKCALL || event == LUA_HOOKRET || event == LUA_HOOKCOUNT {
            let runtime_id = CURRENT_RUNTIME_ID.with(|id| id.get());

            if let Ok(registry) = PROFILER_REGISTRY.lock() {
                if let Some(profiler_arc) = registry.get(&runtime_id) {
                    if let Ok(mut profiler) = profiler_arc.lock() {
                        match event {
                            LUA_HOOKCALL => {
                                // Get function information for the call event
                                let _ = lua_getinfo(_L, b"nS\0".as_ptr() as *const i8, ar);
                                let name = get_hook_function_name(ar);
                                let source = get_hook_source(ar);
                                let line = (*ar).linedefined as u32;
                                profiler.on_call(name, source, line);
                            }
                            LUA_HOOKRET => {
                                profiler.on_return();
                            }
                            LUA_HOOKCOUNT => {
                                profiler.on_sample();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

// Helper functions for profiling hook
unsafe fn get_hook_function_name(ar: *mut lua_Debug) -> String {
    if !(*ar).name.is_null() {
        if let Ok(c_str) = CStr::from_ptr((*ar).name).to_str() {
            return c_str.to_string();
        }
    }
    format!("<?:{}?>", (*ar).linedefined)
}

unsafe fn get_hook_source(ar: *mut lua_Debug) -> Option<String> {
    if !(*ar).source.is_null() {
        if let Ok(c_str) = CStr::from_ptr((*ar).source).to_str() {
            return Some(c_str.to_string());
        }
    }
    None
}

pub struct PUCLuaRuntime {
    lua: Arc<Mutex<Lua>>,
    breakpoints: Arc<Mutex<HashMap<String, Vec<u32>>>>,
    detailed_breakpoints: Arc<Mutex<HashMap<String, Vec<LineBreakpoint>>>>,
    watchpoint_manager: Arc<RwLock<WatchpointManager>>,
    watched_variable_values: Arc<Mutex<HashMap<String, String>>>,
    config: DebuggerConfig,
    step_mode: Arc<Mutex<StepMode>>,
}

impl PUCLuaRuntime {
    #[cfg(feature = "static-lua")]
    pub fn new() -> Self {
        unsafe {
            PAUSED.store(false, Ordering::SeqCst);
            SHOULD_STEP.store(false, Ordering::SeqCst);
            CURRENT_LINE.store(1, Ordering::SeqCst);
        }

        let lua = Arc::new(Mutex::new(Lua::new()));

        Self {
            lua,
            breakpoints: Arc::new(Mutex::new(HashMap::new())),
            detailed_breakpoints: Arc::new(Mutex::new(HashMap::new())),
            watchpoint_manager: Arc::new(RwLock::new(WatchpointManager::new())),
            watched_variable_values: Arc::new(Mutex::new(HashMap::new())),
            config: DebuggerConfig::default(),
            step_mode: Arc::new(Mutex::new(StepMode::Over)),
        }
    }

    #[cfg(feature = "dynamic-lua")]
    pub fn new_with_library(lib: crate::runtime::lua_loader::LuaLibrary) -> Self {
        unsafe {
            PAUSED.store(false, Ordering::SeqCst);
            SHOULD_STEP.store(false, Ordering::SeqCst);
            CURRENT_LINE.store(1, Ordering::SeqCst);
        }

        let lua = Arc::new(Mutex::new(Lua::new_with_library(lib)));

        Self {
            lua,
            breakpoints: Arc::new(Mutex::new(HashMap::new())),
            detailed_breakpoints: Arc::new(Mutex::new(HashMap::new())),
            watchpoint_manager: Arc::new(RwLock::new(WatchpointManager::new())),
            watched_variable_values: Arc::new(Mutex::new(HashMap::new())),
            config: DebuggerConfig::default(),
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
            let ptr = lua.lua_getlocal(ar.ptr(), n);
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
            let ptr = lua.lua_getupvalue(func_index, n);
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
            let result = lua.lua_getinfo(b"SnSluf\0".as_ptr() as *const i8, ar.ptr());

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
            lua.lua_sethook(lua_hook_callback, LUA_MASKLINE, 0);
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

            let lua = self.lua.lock().unwrap();
            let mut ar = DebugInfo::new();
            if lua.lua_getinfo(b"n\0".as_ptr() as *const i8, ar.ptr()) != 0 {
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

    async fn hot_reload(
        &mut self,
        module_source: &str,
        module_name: Option<&str>,
    ) -> Result<crate::hot_reload::HotReloadResult, RuntimeError> {
        #[cfg(feature = "hot-reload")]
        {
            use crate::hot_reload::{HotReloadResult, HotReloadWarning, WarningSeverity};
            use crate::runtime::lua_ffi::*;

            // Compile the module source
            let compile_result: Result<(), RuntimeError> = {
                let lua_guard = self.lua.lock().unwrap();

                unsafe {
                    let source_cstr = std::ffi::CString::new(module_source)
                        .map_err(|_| RuntimeError::Communication("Invalid source string".to_string()))?;

                    if lua_guard.luaL_loadstring(source_cstr.as_ptr()) != LUA_OK as i32 {
                        // Get the error message
                        let error_msg = if lua_guard.lua_type(-1) == LUA_TSTRING as i32 {
                            let c_str = lua_guard.lua_tolstring(-1, std::ptr::null_mut());
                            if !c_str.is_null() {
                                std::ffi::CStr::from_ptr(c_str)
                                    .to_string_lossy()
                                    .to_string()
                            } else {
                                "Unknown compilation error".to_string()
                            }
                        } else {
                            "Unknown compilation error".to_string()
                        };

                        lua_guard.lua_pop(1); // Remove error message
                        return Err(RuntimeError::Communication(format!("Compilation failed: {}", error_msg)));
                    }
                    Ok(())
                }
            };

            compile_result?;

            // Execute the compiled module
            let execute_result: Result<(), RuntimeError> = {
                let lua_guard = self.lua.lock().unwrap();

                unsafe {
                    if lua_guard.lua_pcall(0, 1, 0) != LUA_OK as i32 {
                        // Get the error message
                        let error_msg = if lua_guard.lua_type(-1) == LUA_TSTRING as i32 {
                            let c_str = lua_guard.lua_tolstring(-1, std::ptr::null_mut());
                            if !c_str.is_null() {
                                std::ffi::CStr::from_ptr(c_str)
                                    .to_string_lossy()
                                    .to_string()
                            } else {
                                "Unknown execution error".to_string()
                            }
                        } else {
                            "Unknown execution error".to_string()
                        };

                        lua_guard.lua_pop(1); // Remove error message
                        return Err(RuntimeError::Communication(format!("Execution failed: {}", error_msg)));
                    }

                    // Pop the result
                    lua_guard.lua_pop(1);
                    Ok(())
                }
            };

            execute_result?;

            // Create warnings about limitations
            let warnings = vec![
                HotReloadWarning {
                    message: "State preservation not yet implemented - local variables and upvalues will be reset".to_string(),
                    severity: WarningSeverity::Warning,
                },
                HotReloadWarning {
                    message: "Module references in existing closures will not be updated".to_string(),
                    severity: WarningSeverity::Warning,
                }
            ];

            Ok(HotReloadResult {
                success: true,
                warnings,
                message: Some(format!("Module '{}' reloaded successfully",
                                    module_name.unwrap_or("unnamed"))),
            })
        }

        #[cfg(not(feature = "hot-reload"))]
        {
            let _ = (module_source, module_name);
            Err(RuntimeError::NotImplemented("Hot reload feature not enabled".to_string()))
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
                let result = lua.lua_getinfo(b"nSluf\0".as_ptr() as *const i8, ar.ptr());

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
        let mut lua = self.lua.lock().unwrap();

        if variables_reference >= 0 {
            // Handle local variables using debug.getlocal
            unsafe {
                // For local variables, variables_reference represents the frame ID
                let frame_id = variables_reference as c_int;
                
                // Create a debug info structure for the specified frame
                let mut ar = std::mem::zeroed::<lua_Debug>();
                // Get stack info for the frame
                if lua.lua_getstack(frame_id, &mut ar) != 0 {
                    // Enumerate local variables using lua_getlocal
                    let mut index = 1i32;
                    loop {
                        // Get local variable name and value
                        let name_ptr = lua.lua_getlocal(&mut ar, index);
                        
                        if name_ptr.is_null() {
                            break; // No more local variables
                        }
                        
                        // Get the local variable name
                        let name_cstr = CStr::from_ptr(name_ptr);
                        let name = name_cstr.to_string_lossy().to_string();
                        
                        // Skip special variables that start with "(" like "(temporary)"
                        if !name.starts_with("(") {
                            // Get the local variable value (it's on top of the stack)
                            let value_type = lua.type_of(-1);
                            let value_str = match value_type {
                                0 => "nil".to_string(),
                                1 => format!("{}", lua.pop_boolean()),
                                3 => format!("{}", lua.pop_number()),
                                4 => lua.pop_string(),
                                5 => format!("table: 0x{:x}", lua.topointer(-1) as usize),
                                6 => format!("function: 0x{:x}", lua.topointer(-1) as usize),
                                7 => format!("userdata: 0x{:x}", lua.topointer(-1) as usize),
                                8 => format!("thread: 0x{:x}", lua.topointer(-1) as usize),
                                _ => format!("{}", lua.type_name(value_type)),
                            };

                            variables.push(super::Variable {
                                name,
                                value: value_str,
                                type_: lua.type_name(value_type).to_string(),
                                variables_reference: if value_type == 5 { Some(-(variables_reference * 1000 + index as i64)) } else { None },
                                named_variables: None,
                                indexed_variables: None,
                            });
                        }
                        
                        // Remove the value from the stack
                        lua.lua_settop(-2);
                        
                        index += 1;
                    }
                }
            }
        } else if variables_reference == -1 {
            // Handle global variables by accessing _G
            unsafe {
                // Push "_G" string and get the global table
                let g_name = b"_G\0".as_ptr() as *const i8;
                if lua.lua_getglobal(g_name) == 0 {
                    // _G doesn't exist or is nil, remove it from stack
                    lua.lua_settop(-2);
                } else {
                    // Successfully got _G table, iterate it
                    lua.push_nil(); // First key
                    let mut count = 0;
                    while lua.lua_next(-2) != 0 && count < 100 {
                        let key = lua.pop_string();
                        let value_type = lua.type_of(-1);
                        let value_str = match value_type {
                            0 => "nil".to_string(),
                            1 => format!("{}", lua.pop_boolean()),
                            3 => format!("{}", lua.pop_number()),
                            4 => lua.pop_string(),
                            5 => format!("table: 0x{:x}", lua.topointer(-1) as usize),
                            6 => format!("function: 0x{:x}", lua.topointer(-1) as usize),
                            7 => format!("userdata: 0x{:x}", lua.topointer(-1) as usize),
                            8 => format!("thread: 0x{:x}", lua.topointer(-1) as usize),
                            _ => format!("{}", lua.type_name(value_type)),
                        };

                        variables.push(super::Variable {
                            name: key,
                            value: value_str,
                            type_: lua.type_name(value_type).to_string(),
                            variables_reference: if value_type == 5 { Some(-2) } else { None },
                            named_variables: None,
                            indexed_variables: None,
                        });
                        
                        // Remove value, keep key for next iteration
                        lua.lua_settop(-2);
                        count += 1;
                    }
                    
                    // Remove _G table from stack
                    lua.lua_settop(-2);
                }
            }
        } else if variables_reference < -1000 {
            // Handle upvalues - negative values less than -1000 represent upvalues
            // Format: -(frame_id * 1000 + local_index)
            let abs_ref = -variables_reference;
            let frame_id = (abs_ref / 1000) as c_int;
            // let local_index = (abs_ref % 1000) as c_int;
            
            unsafe {
                let mut ar = std::mem::zeroed::<lua_Debug>();
                if lua.lua_getstack(frame_id, &mut ar) != 0 {
                    // Get upvalues using lua_getupvalue
                    let mut index = 1i32;
                    loop {
                        let name_ptr = lua.lua_getupvalue(-1, index);
                        
                        if name_ptr.is_null() {
                            break; // No more upvalues
                        }
                        
                        // Get the upvalue name
                        let name_cstr = CStr::from_ptr(name_ptr);
                        let name = name_cstr.to_string_lossy().to_string();
                        
                        // Get the upvalue value (it's on top of the stack)
                        let value_type = lua.type_of(-1);
                        let value_str = match value_type {
                            0 => "nil".to_string(),
                            1 => format!("{}", lua.pop_boolean()),
                            3 => format!("{}", lua.pop_number()),
                            4 => lua.pop_string(),
                            5 => format!("table: 0x{:x}", lua.topointer(-1) as usize),
                            6 => format!("function: 0x{:x}", lua.topointer(-1) as usize),
                            7 => format!("userdata: 0x{:x}", lua.topointer(-1) as usize),
                            8 => format!("thread: 0x{:x}", lua.topointer(-1) as usize),
                            _ => format!("{}", lua.type_name(value_type)),
                        };

                        variables.push(super::Variable {
                            name,
                            value: value_str,
                            type_: lua.type_name(value_type).to_string(),
                            variables_reference: if value_type == 5 { Some(-(variables_reference * 100 + index as i64)) } else { None },
                            named_variables: None,
                            indexed_variables: None,
                        });
                        
                        // Remove the value from the stack
                        lua.lua_settop(-2);
                        
                        index += 1;
                    }
                }
            }
        } else if variables_reference == -2 {
            // Handle table expansion with depth limits
            unsafe {
                // The table is already on the stack (placed there by the caller)
                // Limit the number of elements we show to prevent huge expansions
                lua.push_nil(); // First key
                let mut count = 0;
                while lua.lua_next(-2) != 0 && count < 50 {
                    let key = lua.pop_string();
                    let value_type = lua.type_of(-1);
                    let value_str = match value_type {
                        0 => "nil".to_string(),
                        1 => format!("{}", lua.pop_boolean()),
                        3 => format!("{}", lua.pop_number()),
                        4 => lua.pop_string(),
                        5 => format!("table: 0x{:x}", lua.topointer(-1) as usize),
                        6 => format!("function: 0x{:x}", lua.topointer(-1) as usize),
                        7 => format!("userdata: 0x{:x}", lua.topointer(-1) as usize),
                        8 => format!("thread: 0x{:x}", lua.topointer(-1) as usize),
                        _ => format!("{}", lua.type_name(value_type)),
                    };

                    variables.push(super::Variable {
                        name: key,
                        value: value_str,
                        type_: lua.type_name(value_type).to_string(),
                        variables_reference: if value_type == 5 { Some(-2) } else { None },
                        named_variables: None,
                        indexed_variables: None,
                    });
                    
                    // Remove value, keep key for next iteration
                    lua.lua_settop(-2);
                    count += 1;
                }
            }
        }

        Ok(variables)
    }

    async fn evaluate(&mut self, frame_id: i64, expression: &str) -> Result<Value, RuntimeError> {
        let trimmed = expression.trim();

        if trimmed.is_empty() {
            return Ok(Value::Nil);
        }

        // Check if this is an assignment operation
        let is_assignment = trimmed.contains('=') && !trimmed.contains("==") && !trimmed.contains("!=");
        let is_dangerous_function = trimmed.contains("load") || trimmed.contains("dofile") || trimmed.contains("require");

        // Apply safety checks based on configuration
        match self.config.eval_safety {
            EvalSafety::Strict => {
                // In strict mode, prevent all assignments and dangerous functions
                if is_assignment {
                    return Err(RuntimeError::Communication(
                        "Assignment not allowed in strict evaluation mode".to_string()
                    ));
                }
                if is_dangerous_function {
                    return Err(RuntimeError::Communication(
                        "Dangerous function calls not allowed in strict evaluation mode".to_string()
                    ));
                }
            }
            EvalSafety::Basic => {
                // In basic mode, warn about assignments and dangerous functions
                if is_assignment {
                    println!("Warning: Assignment detected in expression evaluation: {}", trimmed);
                }
                if is_dangerous_function {
                    println!("Warning: Potentially dangerous function call detected: {}", trimmed);
                }
            }
            EvalSafety::None => {
                // In none mode, allow everything but still log
                if is_assignment {
                    println!("Info: Assignment in expression evaluation: {}", trimmed);
                }
                if is_dangerous_function {
                    println!("Info: Function call detected: {}", trimmed);
                }
            }
        }

        // If mutation is enabled and this is an assignment, try to handle it properly
        if self.config.evaluate_mutation && is_assignment {
            if let Some(result) = self.handle_assignment(frame_id, trimmed).await {
                return result;
            }
        }

        // Execute the expression
        let mut lua = self.lua.lock().unwrap();
        if let Ok(_) = lua.execute(trimmed) {
            // Convert the result on top of stack to our Value type
            let result = Self::lua_to_value(&mut lua, -1);
            return Ok(result);
        }

        // Handle literal values
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

    async fn get_exception_info(&mut self, _thread_id: u64) -> Result<ExceptionInfo, RuntimeError> {
        Err(RuntimeError::NotImplemented("get_exception_info not implemented".to_string()))
    }

    async fn check_data_breakpoints(&mut self, frame_id: i64) -> Result<bool, RuntimeError> {
        // Call the internal check_watchpoints method
        Ok(self.check_watchpoints(frame_id))
    }

    async fn get_memory_statistics(&self) -> Result<crate::memory::MemoryStatistics, RuntimeError> {
        use crate::runtime::lua_ffi::*;
        use std::time::SystemTime;

        let lua = self.lua.lock().unwrap();
        let state = lua.state();

        let kb = unsafe { lua_gc(state, LUA_GCCOUNT, 0, 0) };
        let bytes = unsafe { lua_gc(state, LUA_GCCOUNTB, 0, 0) };
        let pause = unsafe { lua_gc(state, LUA_GCSETPAUSE, 0, 0) };
        let step_mul = unsafe { lua_gc(state, LUA_GCSETSTEPMUL, 0, 0) };
        let running = unsafe { lua_gc(state, LUA_GCISRUNNING, 0, 0) };

        Ok(crate::memory::MemoryStatistics {
            total_kb: kb as f64 + (bytes as f64 / 1024.0),
            total_bytes: (kb * 1024 + bytes) as usize,
            gc_pause: pause,
            gc_step_mul: step_mul,
            gc_running: running != 0,
            timestamp: SystemTime::now(),
        })
    }

    async fn force_gc(&mut self) -> Result<(), RuntimeError> {
        use crate::runtime::lua_ffi::*;

        let lua = self.lua.lock().unwrap();
        let state = lua.state();

        unsafe {
            lua_gc(state, LUA_GCCOLLECT, 0, 0);
        }
        Ok(())
    }

    async fn start_profiling(&mut self, mode: crate::profiling::ProfilingMode) -> Result<(), RuntimeError> {
        use crate::runtime::lua_ffi::*;

        let runtime_id = self as *const _ as usize;
        CURRENT_RUNTIME_ID.with(|id| id.set(runtime_id));

        let profiler = Arc::new(Mutex::new(crate::profiling::Profiler::new(mode)));
        PROFILER_REGISTRY.lock().unwrap().insert(runtime_id, profiler);

        let lua = self.lua.lock().unwrap();
        let state = lua.state();

        // Update hook mask based on profiling mode
        match mode {
            crate::profiling::ProfilingMode::Sampling { interval_ms } => {
                unsafe {
                    lua_sethook(state, lua_hook_callback, LUA_MASKCOUNT, interval_ms as i32);
                }
            }
            crate::profiling::ProfilingMode::CallTrace => {
                unsafe {
                    lua_sethook(state, lua_hook_callback, LUA_MASKLINE | LUA_MASKCALL | LUA_MASKRET, 0);
                }
            }
            crate::profiling::ProfilingMode::LineLevel => {
                unsafe {
                    lua_sethook(state, lua_hook_callback, LUA_MASKLINE | LUA_MASKCALL | LUA_MASKRET, 0);
                }
            }
            crate::profiling::ProfilingMode::Disabled => return Ok(()),
        }

        Ok(())
    }

    async fn stop_profiling(&mut self) -> Result<crate::profiling::ProfileData, RuntimeError> {
        use crate::runtime::lua_ffi::*;

        let runtime_id = self as *const _ as usize;

        let profiler_arc = PROFILER_REGISTRY.lock().unwrap()
            .remove(&runtime_id)
            .ok_or(RuntimeError::Communication("No active profiler".into()))?;

        // Get the profile data from the Arc<Mutex>
        let data = {
            let profiler_guard = profiler_arc.lock().unwrap();
            profiler_guard.to_profile_data()
        };

        let lua = self.lua.lock().unwrap();
        let state = lua.state();

        // Reset hook to line-only mode for stepping
        unsafe {
            lua_sethook(state, lua_hook_callback, LUA_MASKLINE, 0);
        }

        Ok(data)
    }

    async fn get_profile_snapshot(&self) -> Result<Option<crate::profiling::ProfileData>, RuntimeError> {
        let runtime_id = self as *const _ as usize;

        let registry = PROFILER_REGISTRY.lock().unwrap();
        if let Some(profiler_arc) = registry.get(&runtime_id) {
            let profiler = profiler_arc.lock().unwrap();
            // Create snapshot without finishing
            Ok(Some(crate::profiling::ProfileData {
                mode: profiler.mode(),
                duration_ms: profiler.elapsed().as_secs_f64() * 1000.0,
                functions: profiler.functions().clone(),
                total_samples: profiler.sample_count(),
            }))
        } else {
            Ok(None)
        }
    }
}

impl PUCLuaRuntime {
    /// Sets a data breakpoint in the runtime
    pub async fn set_data_breakpoint(&mut self, data_breakpoint: DataBreakpoint) -> Result<Breakpoint, RuntimeError> {
        // Store the data breakpoint in our watchpoint manager
        let mut watchpoint_manager = self.watchpoint_manager.write().unwrap();
        let breakpoints = vec![data_breakpoint];
        watchpoint_manager.set_data_breakpoints(breakpoints);

        // Install hook if not already installed
        self.install_hook();

        Ok(Breakpoint {
            id: 1,
            verified: true,
            line: 0,
            message: Some("Data breakpoint set".to_string()),
        })
    }

    /// Check if any watchpoints have been triggered
    fn check_watchpoints(&self, frame_id: i64) -> bool {
        // Get the list of watchpoint IDs and their data types first
        let watchpoint_info: Vec<(i64, DataType)> = {
            let watchpoint_manager = self.watchpoint_manager.read().unwrap();
            watchpoint_manager.get_data_breakpoints()
                .iter()
                .map(|wp| (wp.id, wp.data_type.clone()))
                .collect()
        };
        
        // Check each watchpoint
        for (id, data_type) in watchpoint_info {
            // Get the current value based on the data type
            let current_value = match &data_type {
                DataType::Local => {
                    // We need the name for local variables, but we don't have it here
                    // Let's get it from the watchpoint manager
                    let watchpoint_manager = self.watchpoint_manager.read().unwrap();
                    if let Some(wp) = watchpoint_manager.find_data_breakpoint(id) {
                        self.get_local_variable_value(frame_id, &wp.name)
                    } else {
                        None
                    }
                },
                DataType::Global => {
                    let watchpoint_manager = self.watchpoint_manager.read().unwrap();
                    if let Some(wp) = watchpoint_manager.find_data_breakpoint(id) {
                        self.get_global_variable_value(&wp.name)
                    } else {
                        None
                    }
                },
                DataType::Upvalue => {
                    let watchpoint_manager = self.watchpoint_manager.read().unwrap();
                    if let Some(wp) = watchpoint_manager.find_data_breakpoint(id) {
                        self.get_upvalue_value(&wp.name)
                    } else {
                        None
                    }
                },
                DataType::UpvalueId { function_index, upvalue_index, upvalue_id: _ } => {
                    self.get_upvalue_id_value(*function_index, *upvalue_index)
                },
                DataType::TableField { table_ref, field } => {
                    self.get_table_field_value(*table_ref, field)
                }
            };
            
            // If we got a current value, check if it has changed
            if let Some(value) = current_value {
                // Check if the value has changed
                let has_changed = {
                    let watchpoint_manager = self.watchpoint_manager.read().unwrap();
                    watchpoint_manager.has_data_breakpoint_value_changed(id, &value)
                };
                
                if has_changed {
                    // Value has changed, update the previous value
                    let mut watchpoint_manager = self.watchpoint_manager.write().unwrap();
                    watchpoint_manager.update_data_breakpoint_previous_value(id, value.clone());
                    
                    // Check access type - for now we'll assume we're monitoring writes
                    return true; // Trigger the watchpoint
                }
            }
        }
        
        false
    }

    /// Sets the debugger configuration
    pub fn set_config(&mut self, config: DebuggerConfig) {
        self.config = config;
    }

    /// Gets the debugger configuration
    pub fn config(&self) -> &DebuggerConfig {
        &self.config
    }

    /// Check if any watchpoints have been triggered (public method)
    pub async fn check_data_breakpoints(&self, frame_id: i64) -> Result<bool, RuntimeError> {
        // Call the internal check_watchpoints method
        Ok(self.check_watchpoints(frame_id))
    }

    /// Gets the current value of a local variable
    fn get_local_variable_value(&self, frame_id: i64, variable_name: &str) -> Option<String> {
        let mut lua = self.lua.lock().unwrap();
        
        // Create debug info structure for the specified frame
        let mut ar = unsafe { std::mem::zeroed::<lua_Debug>() };
        if lua.get_stack(frame_id as c_int, &mut ar) != 0 {
            // Search for the variable in local scope
            let mut index = 1i32;
            loop {
                // Get local variable name
                let name_opt = lua.get_local(&mut ar, index);
                
                match name_opt {
                    Some(name) => {
                        if name == variable_name {
                            // Found the variable, get its value
                            // Convert the value to a string representation
                            let value_type = lua.type_of(-1);
                            let value_str = match value_type {
                                0 => "nil".to_string(), // nil
                                1 => {
                                    // boolean
                                    if lua.pop_boolean() {
                                        "true".to_string()
                                    } else {
                                        "false".to_string()
                                    }
                                },
                                3 => {
                                    // number
                                    lua.pop_number().to_string()
                                },
                                4 => {
                                    // string
                                    format!("\"{}\"", lua.pop_string())
                                },
                                5 => {
                                    // table
                                    format!("table:0x{:x}", lua.topointer(-1) as usize)
                                },
                                6 => {
                                    // function
                                    format!("function:0x{:x}", lua.topointer(-1) as usize)
                                },
                                7 => {
                                    // userdata
                                    format!("userdata:0x{:x}", lua.topointer(-1) as usize)
                                },
                                8 => {
                                    // thread
                                    format!("thread:0x{:x}", lua.topointer(-1) as usize)
                                },
                                _ => format!("unknown:{}", lua.type_name(value_type)),
                            };
                            
                            // Remove the value from stack
                            lua.set_top(-2);
                            return Some(value_str);
                        }
                        
                        // Remove the local variable value from stack
                        lua.set_top(-2);
                        index += 1;
                    }
                    None => {
                        // No more local variables, break the loop
                        break;
                    }
                }
            }
        }
        
        None
    }

    /// Gets the current value of an upvalue
    fn get_upvalue_value(&self, variable_name: &str) -> Option<String> {
        let mut lua = self.lua.lock().unwrap();
        
        // Try to get the current function (assuming it's at the top of stack)
        let func_index = -1;
        let mut index = 1i32;
        loop {
            let name_opt = lua.get_upvalue(func_index, index);
            
            match name_opt {
                Some(name) => {
                    if name == variable_name {
                        // Found the upvalue, get its value
                        // Convert the value to a string representation
                        let value_type = lua.type_of(-1);
                        let value_str = match value_type {
                            0 => "nil".to_string(), // nil
                            1 => {
                                // boolean
                                if lua.pop_boolean() {
                                    "true".to_string()
                                } else {
                                    "false".to_string()
                                }
                            },
                            3 => {
                                // number
                                lua.pop_number().to_string()
                            },
                            4 => {
                                // string
                                format!("\"{}\"", lua.pop_string())
                            },
                            5 => {
                                // table
                                format!("table:0x{:x}", lua.topointer(-1) as usize)
                            },
                            6 => {
                                // function
                                format!("function:0x{:x}", lua.topointer(-1) as usize)
                            },
                            7 => {
                                // userdata
                                format!("userdata:0x{:x}", lua.topointer(-1) as usize)
                            },
                            8 => {
                                // thread
                                format!("thread:0x{:x}", lua.topointer(-1) as usize)
                            },
                            _ => format!("unknown:{}", lua.type_name(value_type)),
                        };
                        
                        // Remove the value from stack
                        lua.set_top(-2);
                        return Some(value_str);
                    }
                    
                    // Remove the upvalue from stack
                    lua.set_top(-2);
                    index += 1;
                }
                None => {
                    // No more upvalues, break the loop
                    break;
                }
            }
        }
        
        None
    }

    /// Gets the current value of an upvalue identified by its ID
    fn get_upvalue_id_value(&self, function_index: i32, upvalue_index: i32) -> Option<String> {
        let mut lua = self.lua.lock().unwrap();
        
        // Get the upvalue ID to verify it's the same upvalue
        let _upvalue_id = lua.upvalue_id(function_index, upvalue_index) as usize;
        
        // Get the upvalue value
        let name_opt = lua.get_upvalue(function_index, upvalue_index);
        
        if name_opt.is_some() {
            // Convert the value to a string representation
            let value_type = lua.type_of(-1);
            let value_str = match value_type {
                0 => "nil".to_string(), // nil
                1 => {
                    // boolean
                    if lua.pop_boolean() {
                        "true".to_string()
                    } else {
                        "false".to_string()
                    }
                },
                3 => {
                    // number
                    lua.pop_number().to_string()
                },
                4 => {
                    // string
                    format!("\"{}\"", lua.pop_string())
                },
                5 => {
                    // table
                    format!("table:0x{:x}", lua.topointer(-1) as usize)
                },
                6 => {
                    // function
                    format!("function:0x{:x}", lua.topointer(-1) as usize)
                },
                7 => {
                    // userdata
                    format!("userdata:0x{:x}", lua.topointer(-1) as usize)
                },
                8 => {
                    // thread
                    format!("thread:0x{:x}", lua.topointer(-1) as usize)
                },
                _ => format!("unknown:{}", lua.type_name(value_type)),
            };
            
            // Remove the value from stack
            lua.set_top(-2);
            Some(value_str)
        } else {
            None
        }
    }

    /// Creates a watched table that intercepts field access
    fn create_watched_table(&self, table_ref: i64, field: &str) -> Result<(), RuntimeError> {
        let _lua = self.lua.lock().unwrap();
        
        // This is a simplified implementation that would need to be expanded
        // In a full implementation, this would:
        // 1. Create a proxy table with __index and __newindex metamethods
        // 2. Store the original table reference
        // 3. Set up the metamethods to call back to the watchpoint system
        // 4. Replace the original table with the proxy
        
        // For now, we'll just log that a table field is being watched
        println!("Watching table field: table_ref={}, field={}", table_ref, field);
        
        Ok(())
    }

    /// Gets the current value of a table field
    fn get_table_field_value(&self, _table_ref: i64, field: &str) -> Option<String> {
        let mut lua = self.lua.lock().unwrap();
        
        // Push the table onto the stack (this is simplified - in reality we'd need the actual reference)
        // For now, we'll assume the table is accessible somehow
        // In a full implementation, we'd need to track table references properly
        
        // Push the field name
        let field_cstr = std::ffi::CString::new(field).ok()?;
        unsafe {
            lua.lua_pushstring(field_cstr.as_ptr());
            
            // Get the table field value
            if lua.lua_gettable(-2) == 0 { // -2 would be the table index
            // Got the field value, convert to string representation
            let value_type = lua.type_of(-1);
            let value_str = match value_type {
                0 => "nil".to_string(), // nil
                1 => {
                    // boolean
                    if lua.pop_boolean() {
                        "true".to_string()
                    } else {
                        "false".to_string()
                    }
                },
                3 => {
                    // number
                    lua.pop_number().to_string()
                },
                4 => {
                    // string
                    format!("\"{}\"", lua.pop_string())
                },
                5 => {
                    // table
                    format!("table:0x{:x}", lua.topointer(-1) as usize)
                },
                6 => {
                    // function
                    format!("function:0x{:x}", lua.topointer(-1) as usize)
                },
                7 => {
                    // userdata
                    format!("userdata:0x{:x}", lua.topointer(-1) as usize)
                },
                8 => {
                    // thread
                    format!("thread:0x{:x}", lua.topointer(-1) as usize)
                },
                _ => format!("unknown:{}", lua.type_name(value_type)),
            };
            
            // Remove the value from stack
            lua.set_top(-2);
            Some(value_str)
        } else {
            // Failed to get table field
            lua.set_top(-2);
            None
        }
        } // End of unsafe block
    }

    /// Gets the current value of a global variable
    fn get_global_variable_value(&self, variable_name: &str) -> Option<String> {
        let mut lua = self.lua.lock().unwrap();
        
        // Push the variable name and get the global value
        let var_name_cstr = std::ffi::CString::new(variable_name).ok()?;
        let result = unsafe { lua.lua_getglobal(var_name_cstr.as_ptr()) };
        
        if result != 0 {
            // Got the global variable, convert to string representation
            let value_type = lua.type_of(-1);
            let value_str = match value_type {
                0 => "nil".to_string(), // nil
                1 => {
                    // boolean
                    if lua.pop_boolean() {
                        "true".to_string()
                    } else {
                        "false".to_string()
                    }
                },
                3 => {
                    // number
                    lua.pop_number().to_string()
                },
                4 => {
                    // string
                    format!("\"{}\"", lua.pop_string())
                },
                5 => {
                    // table
                    format!("table:0x{:x}", lua.topointer(-1) as usize)
                },
                6 => {
                    // function
                    format!("function:0x{:x}", lua.topointer(-1) as usize)
                },
                7 => {
                    // userdata
                    format!("userdata:0x{:x}", lua.topointer(-1) as usize)
                },
                8 => {
                    // thread
                    format!("thread:0x{:x}", lua.topointer(-1) as usize)
                },
                _ => format!("unknown:{}", lua.type_name(value_type)),
            };
            
            // Remove the value from stack
            lua.set_top(-2);
            Some(value_str)
        } else {
            // Failed to get global variable
            lua.set_top(-2);
            None
        }
    }

    /// Handle assignment expressions when mutation is enabled
    async fn handle_assignment(&self, frame_id: i64, expression: &str) -> Option<Result<Value, RuntimeError>> {
        // Parse the assignment expression (e.g., "x = 10" or "y = x + 5")
        let parts: Vec<&str> = expression.splitn(2, '=').collect();
        if parts.len() != 2 {
            return None;
        }

        let variable_name = parts[0].trim();
        let value_expression = parts[1].trim();

        // Try to set the variable using debug.setlocal or debug.setupvalue
        match self.set_variable_value(frame_id, variable_name, value_expression).await {
            Ok(value) => Some(Ok(value)),
            Err(e) => Some(Err(e)),
        }
    }

    /// Set a variable value using debug.setlocal or debug.setupvalue
    async fn set_variable_value(&self, frame_id: i64, variable_name: &str, value_expression: &str) -> Result<Value, RuntimeError> {
        // First, evaluate the value expression to get the actual value
        let value_result = {
            let mut lua = self.lua.lock().unwrap();
            if let Err(_) = lua.execute(value_expression) {
                return Err(RuntimeError::Communication(
                    format!("Failed to evaluate value expression: {}", value_expression)
                ));
            }
            Self::lua_to_value(&mut lua, -1)
        };

        // Try to find and set the variable using debug API
        {
            let mut lua = self.lua.lock().unwrap();
            
            // Create debug info structure for the specified frame
            let mut ar = unsafe { std::mem::zeroed::<lua_Debug>() };
            if lua.get_stack(frame_id as c_int, &mut ar) != 0 {
                // Search for the variable in local scope
                let mut index = 1i32;
                loop {
                // Get local variable name
                let name_opt = lua.get_local(&mut ar, index);
                    
                    match name_opt {
                        Some(name) => {
                            if name == variable_name {
                                // Found the variable, set its value
                                // The value is already on top of the stack from our earlier evaluation
                                let set_result = lua.set_local(&mut ar, index);
                                if set_result.is_some() {
                                    if self.config.show_modifications {
                                        println!("Modified local variable '{}' to value {:?}", variable_name, value_result);
                                    }
                                    return Ok(value_result);
                                }
                            }
                            
                            // Remove the local variable value from stack
                            lua.set_top(-2);
                            index += 1;
                        }
                        None => {
                            // No more local variables, break the loop
                            break;
                        }
                    }
                }
                
                // If not found in locals, search upvalues
                // Get the function at the top of the stack (current function)
                let func_index = -1; // Assuming function is at top of stack
                let mut index = 1i32;
                loop {
                    let name_opt = lua.get_upvalue(func_index, index);
                    
                    match name_opt {
                        Some(name) => {
                            if name == variable_name {
                                // Found the upvalue, set its value
                                // The value is already on top of the stack
                                let set_result = lua.set_upvalue(func_index, index);
                                if set_result.is_some() {
                                    if self.config.show_modifications {
                                        println!("Modified upvalue '{}' to value {:?}", variable_name, value_result);
                                    }
                                    return Ok(value_result);
                                }
                            }
                            
                            // Remove the upvalue from stack
                            lua.set_top(-2);
                            index += 1;
                        }
                        None => {
                            // No more upvalues, break the loop
                            break;
                        }
                    }
                }
            }
        }
        
        // If not found in locals or upvalues, treat as global variable
        let assignment_expr = format!("{} = {}", variable_name, value_expression);
        let mut lua = self.lua.lock().unwrap();
        if let Err(_) = lua.execute(&assignment_expr) {
            return Err(RuntimeError::Communication(
                format!("Failed to execute assignment: {}", assignment_expr)
            ));
        }
        
        if self.config.show_modifications {
            println!("Modified variable '{}' to value {:?}", variable_name, value_result);
        }

        Ok(value_result)
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
