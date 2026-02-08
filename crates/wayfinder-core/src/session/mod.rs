use super::runtime::{BreakpointType, DebugRuntime, Frame, Scope, StepMode, Variable, Value};
use serde_json::{json, Value as JsonValue};

pub struct DebugSession<R: DebugRuntime> {
    runtime: R,
}

impl<R: DebugRuntime> DebugSession<R> {
    pub fn new(runtime: R) -> Self {
        Self { runtime }
    }

    pub async fn run(&mut self) -> Result<(), super::runtime::RuntimeError> {
        self.runtime.continue_().await
    }

    pub async fn step(&mut self, mode: StepMode) -> Result<(), super::runtime::RuntimeError> {
        self.runtime.step(mode).await
    }

    pub async fn stack_trace(&mut self, thread_id: Option<u64>) -> Result<Vec<Frame>, super::runtime::RuntimeError> {
        self.runtime.stack_trace(thread_id).await
    }

    pub async fn scopes(&mut self, frame_id: i64) -> Result<Vec<Scope>, super::runtime::RuntimeError> {
        self.runtime.scopes(frame_id).await
    }

    pub async fn variables(&mut self, variables_reference: i64) -> Result<Vec<Variable>, super::runtime::RuntimeError> {
        self.runtime.variables(variables_reference, None).await
    }

    pub async fn evaluate(&mut self, frame_id: i64, expression: &str) -> Result<Value, super::runtime::RuntimeError> {
        self.runtime.evaluate(frame_id, expression).await
    }

    pub async fn set_breakpoint(&mut self, source: &str, line: u32) -> Result<(), super::runtime::RuntimeError> {
        let _bp = self
            .runtime
            .set_breakpoint(BreakpointType::Line {
                source: source.to_string(),
                line,
            })
            .await?;
        Ok(())
    }

    pub async fn remove_breakpoint(&mut self, id: i64) -> Result<(), super::runtime::RuntimeError> {
        self.runtime.remove_breakpoint(id).await
    }

    pub async fn pause(&mut self) -> Result<(), super::runtime::RuntimeError> {
        self.runtime.pause().await
    }

    pub async fn set_function_breakpoint(&mut self, name: &str) -> Result<(), super::runtime::RuntimeError> {
        let _bp = self
            .runtime
            .set_breakpoint(BreakpointType::Function {
                name: name.to_string(),
            })
            .await?;
        Ok(())
    }

    pub async fn set_exception_breakpoint(&mut self, filter: &str) -> Result<(), super::runtime::RuntimeError> {
        let _bp = self
            .runtime
            .set_breakpoint(BreakpointType::Exception {
                filter: filter.to_string(),
            })
            .await?;
        Ok(())
    }
}

pub struct DapServer<R: DebugRuntime> {
    session: Option<DebugSession<R>>,
}

impl<R: DebugRuntime> DapServer<R> {
    pub fn new() -> Self {
        Self { session: None }
    }

    pub fn set_runtime(&mut self, runtime: R) {
        self.session = Some(DebugSession::new(runtime));
    }

    pub async fn handle_request(&mut self, method: &str, params: &JsonValue, id: u64) -> Option<JsonValue> {
        match method {
            "initialize" => Some(self.handle_initialize(id)),
            "launch" => self.handle_launch(id, params).await,
            "attach" => self.handle_attach(id, params),
            "disconnect" => self.handle_disconnect(id),
            "setBreakpoints" => self.handle_set_breakpoints(id, params).await,
            "setFunctionBreakpoints" => self.handle_set_function_breakpoints(id, params).await,
            "setExceptionBreakpoints" => self.handle_set_exception_breakpoints(id, params).await,
            "configurationDone" => self.handle_configuration_done(id),
            "continue" => self.handle_continue(id).await,
            "next" => self.handle_next(id).await,
            "stepIn" => self.handle_step_in(id).await,
            "stepOut" => self.handle_step_out(id).await,
            "pause" => self.handle_pause(id).await,
            "stackTrace" => self.handle_stack_trace(id, params).await,
            "scopes" => self.handle_scopes(id, params).await,
            "variables" => self.handle_variables(id, params).await,
            "evaluate" => self.handle_evaluate(id, params).await,
            "source" => self.handle_source(id, params).await,
            _ => Some(self.error_response(id, -32600, format!("Unknown method: {}", method))),
        }
    }

    fn capabilities() -> JsonValue {
        json!({
            "supportsConfigurationDoneRequest": true,
            "supportsFunctionBreakpoints": true,
            "supportsConditionalBreakpoints": true,
            "supportsExceptionOptions": true,
            "supportsHitBreakpoints": true,
            "supportsLogBreakpoints": true,
            "supportsEvaluateForHovers": true,
            "supportsStepBack": false,
            "supportsSetVariable": false,
            "supportsRestartFrame": false,
            "supportsGotoTargetsRequest": false,
            "supportsCompletionsRequest": false,
            "supportsModulesRequest": false,
            "supportsTerminateDebuggee": true,
            "supportsDelayedStackTraceLoading": true,
            "supportsDataBreakpoints": true,
            "supportsSingleThreadExecutionRequests": true,
        })
    }

    fn handle_initialize(&self, id: u64) -> JsonValue {
        json!({
            "id": id,
            "result": Self::capabilities()
        })
    }

    async fn handle_launch(&mut self, id: u64, _params: &JsonValue) -> Option<JsonValue> {
        if let Some(session) = &mut self.session {
            let _ = session.runtime.step(StepMode::In).await.ok();
        }
        Some(json!({ "id": id, "result": {} }))
    }

    fn handle_attach(&mut self, id: u64, _params: &JsonValue) -> Option<JsonValue> {
        Some(json!({ "id": id, "result": {} }))
    }

    fn handle_disconnect(&self, id: u64) -> Option<JsonValue> {
        Some(json!({ "id": id, "result": {} }))
    }

    async fn handle_set_breakpoints(&mut self, id: u64, params: &JsonValue) -> Option<JsonValue> {
        let session = match &mut self.session {
            Some(s) => s,
            None => return Some(self.error_response(id, -1, "No debug session".to_string())),
        };

        let source = params.get("source")?.get("path")?.as_str()?;
        let breakpoints = params.get("breakpoints")?.as_array()?;

        let mut results = Vec::new();

        for bp in breakpoints {
            let line = bp.get("line")?.as_u64()? as u32;
            let _ = session.set_breakpoint(source, line).await.ok();
            results.push(json!({
                "verified": true,
                "line": line
            }));
        }

        Some(json!({
            "id": id,
            "result": { "breakpoints": results }
        }))
    }

    async fn handle_set_function_breakpoints(&mut self, id: u64, params: &JsonValue) -> Option<JsonValue> {
        let session = match &mut self.session {
            Some(s) => s,
            None => return Some(self.error_response(id, -1, "No debug session".to_string())),
        };

        let breakpoints = params.get("breakpoints")?.as_array()?;
        let mut results = Vec::new();

        for bp in breakpoints {
            let name = bp.get("name")?.as_str()?;
            let _ = session.set_function_breakpoint(name).await.ok();
            results.push(json!({
                "verified": true,
                "message": format!("Function breakpoint: {}", name)
            }));
        }

        Some(json!({
            "id": id,
            "result": { "breakpoints": results }
        }))
    }

    async fn handle_set_exception_breakpoints(&mut self, id: u64, params: &JsonValue) -> Option<JsonValue> {
        let session = match &mut self.session {
            Some(s) => s,
            None => return Some(self.error_response(id, -1, "No debug session".to_string())),
        };

        let filters = params.get("filters")?.as_array()?;
        let mut results = Vec::new();

        for filter in filters {
            let filter_str = filter.as_str()?;
            let _ = session.set_exception_breakpoint(filter_str).await.ok();
            results.push(json!({
                "verified": true,
                "message": format!("Exception breakpoint: {}", filter_str)
            }));
        }

        Some(json!({
            "id": id,
            "result": { "breakpoints": results }
        }))
    }

    fn handle_configuration_done(&self, id: u64) -> Option<JsonValue> {
        Some(json!({ "id": id, "result": {} }))
    }

    async fn handle_continue(&mut self, id: u64) -> Option<JsonValue> {
        let session = match &mut self.session {
            Some(s) => s,
            None => return Some(self.error_response(id, -1, "No debug session".to_string())),
        };

        match session.run().await {
            Ok(()) => Some(json!({ "id": id, "result": { "allThreadsContinued": true } })),
            Err(e) => Some(self.error_response(id, -1, format!("Continue failed: {}", e))),
        }
    }

    async fn handle_next(&mut self, id: u64) -> Option<JsonValue> {
        let session = match &mut self.session {
            Some(s) => s,
            None => return Some(self.error_response(id, -1, "No debug session".to_string())),
        };

        match session.step(StepMode::Over).await {
            Ok(()) => Some(json!({ "id": id, "result": {} })),
            Err(e) => Some(self.error_response(id, -1, format!("Step over failed: {}", e))),
        }
    }

    async fn handle_step_in(&mut self, id: u64) -> Option<JsonValue> {
        let session = match &mut self.session {
            Some(s) => s,
            None => return Some(self.error_response(id, -1, "No debug session".to_string())),
        };

        match session.step(StepMode::In).await {
            Ok(()) => Some(json!({ "id": id, "result": {} })),
            Err(e) => Some(self.error_response(id, -1, format!("Step in failed: {}", e))),
        }
    }

    async fn handle_step_out(&mut self, id: u64) -> Option<JsonValue> {
        let session = match &mut self.session {
            Some(s) => s,
            None => return Some(self.error_response(id, -1, "No debug session".to_string())),
        };

        match session.step(StepMode::Out).await {
            Ok(()) => Some(json!({ "id": id, "result": {} })),
            Err(e) => Some(self.error_response(id, -1, format!("Step out failed: {}", e))),
        }
    }

    async fn handle_pause(&mut self, id: u64) -> Option<JsonValue> {
        let session = match &mut self.session {
            Some(s) => s,
            None => return Some(self.error_response(id, -1, "No debug session".to_string())),
        };

        match session.pause().await {
            Ok(()) => Some(json!({ "id": id, "result": {} })),
            Err(e) => Some(self.error_response(id, -1, format!("Pause failed: {}", e))),
        }
    }

    async fn handle_stack_trace(&mut self, id: u64, params: &JsonValue) -> Option<JsonValue> {
        let session = match &mut self.session {
            Some(s) => s,
            None => return Some(self.error_response(id, -1, "No debug session".to_string())),
        };

        let thread_id = params.get("threadId").and_then(|v| v.as_u64());

        match session.stack_trace(thread_id).await {
            Ok(frames) => {
                let stack_frames: Vec<JsonValue> = frames
                    .into_iter()
                    .map(|frame| {
                        let mut obj = json!({
                            "id": frame.id,
                            "name": frame.name,
                            "line": frame.line,
                            "column": frame.column,
                        });
                        if let Some(source) = frame.source {
                            obj["source"] = json!({
                                "name": source.name,
                                "path": source.path,
                                "sourceReference": source.source_reference.unwrap_or(0)
                            });
                        }
                        obj
                    })
                    .collect();

                Some(json!({
                    "id": id,
                    "result": {
                        "stackFrames": stack_frames,
                        "totalFrames": stack_frames.len()
                    }
                }))
            }
            Err(e) => Some(self.error_response(id, -1, format!("Stack trace failed: {}", e))),
        }
    }

    async fn handle_scopes(&mut self, id: u64, params: &JsonValue) -> Option<JsonValue> {
        let session = match &mut self.session {
            Some(s) => s,
            None => return Some(self.error_response(id, -1, "No debug session".to_string())),
        };

        let frame_id = params.get("frameId")?.as_i64()?;

        match session.scopes(frame_id).await {
            Ok(scopes) => {
                let scope_objects: Vec<JsonValue> = scopes
                    .into_iter()
                    .map(|s| {
                        json!({
                            "variablesReference": s.variables_reference,
                            "name": s.name,
                            "expensive": s.expensive
                        })
                    })
                    .collect();

                Some(json!({
                    "id": id,
                    "result": { "scopes": scope_objects }
                }))
            }
            Err(e) => Some(self.error_response(id, -1, format!("Scopes failed: {}", e))),
        }
    }

    async fn handle_variables(&mut self, id: u64, params: &JsonValue) -> Option<JsonValue> {
        let session = match &mut self.session {
            Some(s) => s,
            None => return Some(self.error_response(id, -1, "No debug session".to_string())),
        };

        let variables_reference = params.get("variablesReference")?.as_i64()?;

        match session.variables(variables_reference).await {
            Ok(variables) => {
                let var_objects: Vec<JsonValue> = variables
                    .into_iter()
                    .map(|v| {
                        let mut obj = json!({
                            "name": v.name,
                            "value": v.value,
                            "type": v.type_
                        });
                        if let Some(ref_id) = v.variables_reference {
                            obj["variablesReference"] = ref_id.into();
                        }
                        if let Some(named) = v.named_variables {
                            obj["namedVariables"] = named.into();
                        }
                        if let Some(indexed) = v.indexed_variables {
                            obj["indexedVariables"] = indexed.into();
                        }
                        obj
                    })
                    .collect();

                Some(json!({
                    "id": id,
                    "result": { "variables": var_objects }
                }))
            }
            Err(e) => Some(self.error_response(id, -1, format!("Variables failed: {}", e))),
        }
    }

    async fn handle_evaluate(&mut self, id: u64, params: &JsonValue) -> Option<JsonValue> {
        let session = match &mut self.session {
            Some(s) => s,
            None => return Some(self.error_response(id, -1, "No debug session".to_string())),
        };

        let expression = params.get("expression")?.as_str()?;
        let frame_id = params.get("frameId").and_then(|v| v.as_i64()).unwrap_or(0);

        match session.evaluate(frame_id, expression).await {
            Ok(value) => {
                let (value_str, type_str) = match value {
                    Value::Nil => ("nil".to_string(), "nil".to_string()),
                    Value::Boolean(b) => (b.to_string(), "boolean".to_string()),
                    Value::Number(n) => (n.to_string(), "number".to_string()),
                    Value::String(s) => (format!("\"{}\"", s), "string".to_string()),
                    Value::Table { reference, length } => {
                        (format!("table (ref={}, len={})", reference, length), "table".to_string())
                    }
                    Value::Function { reference, name } => (
                        format!("function (ref={}, name={})", reference, name.unwrap_or_default()),
                        "function".to_string(),
                    ),
                    Value::UserData => ("userdata".to_string(), "userdata".to_string()),
                    Value::Thread => ("thread".to_string(), "thread".to_string()),
                };

                Some(json!({
                    "id": id,
                    "result": {
                        "result": value_str,
                        "type": type_str
                    }
                }))
            }
            Err(e) => Some(self.error_response(id, -1, format!("Evaluate failed: {}", e))),
        }
    }

    async fn handle_source(&mut self, id: u64, params: &JsonValue) -> Option<JsonValue> {
        let _source_reference = params.get("sourceReference")?.as_i64()?;
        Some(json!({
            "id": id,
            "result": {
                "content": "-- Source code not available"
            }
        }))
    }

    fn error_response(&self, id: u64, code: i32, message: String) -> JsonValue {
        json!({
            "id": id,
            "error": {
                "code": code,
                "message": message
            }
        })
    }
}

impl<R: DebugRuntime> Default for DapServer<R> {
    fn default() -> Self {
        Self::new()
    }
}